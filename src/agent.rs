
use std::rc::Rc;
use std::time;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };
use sc2_proto::{ sc2api, common, debug };
use url::Url;

use super::{
    Result,
    IntoProto,

    Message,
    Role,
    Soma,
    Cortex,

    FrameData,
    Command,
    DebugCommand,
    DebugTextTarget,

    GameSettings,
    GamePorts,
    PlayerSetup,
    Map,
    ActionTarget,
};
use client::{ ClientLobe, Transactor, ClientRequest, ClientResponse };
use observer::{ ObserverLobe };

pub enum AgentLobe {
    Init(Init),
    Setup(Setup),

    CreateGame(CreateGame),
    GameCreated(GameCreated),
    JoinGame(JoinGame),

    FetchGameData(FetchGameData),
    StepperSetup(StepperSetup),

    Update(Update),
    SendActions(SendActions),
    SendDebug(SendDebug),
    Step(Step),
    Observe(Observe),

    LeaveGame(LeaveGame),
    Reset(Reset),
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
                            Constraint::RequireOne(Role::Observer),
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
        let observer = cortex.add_lobe::<ObserverLobe>(ObserverLobe::new()?);

        cortex.connect(agent, client, Role::InstanceProvider);
        cortex.connect(agent, client, Role::Client);
        cortex.connect(observer, client, Role::Client);

        cortex.connect(agent, observer, Role::Observer);
        cortex.connect(agent, player, Role::Agent);

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

            AgentLobe::Update(state) => state.update(msg),
            AgentLobe::SendActions(state) => state.update(msg),
            AgentLobe::SendDebug(state) => state.update(msg),
            AgentLobe::Step(state) => state.update(msg),
            AgentLobe::Observe(state) => state.update(msg),

            AgentLobe::LeaveGame(state) => state.update(msg),
            AgentLobe::Reset(state) => state.update(msg),
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
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma),

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::Init(self))
        }
    }
}

pub struct Setup {
    soma:           Soma,
}

impl Setup {
    fn setup(soma: Soma) -> Result<AgentLobe> {
        Ok(AgentLobe::Setup(Setup { soma: soma, }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::Ready) => {
                    self.on_ready(src)
                },

                Protocol::Message(
                    src, Message::RequestPlayerSetup(settings)
                ) => {
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
                Protocol::Message(
                    src, Message::CreateGame(settings, players)
                ) => {
                    self.create_game(src, settings, players)
                },
                Protocol::Message(_, Message::GameReady(setup, ports)) => {
                    self.on_game_ready(setup, ports)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::Setup(self))
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

    fn on_game_ready(self, setup: PlayerSetup, ports: Option<GamePorts>)
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
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    self.transactor.expect(src, rsp)?;

                    GameCreated::game_created(self.soma)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::CreateGame(self))
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
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::GameReady(setup, ports)) => {
                    JoinGame::join_game(self.soma, setup, ports)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::GameCreated(self))
        }
    }
}

pub struct JoinGame {
    soma:           Soma,
    transactor:     Transactor,
}

impl JoinGame {
    fn join_game(soma: Soma, setup: PlayerSetup, ports: Option<GamePorts>)
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

        if let Some(ports) = ports {
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
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    self.on_join_game(src, rsp)
                }

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::JoinGame(self))
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
}

impl FetchGameData {
    fn fetch(soma: Soma) -> Result<AgentLobe> {
        soma.send_req_output(Role::Observer, Message::FetchGameData)?;

        Ok(AgentLobe::FetchGameData(FetchGameData { soma: soma }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::GameDataReady) => {
                    StepperSetup::setup(self.soma)
                },
                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::FetchGameData(self))
        }
    }
}

pub struct StepperSetup {
    soma:           Soma,
    stepper:        Handle,
}

