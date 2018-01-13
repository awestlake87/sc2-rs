
use std::rc::Rc;
use std::time;

use organelle;
use organelle::{ ResultExt, Handle, Soma, Impulse, Dendrite };
use sc2_proto::{ sc2api, debug };
use url::Url;

use super::{
    Result,
    IntoProto,

    Signal,
    Synapse,
    Axon,
    Organelle,
    Sheath,

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
use client::{ ClientSoma, Transactor, ClientRequest, ClientResult };
use observer::{ ObserverSoma };

/// mediates interactions between the player and the game instance
pub enum AgentSoma {
    /// initialize the axon
    Init(Init),
    /// perform setup queries
    Setup(Setup),

    /// order the game instance to create a game
    CreateGame(CreateGame),
    /// game has been created
    GameCreated(GameCreated),
    /// join an existing game
    JoinGame(JoinGame),

    /// order the observer to fetch game data
    FetchGameData(FetchGameData),
    /// query the request interval from the player soma
    StepperSetup(StepperSetup),

    /// broadcast game updates to the player soma
    Update(Update),
    /// send any actions for this step to the game instance
    SendActions(SendActions),
    /// send any debug actions for this step to the game instance
    SendDebug(SendDebug),
    /// step the game instance
    Step(Step),
    /// order the observer to observe the game state
    Observe(Observe),

    /// leave the current game
    LeaveGame(LeaveGame),
    /// reset the client and re-enter setup state
    Reset(Reset),
}

impl AgentSoma {
    fn new() -> Result<Self> {
        Ok(
            AgentSoma::Init(
                Init {
                    axon: Axon::new(
                        vec![
                            Dendrite::RequireOne(Synapse::Controller),
                            Dendrite::RequireOne(Synapse::InstanceProvider),
                        ],
                        vec![
                            Dendrite::RequireOne(Synapse::Client),
                            Dendrite::RequireOne(Synapse::Agent),
                            Dendrite::RequireOne(Synapse::InstanceProvider),
                            Dendrite::RequireOne(Synapse::Observer),
                        ],
                    )?,
                }
            )
        )
    }

    /// compose an agent organelle to interact with a controller soma
    pub fn organelle<L>(soma: L) -> Result<Organelle> where
        L: Soma + 'static,

        L::Signal: From<Signal>,
        L::Synapse: From<Synapse>,

        Signal: From<L::Signal>,
        Synapse: From<L::Synapse>,
    {
        let mut organelle = Organelle::new(AgentSoma::new()?);

        let agent = organelle.get_main_handle();
        let player = organelle.add_soma(soma);

        // TODO: find out why these explicit annotation is needed. it's
        // possible that it's a bug in the rust type system because it will
        // work when the function is generic across two soma types, but not one
        let client = organelle.add_soma::<Sheath<ClientSoma>>(
            ClientSoma::new()?
        );
        let observer = organelle.add_soma::<ObserverSoma>(ObserverSoma::new()?);

        organelle.connect(agent, client, Synapse::InstanceProvider);
        organelle.connect(agent, client, Synapse::Client);
        organelle.connect(observer, client, Synapse::Client);

        organelle.connect(agent, observer, Synapse::Observer);
        organelle.connect(agent, player, Synapse::Agent);

        Ok(organelle)
    }
}

impl Soma for AgentSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, msg: Impulse<Signal, Synapse>)
        -> organelle::Result<Self>
    {
        match self {
            AgentSoma::Init(state) => state.update(msg),
            AgentSoma::Setup(state) => state.update(msg),

            AgentSoma::CreateGame(state) => state.update(msg),
            AgentSoma::GameCreated(state) => state.update(msg),
            AgentSoma::JoinGame(state) => state.update(msg),

            AgentSoma::StepperSetup(state) => state.update(msg),
            AgentSoma::FetchGameData(state) => state.update(msg),

            AgentSoma::Update(state) => state.update(msg),
            AgentSoma::SendActions(state) => state.update(msg),
            AgentSoma::SendDebug(state) => state.update(msg),
            AgentSoma::Step(state) => state.update(msg),
            AgentSoma::Observe(state) => state.update(msg),

            AgentSoma::LeaveGame(state) => state.update(msg),
            AgentSoma::Reset(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init {
    axon:           Axon,
}

impl Init {
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Start => Setup::setup(self.axon),

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::Init(self))
        }
    }
}

