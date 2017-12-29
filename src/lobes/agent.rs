
use std::collections::HashMap;
use std::rc::Rc;
use std::time;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };
use sc2_proto::{ sc2api, common, debug };
use url::Url;

use super::super::{ Result, FromProto, IntoProto, IntoSc2 };
use lobes::{ Message, Role, Soma, Cortex };
use lobes::client::{ ClientLobe, Transactor, ClientRequest, ClientResponse };
use lobes::frame::{
    FrameData,
    Command,
    DebugCommand,
    DebugTextTarget,
    GameData,
};

use data::{
    GameSettings,
    GamePorts,
    PlayerSetup,
    Map,
    ActionTarget,
    Buff,
    BuffData,
    Upgrade,
    UpgradeData,
    Ability,
    AbilityData,
    UnitType,
    UnitTypeData,
};

pub enum AgentLobe {
    Init(Init),
    Setup(Setup),

    CreateGame(CreateGame),
    GameCreated(GameCreated),
    JoinGame(JoinGame),

    StepperSetup(StepperSetup),
    FetchGameData(FetchGameData),
    FetchTerrainData(FetchTerrainData),

    Update(Update),
    SendActions(SendActions),
    SendDebug(SendDebug),
    Step(Step),
}

impl AgentLobe {
    fn new() -> Result<Self> {
        Ok(
            AgentLobe::Init(
                Init {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::Controller),
                            Constraint::RequireOne(Role::InstanceProvider),
                        ],
                        vec![
                            Constraint::RequireOne(Role::Client),
                            Constraint::RequireOne(Role::Agent),
                            Constraint::RequireOne(Role::InstanceProvider),
                            Constraint::RequireOne(Role::Stepper),
                        ],
                    )?,
                }
            )
        )
    }

    pub fn cortex<L>(lobe: L) -> Result<Cortex> where
        L: Lobe + 'static,

        L::Message: From<Message>,
        L::Role: From<Role>,

        Message: From<L::Message>,
        Role: From<L::Role>,
    {
        let mut cortex = Cortex::new(AgentLobe::new()?);

        let agent = cortex.get_main_handle();
        let player = cortex.add_lobe(lobe);

        // TODO: find out why these explicit annotation is needed. it's
        // possible that it's a bug in the rust type system because it will
        // work when the function is generic across two lobe types, but not one
        let client = cortex.add_lobe::<ClientLobe>(ClientLobe::new()?);

        cortex.connect(agent, client, Role::InstanceProvider);
        cortex.connect(agent, client, Role::Client);
        cortex.connect(agent, player, Role::Agent);
        cortex.connect(agent, player, Role::Stepper);

        Ok(cortex)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        match self {
            AgentLobe::Init(state) => state.update(msg),
            AgentLobe::Setup(state) => state.update(msg),

            AgentLobe::CreateGame(state) => state.update(msg),
            AgentLobe::GameCreated(state) => state.update(msg),
            AgentLobe::JoinGame(state) => state.update(msg),

            AgentLobe::StepperSetup(state) => state.update(msg),
            AgentLobe::FetchGameData(state) => state.update(msg),
            AgentLobe::FetchTerrainData(state) => state.update(msg),

            AgentLobe::Update(state) => state.update(msg),
            AgentLobe::SendActions(state) => state.update(msg),
            AgentLobe::SendDebug(state) => state.update(msg),
            AgentLobe::Step(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct Init {
    soma:           Soma,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Start => Setup::setup(self.soma),

            _ => Ok(AgentLobe::Init(self)),
        }
    }
}

pub struct Setup {
    soma:           Soma,
}

impl Setup {
    fn setup(soma: Soma) -> Result<AgentLobe> {
        Ok(
            AgentLobe::Setup(
                Setup {
                    soma: soma,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::Ready) => {
                self.on_ready(src)
            },

            Protocol::Message(src, Message::RequestPlayerSetup(settings)) => {
                self.on_req_player_setup(src, settings)
            },
            Protocol::Message(src, Message::PlayerSetup(setup)) => {
                self.on_player_setup(src, setup)
            },

            Protocol::Message(
                src, Message::ProvideInstance(instance, url)
            ) => {
                self.provide_instance(src, instance, url)
            }
            Protocol::Message(src, Message::CreateGame(settings, players)) => {
                self.create_game(src, settings, players)
            },
            Protocol::Message(_, Message::GameReady(setup, ports)) => {
                self.on_game_ready(setup, ports)
            },

            _ => Ok(AgentLobe::Setup(self))
        }
    }

    fn on_ready(self, src: Handle) -> Result<AgentLobe> {
        assert_eq!(src, self.soma.req_output(Role::Client)?);

        self.soma.send_req_input(Role::Controller, Message::Ready)?;

        Ok(AgentLobe::Setup(self))
    }

    fn on_req_player_setup(self, src: Handle, settings: GameSettings)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        self.soma.send_req_output(
            Role::Agent, Message::RequestPlayerSetup(settings)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn on_player_setup(self, src: Handle, setup: PlayerSetup)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_output(Role::Agent)?);

        self.soma.send_req_input(
            Role::Controller, Message::PlayerSetup(setup)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn provide_instance(self, src: Handle, instance: Handle, url: Url)
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::InstanceProvider)?);

        self.soma.send_req_output(
            Role::InstanceProvider, Message::ProvideInstance(instance, url)
        )?;

        Ok(AgentLobe::Setup(self))
    }

    fn create_game(
        self,
        src: Handle,
        settings: GameSettings,
        players: Vec<PlayerSetup>
    )
        -> Result<AgentLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(ref path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.clone().into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => bail!("invalid path string")
                    }
                );
            },
            Map::BlizzardMap(ref map) => {
                req.mut_create_game().set_battlenet_map_name(map.clone());
            }
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                PlayerSetup::Computer { difficulty, race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Player { race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(false);

        let transactor = Transactor::send(
            &self.soma, ClientRequest::new(req)
        )?;

        Ok(
            AgentLobe::CreateGame(
                CreateGame {
                    soma: self.soma,
                    transactor: transactor,
                }
            )
        )
    }

    fn on_game_ready(self, setup: PlayerSetup, ports: GamePorts)
        -> Result<AgentLobe>
    {
        let this_lobe = self.soma.effector()?.this_lobe();

        self.soma.effector()?.send(
            this_lobe, Message::GameReady(setup, ports)
        );

        Ok(AgentLobe::GameCreated(GameCreated { soma: self.soma }))
    }
}

