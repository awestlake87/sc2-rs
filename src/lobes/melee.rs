
use std::collections::HashMap;

use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt, Constraint };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use data::{ GameSettings, GamePorts, PortSet, PlayerSetup };
use lobes::{ Message, Role, Cortex, Soma };
use lobes::agent::{ AgentLobe };
use lobes::launcher::{ LauncherLobe, LauncherSettings };

/// suite of games to choose from when pitting bots against each other
pub enum MeleeSuite {
    /// play one game with the given settings
    OneAndDone(GameSettings),
}

/// settings for the melee lobe
pub struct MeleeSettings<L1: Lobe + 'static, L2: Lobe + 'static> {
    /// the settings for the launcher lobe
    pub launcher:   LauncherSettings,
    /// the player cortices
    pub players:    (L1, L2),
    /// the suite of games to choose from
    pub suite:      MeleeSuite,
}

/// lobe designed to pit two bots against each other in Sc2 games
pub enum MeleeLobe {
    /// wait for soma to gather effector, inputs, and outputs
    Init(MeleeInit),

    /// fetch player info in order to decide how many instances it needs
    Setup(MeleeSetup),
    /// gather instances and game ports, then transition to PVP or PVC
    Launch(MeleeLaunch),

    /// coordinate two instances for player vs player
    PlayerVsPlayer(MeleePlayerVsPlayer),
    /// coordinate one instance for player vs the built-in Sc2 AI
    PlayerVsComputer(MeleePlayerVsComputer),
}

impl MeleeLobe {
    /// melee lobe only works as a controller in a melee cortex
    fn new(suite: MeleeSuite) -> Result<Self> {
        Ok(
            MeleeLobe::Init(
                MeleeInit {
                    soma: Soma::new(
                        vec![ ],
                        vec![
                            Constraint::RequireOne(Role::Launcher),

                            Constraint::Variadic(Role::Controller),
                            Constraint::Variadic(Role::InstanceProvider),
                        ]
                    )?,

                    suite: suite,
                }
            )
        )
    }

    /// create the melee cortex
    pub fn cortex<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Cortex>
        where
            L1: Lobe,
            L2: Lobe,

            Message: From<L1::Message> + From<L2::Message>,
            Role: From<L1::Role> + From<L2::Role>,

            L1::Message: From<Message>,
            L2::Message: From<Message>,

            L1::Role: From<Role>,
            L2::Role: From<Role>,
    {
        let mut cortex = Cortex::new(MeleeLobe::new(settings.suite)?);

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let melee = cortex.get_main_handle();

        let agent1 = cortex.add_lobe(AgentLobe::cortex(settings.players.0)?);
        let agent2 = cortex.add_lobe(AgentLobe::cortex(settings.players.1)?);

        cortex.connect(melee, launcher, Role::Launcher);

        cortex.connect(melee, agent1, Role::Controller);
        cortex.connect(melee, agent2, Role::Controller);
        cortex.connect(melee, agent1, Role::InstanceProvider);
        cortex.connect(melee, agent2, Role::InstanceProvider);

        Ok(cortex)
    }
}

pub struct MeleeInit {
    soma:               Soma,
    suite:              MeleeSuite,
}

impl MeleeInit {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Start => self.start(),

            _ => Ok(MeleeLobe::Init(self)),
        }
    }

    fn start(self) -> Result<MeleeLobe> {
        let clients = self.soma.var_output(Role::InstanceProvider)?.clone();
        let agents = self.soma.var_output(Role::Controller)?.clone();

        if clients.len() != 2 {
            bail!("expected 2 clients, got {}", clients.len())
        }

        if agents.len() != 2 {
            bail!("expected 2 agents, got {}", agents.len())
        }

        let game = match self.suite {
            MeleeSuite::OneAndDone(game) => game,
        };

        self.soma.effector()?.send(
            agents[0], Message::RequestPlayerSetup(game.clone())
        );
        self.soma.effector()?.send(
            agents[1], Message::RequestPlayerSetup(game.clone())
        );

        Ok(
            MeleeLobe::Setup(
                MeleeSetup {
                    soma: self.soma,

                    agents: (agents[0], agents[1]),
                    clients: (clients[0], clients[1]),

                    game: game,
                    players: (None, None),
                }
            )
        )
    }
}

pub struct MeleeSetup {
    soma:               Soma,

    agents:             (Handle, Handle),
    clients:            (Handle, Handle),

    game:               GameSettings,
    players:            (Option<PlayerSetup>, Option<PlayerSetup>),
}

impl MeleeSetup {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::PlayerSetup(setup)) => {
                self.on_player_setup(src, setup)
            },

            _ => Ok(MeleeLobe::Setup(self))
        }
    }

    fn on_player_setup(mut self, src: Handle, setup: PlayerSetup)
        -> Result<MeleeLobe>
    {
        if src == self.agents.0 {
            self.players.0 = Some(setup);
        }
        else if src == self.agents.1 {
            self.players.1 = Some(setup);
        }
        else {
            bail!("invalid source for player setup")
        }

        match self.players {
            (Some(setup1), Some(setup2)) => {
                let is_pvp = match setup1 {
                    PlayerSetup::Player { .. } => match setup2 {
                        PlayerSetup::Player { .. } => true,
                        PlayerSetup::Computer { .. } => false,
                        _ => bail!(
                            "melee lobe cannot do player vs observer"
                        )
                    },
                    PlayerSetup::Computer { .. } => match setup2 {
                        PlayerSetup::Player { .. } => false,
                        PlayerSetup::Computer { .. } => bail!(
                            "melee lobe cannot do computer vs computer"
                        ),
                        _ => bail!(
                            "melee lobe cannot do computer vs observer"
                        )
                    },
                    _ => bail!("melee lobe cannot use observer")
                };

                let launcher = self.soma.req_output(Role::Launcher)?;

                self.soma.effector()?.send(launcher, Message::LaunchInstance);

                if is_pvp {
                    self.soma.effector()?.send(
                        launcher, Message::LaunchInstance
                    );
                }

                Ok(
                    MeleeLobe::Launch(
                        MeleeLaunch {
                            soma: self.soma,
                            launcher: launcher,

                            agents: self.agents,
                            clients: self.clients,

                            game: self.game,
                            players: (setup1, setup2),
                            instances: HashMap::new(),
                            ports: vec![ ],

                            is_pvp: is_pvp,
                        }
                    )
                )
            },

            _ => Ok(MeleeLobe::Setup(self))
        }
    }
}