pub struct Setup {
    axon:           Axon,
}

impl Setup {
    fn setup(axon: Axon) -> Result<AgentSoma> {
        Ok(AgentSoma::Setup(Setup { axon: axon, }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::Ready) => {
                    self.on_ready(src)
                },

                Impulse::Signal(
                    src, Signal::RequestPlayerSetup(settings)
                ) => {
                    self.on_req_player_setup(src, settings)
                },
                Impulse::Signal(src, Signal::PlayerSetup(setup)) => {
                    self.on_player_setup(src, setup)
                },

                Impulse::Signal(
                    src, Signal::ProvideInstance(instance, url)
                ) => {
                    self.provide_instance(src, instance, url)
                }
                Impulse::Signal(
                    src, Signal::CreateGame(settings, players)
                ) => {
                    self.create_game(src, settings, players)
                },
                Impulse::Signal(_, Signal::GameReady(setup, ports)) => {
                    self.on_game_ready(setup, ports)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::Setup(self))
        }
    }

    fn on_ready(self, src: Handle) -> Result<AgentSoma> {
        assert_eq!(src, self.axon.req_output(Synapse::Client)?);

        self.axon.send_req_input(Synapse::Controller, Signal::Ready)?;

        Ok(AgentSoma::Setup(self))
    }

    fn on_req_player_setup(self, src: Handle, settings: GameSettings)
        -> Result<AgentSoma>
    {
        assert_eq!(src, self.axon.req_input(Synapse::Controller)?);

        self.axon.send_req_output(
            Synapse::Agent, Signal::RequestPlayerSetup(settings)
        )?;

        Ok(AgentSoma::Setup(self))
    }

    fn on_player_setup(self, src: Handle, setup: PlayerSetup)
        -> Result<AgentSoma>
    {
        assert_eq!(src, self.axon.req_output(Synapse::Agent)?);

        self.axon.send_req_input(
            Synapse::Controller, Signal::PlayerSetup(setup)
        )?;

        Ok(AgentSoma::Setup(self))
    }

    fn provide_instance(self, src: Handle, instance: Handle, url: Url)
        -> Result<AgentSoma>
    {
        assert_eq!(src, self.axon.req_input(Synapse::InstanceProvider)?);

        self.axon.send_req_output(
            Synapse::InstanceProvider, Signal::ProvideInstance(instance, url)
        )?;

        Ok(AgentSoma::Setup(self))
    }

    fn create_game(
        self,
        src: Handle,
        settings: GameSettings,
        players: Vec<PlayerSetup>
    )
        -> Result<AgentSoma>
    {
        assert_eq!(src, self.axon.req_input(Synapse::Controller)?);

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
                /*PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }*/
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(false);

        let transactor = Transactor::send(
            &self.axon, ClientRequest::new(req)
        )?;

        Ok(
            AgentSoma::CreateGame(
                CreateGame {
                    axon: self.axon,
                    transactor: transactor,
                }
            )
        )
    }

    fn on_game_ready(self, setup: PlayerSetup, ports: Option<GamePorts>)
        -> Result<AgentSoma>
    {
        let this_soma = self.axon.effector()?.this_soma();

        self.axon.effector()?.send(
            this_soma, Signal::GameReady(setup, ports)
        );

        Ok(AgentSoma::GameCreated(GameCreated { axon: self.axon }))
    }
}

pub struct CreateGame {
    axon:           Axon,
    transactor:     Transactor,
}

impl CreateGame {
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.transactor.expect(src, result)?;

                    GameCreated::game_created(self.axon)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::CreateGame(self))
        }
    }
}

pub struct GameCreated {
    axon:           Axon,
}