impl StepperSetup {
    fn setup(soma: Soma) -> Result<AgentLobe> {
        let stepper = soma.req_output(Role::Agent)?;

        soma.effector()?.send(stepper, Message::RequestUpdateInterval);

        Ok(
            AgentLobe::StepperSetup(
                StepperSetup {
                    soma: soma,
                    stepper: stepper,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::UpdateInterval(interval)) => {
                    self.on_update_interval(src, interval)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::StepperSetup(self))
        }
    }

    fn on_update_interval(self, src: Handle, interval: u32)
        -> Result<AgentLobe>
    {
        if src == self.stepper {
            Step::first(self.soma, interval)
        }
        else {
            bail!("unexpected source of update interval: {}", src)
        }
    }
}

pub struct Update {
    soma:               Soma,
    interval:           u32,
    commands:           Vec<Command>,
    debug_commands:     Vec<DebugCommand>,
}

impl Update {
    fn next(soma: Soma, interval: u32, frame: Rc<FrameData>)
        -> Result<AgentLobe>
    {
        soma.send_req_output(
            Role::Agent, Message::Observation(frame)
        )?;

        Ok(
            AgentLobe::Update(
                Update {
                    soma: soma,
                    interval: interval,
                    commands: vec![ ],
                    debug_commands: vec![ ],
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::Command(cmd)) => {
                    self.commands.push(cmd);
                    Ok(AgentLobe::Update(self))
                },
                Protocol::Message(_, Message::DebugCommand(cmd)) => {
                    self.debug_commands.push(cmd);
                    Ok(AgentLobe::Update(self))
                },

                Protocol::Message(_, Message::UpdateComplete) => {
                    self.on_update_complete()
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::Update(self))
        }
    }

    fn on_update_complete(self) -> Result<AgentLobe> {
        SendActions::send_actions(
            self.soma,
            self.interval,
            self.commands,
            self.debug_commands
        )
    }
}

pub struct SendActions {
    soma:               Soma,
    interval:           u32,
    transactor:         Transactor,

    debug_commands:     Vec<DebugCommand>,
}

impl SendActions {
    fn send_actions(
        soma: Soma,
        interval: u32,
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

                    debug_commands: debug_commands,
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    let rsp = self.transactor.expect(src, rsp)?;

                    SendDebug::send_debug(
                        self.soma,
                        self.interval,
                        self.debug_commands
                    )
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::SendActions(self))
        }
    }
}

pub struct SendDebug {
    soma:           Soma,
    interval:       u32,
    transactor:     Transactor,
}

impl SendDebug {
    fn send_debug(
        soma: Soma,
        interval: u32,
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
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    let rsp = self.transactor.expect(src, rsp)?;

                    Step::step(self.soma, self.interval)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::SendDebug(self))
        }
    }
}

pub struct Step {
    soma:           Soma,
    interval:       u32,
    transactor:     Transactor,
}

impl Step {
    fn first(soma: Soma, interval: u32) -> Result<AgentLobe> {
        soma.send_req_output(Role::Agent, Message::GameStarted)?;

        Step::step(
            soma,
            interval,
        )
    }
    fn step(soma: Soma, interval: u32)
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
                }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    self.on_step(src, rsp)
                },


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::Step(self))
        }
    }

    fn on_step(self, src: Handle, rsp: ClientResponse) -> Result<AgentLobe> {
        self.transactor.expect(src, rsp)?;

        Observe::observe(self.soma, self.interval)
    }
}

pub struct Observe {
    soma:           Soma,
    interval:       u32,
}

impl Observe {
    fn observe(soma: Soma, interval: u32) -> Result<AgentLobe> {
        soma.send_req_output(Role::Observer, Message::Observe)?;

        Ok(AgentLobe::Observe(Observe { soma: soma, interval: interval }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::Observation(frame)) => {
                    Update::next(self.soma, self.interval, frame)
                },
                Protocol::Message(_, Message::GameEnded) => {
                    LeaveGame::leave(self.soma)
                }

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentLobe::Observe(self))
        }
    }
}

pub struct LeaveGame {
    soma:           Soma,
    transactor:     Transactor,
}

impl LeaveGame {
    fn leave(soma: Soma) -> Result<AgentLobe> {
        let mut req = sc2api::Request::new();

        req.mut_leave_game();

        let transactor = Transactor::send(&soma, ClientRequest::new(req))?;

        Ok(
            AgentLobe::LeaveGame(
                LeaveGame { soma: soma, transactor: transactor }
            )
        )
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(src, Message::ClientResponse(rsp)) => {
                    self.transactor.expect(src, rsp)?;

                    Reset::reset(self.soma)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentLobe::LeaveGame(self))
        }
    }
}

pub struct Reset {
    soma:           Soma,
}

impl Reset {
    fn reset(soma: Soma) -> Result<AgentLobe> {
        soma.send_req_output(Role::Client, Message::ClientDisconnect)?;

        Ok(AgentLobe::Reset(Reset { soma: soma, }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<AgentLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::ClientError(_)) => {
                    // client does not close cleanly anyway right now, so just
                    // ignore the error and wait for ClientClosed.
                    Ok(AgentLobe::Reset(self))
                }
                Protocol::Message(_, Message::ClientClosed) => {
                    self.soma.send_req_input(
                        Role::Controller, Message::GameEnded
                    )?;
                    self.soma.send_req_output(
                        Role::Agent, Message::GameEnded
                    )?;

                    Setup::setup(self.soma)
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => {
                    bail!("unexpected protocol message")
                }
            }
        }
        else {
            Ok(AgentLobe::Reset(self))
        }
    }
}