pub struct MeleeLaunch {
    soma:               Soma,
    launcher:           Handle,

    agents:             (Handle, Handle),
    clients:            (Handle, Handle),

    game:               GameSettings,
    players:            (PlayerSetup, PlayerSetup),
    instances:          HashMap<Uuid, (Url, PortSet)>,
    ports:              Vec<GamePorts>,

    is_pvp:             bool,
}

impl MeleeLaunch {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::InstancePool(instances)) => {
                self.on_instance_pool(src, instances)
            },
            Protocol::Message(src, Message::PortsPool(ports)) => {
                self.on_ports_pool(src, ports)
            },

            _ => Ok(MeleeLobe::Launch(self))
        }
    }

    fn on_instance_pool(
        mut self, src: Handle, instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<MeleeLobe>
    {
        assert_eq!(src, self.launcher);

        self.instances = instances;

        self.try_provide_instances()
    }

    fn on_ports_pool(mut self, src: Handle, ports: Vec<GamePorts>)
        -> Result<MeleeLobe>
    {
        assert_eq!(src, self.launcher);

        self.ports = ports;

        self.try_provide_instances()
    }

    fn try_provide_instances(self) -> Result<MeleeLobe> {
        if self.is_pvp {
            if self.instances.len() >= 2 && self.ports.len() >= 1 {
                let (id1, &(ref url1, ref ports1)) = self.instances.iter()
                    .nth(0).unwrap()
                ;
                let (id2, &(ref url2, ref ports2)) = self.instances.iter()
                    .nth(1).unwrap()
                ;
                let mut ports = self.ports[0].clone();

                ports.client_ports = vec![ *ports1, *ports2 ];

                self.soma.effector()?.send(
                    self.clients.0,
                    Message::ProvideInstance(*id1, url1.clone())
                );
                self.soma.effector()?.send(
                    self.clients.1,
                    Message::ProvideInstance(*id2, url2.clone())
                );

                Ok(
                    MeleeLobe::PlayerVsPlayer(
                        MeleePlayerVsPlayer {
                            soma: self.soma,

                            agents: self.agents,

                            game: self.game,
                            ports: ports,
                            players: self.players,

                            ready: (false, false),
                        }
                    )
                )
            }
            else {
                Ok(MeleeLobe::Launch(self))
            }
        }
        else {
            if self.instances.len() >= 1 {
                Ok(
                    MeleeLobe::PlayerVsComputer(
                        MeleePlayerVsComputer { }
                    )
                )
            }
            else {
                Ok(MeleeLobe::Launch(self))
            }
        }
    }
}

pub struct MeleePlayerVsPlayer {
    soma: Soma,

    agents: (Handle, Handle),

    game: GameSettings,
    ports: GamePorts,
    players: (PlayerSetup, PlayerSetup),

    ready: (bool, bool)
}

impl MeleePlayerVsPlayer {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<MeleeLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::Ready) => {
                self.on_agent_ready(src)
            },
            Protocol::Message(src, Message::GameCreated) => {
                self.on_game_created(src)
            },

            _ => Ok(MeleeLobe::PlayerVsPlayer(self))
        }
    }

    fn on_agent_ready(mut self, src: Handle) -> Result<MeleeLobe> {
        if src == self.agents.0 {
            self.ready.0 = true;
        }
        else if src == self.agents.1 {
            self.ready.1 = true;
        }
        else {
            bail!("expected source of Ready to be an agent")
        }

        if self.ready == (true, true) {
            self.soma.effector()?.send(
                self.agents.0,
                Message::CreateGame(
                    self.game.clone(), vec![ self.players.0, self.players.1 ]
                )
            );
        }

        Ok(MeleeLobe::PlayerVsPlayer(self))
    }

    fn on_game_created(self, src: Handle) -> Result<MeleeLobe> {
        assert_eq!(src, self.agents.0);

        self.soma.effector()?.send(
            self.agents.0,
            Message::GameReady(self.players.0, self.ports.clone())
        );
        self.soma.effector()?.send(
            self.agents.1,
            Message::GameReady(self.players.1, self.ports.clone())
        );

        Ok(MeleeLobe::PlayerVsPlayer(self))
    }
}

/// MeleeLobe state that pits players against the built-in AI
pub struct MeleePlayerVsComputer {
}

impl MeleePlayerVsComputer {
    fn update(self, _msg: Protocol<Message, Role>) -> Result<MeleeLobe> {
        unimplemented!()
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        match self {
            MeleeLobe::Init(state) => state.update(msg),
            MeleeLobe::Setup(state) => state.update(msg),
            MeleeLobe::Launch(state) => state.update(msg),
            MeleeLobe::PlayerVsPlayer(state) => state.update(msg),
            MeleeLobe::PlayerVsComputer(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