impl GameCreated {
    fn game_created(axon: Axon) -> Result<AgentSoma> {
        axon.send_req_input(
            Synapse::Controller, Signal::GameCreated
        )?;

        Ok(
            AgentSoma::GameCreated(
                GameCreated {
                    axon: axon,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::GameReady(setup, ports)) => {
                    JoinGame::join_game(self.axon, setup, ports)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::GameCreated(self))
        }
    }
}

pub struct JoinGame {
    axon:           Axon,
    transactor:     Transactor,
}

impl JoinGame {
    fn join_game(axon: Axon, setup: PlayerSetup, ports: Option<GamePorts>)
        -> Result<AgentSoma>
    {
        let mut req = sc2api::Request::new();

        match setup {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            //_ => req.mut_join_game().set_race(common::Race::NoRace)
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
            &axon,
            ClientRequest::with_timeout(req, time::Duration::from_secs(60))
        )?;

        Ok(
            AgentSoma::JoinGame(
                JoinGame {
                    axon: axon,
                    transactor: transactor,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.on_join_game(src, result)
                }

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::JoinGame(self))
        }
    }

    fn on_join_game(self, src: Handle, result: ClientResult)
        -> Result<AgentSoma>
    {
        self.transactor.expect(src, result)?;

        FetchGameData::fetch(self.axon)
    }
}

pub struct FetchGameData {
    axon:           Axon,
}

impl FetchGameData {
    fn fetch(axon: Axon) -> Result<AgentSoma> {
        axon.send_req_output(Synapse::Observer, Signal::FetchGameData)?;

        Ok(AgentSoma::FetchGameData(FetchGameData { axon: axon }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::GameDataReady) => {
                    StepperSetup::setup(self.axon)
                },
                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::FetchGameData(self))
        }
    }
}

pub struct StepperSetup {
    axon:           Axon,
    stepper:        Handle,
}

impl StepperSetup {
    fn setup(axon: Axon) -> Result<AgentSoma> {
        let stepper = axon.req_output(Synapse::Agent)?;

        axon.effector()?.send(stepper, Signal::RequestUpdateInterval);

        Ok(
            AgentSoma::StepperSetup(
                StepperSetup {
                    axon: axon,
                    stepper: stepper,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::UpdateInterval(interval)) => {
                    self.on_update_interval(src, interval)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::StepperSetup(self))
        }
    }

    fn on_update_interval(self, src: Handle, interval: u32)
        -> Result<AgentSoma>
    {
        if src == self.stepper {
            Step::first(self.axon, interval)
        }
        else {
            bail!("unexpected source of update interval: {}", src)
        }
    }
}

pub struct Update {
    axon:               Axon,
    interval:           u32,
    commands:           Vec<Command>,
    debug_commands:     Vec<DebugCommand>,
}

impl Update {
    fn next(axon: Axon, interval: u32, frame: Rc<FrameData>)
        -> Result<AgentSoma>
    {
        axon.send_req_output(
            Synapse::Agent, Signal::Observation(frame)
        )?;

        Ok(
            AgentSoma::Update(
                Update {
                    axon: axon,
                    interval: interval,
                    commands: vec![ ],
                    debug_commands: vec![ ],
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::Command(cmd)) => {
                    self.commands.push(cmd);
                    Ok(AgentSoma::Update(self))
                },
                Impulse::Signal(_, Signal::DebugCommand(cmd)) => {
                    self.debug_commands.push(cmd);
                    Ok(AgentSoma::Update(self))
                },

                Impulse::Signal(_, Signal::UpdateComplete) => {
                    self.on_update_complete()
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::Update(self))
        }
    }

    fn on_update_complete(self) -> Result<AgentSoma> {
        SendActions::send_actions(
            self.axon,
            self.interval,
            self.commands,
            self.debug_commands
        )
    }
}

pub struct SendActions {
    axon:               Axon,
    interval:           u32,
    transactor:         Transactor,

    debug_commands:     Vec<DebugCommand>,
}

impl SendActions {
    fn send_actions(
        axon: Axon,
        interval: u32,
        commands: Vec<Command>,
        debug_commands: Vec<DebugCommand>
    )
        -> Result<AgentSoma>
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