pub struct CreateGame {
    soma:           Soma,
    transactor:     Transactor,
}

impl CreateGame {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.transactor.expect(src, rsp)?;

                GameCreated::game_created(self.soma)
            },

            _ => Ok(AgentLobe::CreateGame(self))
        }
    }
}

pub struct GameCreated {
    soma:           Soma,
}

impl GameCreated {
    fn game_created(soma: Soma) -> Result<AgentLobe> {
        soma.send_req_input(
            Role::Controller, Message::GameCreated
        )?;

        Ok(
            AgentLobe::GameCreated(
                GameCreated {
                    soma: soma,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::GameReady(setup, ports)) => {
                JoinGame::join_game(self.soma, setup, ports)
            },

            _ => Ok(AgentLobe::GameCreated(self))
        }
    }
}

pub struct JoinGame {
    soma:           Soma,
    transactor:     Transactor,
}

impl JoinGame {
    fn join_game(soma: Soma, setup: PlayerSetup, ports: GamePorts)
        -> Result<AgentLobe>
    {
        let mut req = sc2api::Request::new();

        match setup {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            _ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        req.mut_join_game().set_shared_port(ports.shared_port as i32);

        {
            let s = req.mut_join_game().mut_server_ports();

            s.set_game_port(ports.server_ports.game_port as i32);
            s.set_base_port(ports.server_ports.base_port as i32);
        }

        {
            let client_ports = req.mut_join_game().mut_client_ports();

            for c in &ports.client_ports {
                let mut p = sc2api::PortSet::new();

                p.set_game_port(c.game_port as i32);
                p.set_base_port(c.base_port as i32);

                client_ports.push(p);
            }
        }

        {
            let options = req.mut_join_game().mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        let transactor = Transactor::send(
            &soma,
            ClientRequest::with_timeout(req, time::Duration::from_secs(60))
        )?;

        Ok(
            AgentLobe::JoinGame(
                JoinGame {
                    soma: soma,
                    transactor: transactor,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.on_join_game(src, rsp)
            }
            _ => Ok(AgentLobe::JoinGame(self))
        }
    }

    fn on_join_game(self, src: Handle, rsp: ClientResponse)
        -> Result<AgentLobe>
    {
        self.transactor.expect(src, rsp)?;

        FetchGameData::fetch(self.soma)
    }
}

pub struct FetchGameData {
    soma:           Soma,
    transactor:     Transactor,
}

impl FetchGameData {
    fn fetch(soma: Soma) -> Result<AgentLobe> {
        let mut req = sc2api::Request::new();
        req.mut_data().set_unit_type_id(true);

        let transactor = Transactor::send(
            &soma, ClientRequest::new(req)
        )?;

        Ok(
            AgentLobe::FetchGameData(
                FetchGameData {
                    soma: soma,
                    transactor: transactor
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.on_game_data(src, rsp)
            }
            _ => Ok(AgentLobe::FetchGameData(self))
        }
    }

    fn on_game_data(self, src: Handle, rsp: ClientResponse)
        -> Result<AgentLobe>
    {
        let mut rsp = self.transactor.expect(src, rsp)?;

        let mut unit_type_data = HashMap::new();
        let mut ability_data = HashMap::new();
        let mut upgrade_data = HashMap::new();
        let mut buff_data = HashMap::new();

        for data in rsp.response.mut_data().take_units().into_iter() {
            let u = UnitTypeData::from_proto(data)?;

            let unit_type = u.unit_type;
            unit_type_data.insert(unit_type, u);
        }

        for data in rsp.response.mut_data().take_abilities().into_iter() {
            let a = AbilityData::from_proto(data)?;

            let ability = a.ability;
            ability_data.insert(ability, a);
        }

        for data in rsp.response.mut_data().take_upgrades().into_iter() {
            let u = UpgradeData::from_proto(data)?;

            let upgrade = u.upgrade;
            upgrade_data.insert(upgrade, u);
        }

        for data in rsp.response.mut_data().take_buffs().into_iter() {
            let b = BuffData::from_proto(data)?;

            let buff = b.buff;
            buff_data.insert(buff, b);
        }

        FetchTerrainData::fetch(
            self.soma, unit_type_data, ability_data, upgrade_data, buff_data
        )
    }
}

pub struct FetchTerrainData {
    soma:                   Soma,
    transactor:             Transactor,

    unit_type_data:         HashMap<UnitType, UnitTypeData>,
    ability_data:           HashMap<Ability, AbilityData>,
    upgrade_data:           HashMap<Upgrade, UpgradeData>,
    buff_data:              HashMap<Buff, BuffData>,
}

impl FetchTerrainData {
    fn fetch(
        soma: Soma,
        unit_type_data: HashMap<UnitType, UnitTypeData>,
        ability_data: HashMap<Ability, AbilityData>,
        upgrade_data: HashMap<Upgrade, UpgradeData>,
        buff_data: HashMap<Buff, BuffData>
    )
        -> Result<AgentLobe>
    {
        let mut req = sc2api::Request::new();
        req.mut_game_info();

        let transactor = Transactor::send(&soma, ClientRequest::new(req))?;

        Ok(
            AgentLobe::FetchTerrainData(
                FetchTerrainData {
                    soma: soma,
                    transactor: transactor,

                    unit_type_data: unit_type_data,
                    ability_data: ability_data,
                    upgrade_data: upgrade_data,
                    buff_data: buff_data,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.on_terrain_info(src, rsp)
            },

            _ => Ok(AgentLobe::FetchTerrainData(self))
        }
    }

    fn on_terrain_info(self, src: Handle, rsp: ClientResponse)
        -> Result<AgentLobe>
    {
        let mut rsp = self.transactor.expect(src, rsp)?;

        let game_data = Rc::from(
            GameData {
                unit_type_data: self.unit_type_data,
                ability_data: self.ability_data,
                upgrade_data: self.upgrade_data,
                buff_data: self.buff_data,

                terrain_info: rsp.response.take_game_info().into_sc2()?
            }
        );

        StepperSetup::setup(self.soma, game_data)
    }
}

pub struct StepperSetup {
    soma:           Soma,
    stepper:        Handle,
    game_data:      Rc<GameData>,
}

impl StepperSetup {
    fn setup(soma: Soma, game_data: Rc<GameData>) -> Result<AgentLobe> {
        let stepper = soma.req_output(Role::Stepper)?;

        soma.effector()?.send(stepper, Message::RequestUpdateInterval);

        Ok(
            AgentLobe::StepperSetup(
                StepperSetup {
                    soma: soma,
                    stepper: stepper,
                    game_data: game_data,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::UpdateInterval(interval)) => {
                self.on_update_interval(src, interval)
            },

            _ => Ok(AgentLobe::StepperSetup(self))
        }
    }

    fn on_update_interval(self, src: Handle, interval: u32)
        -> Result<AgentLobe>
    {
        if src == self.stepper {
            Step::step(self.soma, interval, self.game_data)
        }
        else {
            bail!("unexpected source of update interval: {}", src)
        }
    }
}

pub struct Update {
    soma:           Soma,
    interval:       u32,
    game_data:      Rc<GameData>,
}

impl Update {
    fn next(soma: Soma, interval: u32, game_data: Rc<GameData>)
        -> Result<AgentLobe>
    {
        let this_lobe = soma.effector()?.this_lobe();
        soma.effector()?.send(this_lobe, Message::UpdateComplete);

        Ok(
            AgentLobe::Update(
                Update {
                    soma: soma,
                    interval: interval,
                    game_data: game_data,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(_, Message::UpdateComplete) => {
                SendActions::send_actions(
                    self.soma, self.interval, self.game_data, vec![ ], vec![ ]
                )
            },
            _ => Ok(AgentLobe::Update(self))
        }
    }
}

pub struct SendActions {
    soma:               Soma,
    interval:           u32,
    transactor:         Transactor,

    game_data:          Rc<GameData>,
    debug_commands:     Vec<DebugCommand>,
}

impl SendActions {
    fn send_actions(
        soma: Soma,
        interval: u32,
        game_data: Rc<GameData>,
        commands: Vec<Command>,
        debug_commands: Vec<DebugCommand>
    )
        -> Result<AgentLobe>
    {
        let mut req = sc2api::Request::new();
        req.mut_action().mut_actions();

        for cmd in commands {
            match cmd {
                Command::Action { units, ability, target } => {
                    let mut a = sc2api::Action::new();

                    {
                        let cmd = a.mut_action_raw().mut_unit_command();

                        cmd.set_ability_id(ability.into_proto()? as i32);

                        match target {
                            Some(ActionTarget::UnitTag(tag)) => {
                                cmd.set_target_unit_tag(tag);
                            }
                            Some(ActionTarget::Location(pos)) => {
                                let target = cmd.mut_target_world_space_pos();
                                target.set_x(pos.x);
                                target.set_y(pos.y);
                            },
                            None => ()
                        }

                        for u in units {
                            cmd.mut_unit_tags().push(u.tag);
                        }
                    }

                    req.mut_action().mut_actions().push(a);
                }
            }
        }

        let transactor = Transactor::send(&soma, ClientRequest::new(req))?;

        Ok(
            AgentLobe::SendActions(
                SendActions {
                    soma: soma,
                    interval: interval,
                    transactor: transactor,

                    game_data: game_data,
                    debug_commands: debug_commands,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                let rsp = self.transactor.expect(src, rsp)?;

                SendDebug::send_debug(
                    self.soma,
                    self.interval,
                    self.game_data,
                    self.debug_commands
                )
            }
            _ => Ok(AgentLobe::SendActions(self))
        }
    }
}

pub struct SendDebug {
    soma:           Soma,
    interval:       u32,
    transactor:     Transactor,

    game_data:      Rc<GameData>,
}

impl SendDebug {
    fn send_debug(
        soma: Soma,
        interval: u32,
        game_data: Rc<GameData>,
        commands: Vec<DebugCommand>
    )
        -> Result<AgentLobe>
    {
        let mut req = sc2api::Request::new();
        req.mut_debug().mut_debug();

        for cmd in commands {
            match cmd {
                DebugCommand::DebugText { text, target, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_text = debug::DebugText::new();

                    debug_text.set_text(text);

                    match target {
                        Some(DebugTextTarget::Screen(p)) => {
                            debug_text.mut_virtual_pos().set_x(p.x);
                            debug_text.mut_virtual_pos().set_y(p.y);
                        },
                        Some(DebugTextTarget::World(p)) => {
                            debug_text.mut_world_pos().set_x(p.x);
                            debug_text.mut_world_pos().set_y(p.y);
                            debug_text.mut_world_pos().set_z(p.z);
                        },
                        None => ()
                    }

                    debug_text.mut_color().set_r(color.0 as u32);
                    debug_text.mut_color().set_g(color.1 as u32);
                    debug_text.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_text().push(debug_text);
                    req.mut_debug().mut_debug().push(cmd);
                },
                DebugCommand::DebugLine { p1, p2, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_line = debug::DebugLine::new();

                    debug_line.mut_line().mut_p0().set_x(p1.x);
                    debug_line.mut_line().mut_p0().set_y(p1.y);
                    debug_line.mut_line().mut_p0().set_z(p1.z);

                    debug_line.mut_line().mut_p1().set_x(p2.x);
                    debug_line.mut_line().mut_p1().set_y(p2.y);
                    debug_line.mut_line().mut_p1().set_z(p2.z);

                    debug_line.mut_color().set_r(color.0 as u32);
                    debug_line.mut_color().set_g(color.1 as u32);
                    debug_line.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_lines().push(debug_line);
                    req.mut_debug().mut_debug().push(cmd);
                },
                DebugCommand::DebugBox { min, max, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_box = debug::DebugBox::new();

                    debug_box.mut_min().set_x(min.x);
                    debug_box.mut_min().set_y(min.y);
                    debug_box.mut_min().set_z(min.z);

                    debug_box.mut_max().set_x(max.x);
                    debug_box.mut_max().set_y(max.y);
                    debug_box.mut_max().set_z(max.z);

                    debug_box.mut_color().set_r(color.0 as u32);
                    debug_box.mut_color().set_g(color.1 as u32);
                    debug_box.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_boxes().push(debug_box);
                    req.mut_debug().mut_debug().push(cmd);
                }
                DebugCommand::DebugSphere { center, radius, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_sphere = debug::DebugSphere::new();

                    debug_sphere.mut_p().set_x(center.x);
                    debug_sphere.mut_p().set_y(center.y);
                    debug_sphere.mut_p().set_z(center.z);

                    debug_sphere.set_r(radius);

                    debug_sphere.mut_color().set_r(color.0 as u32);
                    debug_sphere.mut_color().set_g(color.1 as u32);
                    debug_sphere.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_spheres().push(debug_sphere);
                    req.mut_debug().mut_debug().push(cmd);
                }
            }
        }

        let transactor = Transactor::send(&soma, ClientRequest::new(req))?;

        Ok(
            AgentLobe::SendDebug(
                SendDebug {
                    soma: soma,
                    interval: interval,
                    transactor: transactor,

                    game_data: game_data,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                let rsp = self.transactor.expect(src, rsp)?;

                Step::step(self.soma, self.interval, self.game_data)
            }
            _ => Ok(AgentLobe::SendDebug(self))
        }
    }
}

pub struct Step {
    soma:           Soma,
    interval:       u32,
    transactor:     Transactor,

    game_data:      Rc<GameData>,
}

impl Step {
    fn step(soma: Soma, interval: u32, game_data: Rc<GameData>)
        -> Result<AgentLobe>
    {
        let mut req = sc2api::Request::new();

        req.mut_step().set_count(interval);

        let transactor = Transactor::send(&soma, ClientRequest::new(req))?;

        Ok(
            AgentLobe::Step(
                Step {
                    soma: soma,
                    interval: interval,
                    transactor: transactor,

                    game_data: game_data,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::ClientResponse(rsp)) => {
                self.transactor.expect(src, rsp)?;

                Update::next(self.soma, self.interval, self.game_data)
            },

            _ => Ok(AgentLobe::Step(self)),
        }
    }
}