        let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

        Ok(
            AgentSoma::SendActions(
                SendActions {
                    axon: axon,
                    interval: interval,
                    transactor: transactor,

                    debug_commands: debug_commands,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.transactor.expect(src, result)?;

                    SendDebug::send_debug(
                        self.axon,
                        self.interval,
                        self.debug_commands
                    )
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::SendActions(self))
        }
    }
}

pub struct SendDebug {
    axon:           Axon,
    interval:       u32,
    transactor:     Transactor,
}

impl SendDebug {
    fn send_debug(
        axon: Axon,
        interval: u32,
        commands: Vec<DebugCommand>
    )
        -> Result<AgentSoma>
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

        let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

        Ok(
            AgentSoma::SendDebug(
                SendDebug {
                    axon: axon,
                    interval: interval,
                    transactor: transactor,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.transactor.expect(src, result)?;

                    Step::step(self.axon, self.interval)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::SendDebug(self))
        }
    }
}

pub struct Step {
    axon:           Axon,
    interval:       u32,
    transactor:     Transactor,
}

impl Step {
    fn first(axon: Axon, interval: u32) -> Result<AgentSoma> {
        axon.send_req_output(Synapse::Agent, Signal::GameStarted)?;

        Step::step(
            axon,
            interval,
        )
    }
    fn step(axon: Axon, interval: u32)
        -> Result<AgentSoma>
    {
        let mut req = sc2api::Request::new();

        req.mut_step().set_count(interval);

        let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

        Ok(
            AgentSoma::Step(
                Step {
                    axon: axon,
                    interval: interval,
                    transactor: transactor,
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.on_step(src, result)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::Step(self))
        }
    }

    fn on_step(self, src: Handle, result: ClientResult) -> Result<AgentSoma> {
        self.transactor.expect(src, result)?;

        Observe::observe(self.axon, self.interval)
    }
}

pub struct Observe {
    axon:           Axon,
    interval:       u32,
}

impl Observe {
    fn observe(axon: Axon, interval: u32) -> Result<AgentSoma> {
        axon.send_req_output(Synapse::Observer, Signal::Observe)?;

        Ok(AgentSoma::Observe(Observe { axon: axon, interval: interval }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::Observation(frame)) => {
                    Update::next(self.axon, self.interval, frame)
                },
                Impulse::Signal(_, Signal::GameEnded) => {
                    LeaveGame::leave(self.axon)
                }

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(AgentSoma::Observe(self))
        }
    }
}

pub struct LeaveGame {
    axon:           Axon,
    transactor:     Transactor,
}

impl LeaveGame {
    fn leave(axon: Axon) -> Result<AgentSoma> {
        let mut req = sc2api::Request::new();

        req.mut_leave_game();

        let transactor = Transactor::send(&axon, ClientRequest::new(req))?;

        Ok(
            AgentSoma::LeaveGame(
                LeaveGame { axon: axon, transactor: transactor }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::ClientResult(result)) => {
                    self.transactor.expect(src, result)?;

                    Reset::reset(self.axon)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(AgentSoma::LeaveGame(self))
        }
    }
}

pub struct Reset {
    axon:           Axon,
}

impl Reset {
    fn reset(axon: Axon) -> Result<AgentSoma> {
        axon.send_req_output(Synapse::Client, Signal::ClientDisconnect)?;

        Ok(AgentSoma::Reset(Reset { axon: axon, }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<AgentSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::ClientError(_)) => {
                    // client does not close cleanly anyway right now, so just
                    // ignore the error and wait for ClientClosed.
                    Ok(AgentSoma::Reset(self))
                }
                Impulse::Signal(_, Signal::ClientClosed) => {
                    self.axon.send_req_input(
                        Synapse::Controller, Signal::GameEnded
                    )?;
                    self.axon.send_req_output(
                        Synapse::Agent, Signal::GameEnded
                    )?;

                    Setup::setup(self.axon)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => {
                    bail!("unexpected protocol message")
                }
            }
        }
        else {
            Ok(AgentSoma::Reset(self))
        }
    }
}
