
use std::collections::HashMap;

use organelle;
use organelle::{ Soma, Handle, Impulse, ResultExt, Dendrite };
use url::Url;
use uuid::Uuid;

use super::{
    Result,

    Signal,
    Synapse,
    Organelle,
    Axon,

    GameSettings,
    GamePorts,
    PortSet,
    PlayerSetup,
};

use launcher::{ LauncherSoma, LauncherSettings };

/// suite of games to choose from when pitting bots against each other
pub enum MeleeSuite {
    /// play one game with the given settings
    OneAndDone(GameSettings),
    /// repeat this game indefinitely
    EndlessRepeat(GameSettings),
}

/// settings for the melee soma
pub struct MeleeSettings<L1: Soma + 'static, L2: Soma + 'static> {
    /// the settings for the launcher soma
    pub launcher:   LauncherSettings,
    /// the player organelles
    pub players:    (L1, L2),
    /// the suite of games to choose from
    pub suite:      MeleeSuite,
}

/// soma designed to pit two bots against each other in Sc2 games
pub enum MeleeSoma {
    /// wait for axon to gather effector, inputs, and outputs
    Init(Init),

    /// fetch player info in order to decide how many instances it needs
    Setup(Setup),
    /// gather instances and game ports, then transition to PVP or PVC
    Launch(Launch),

    /// coordinate two instances for player vs player
    PlayerVsPlayer(PlayerVsPlayer),
    /// coordinate one instance for player vs the built-in Sc2 AI
    PlayerVsComputer(PlayerVsComputer),

    /// melee suite is exhausted and organelle is awaiting shutdown
    Completed(Completed),
}

impl MeleeSoma {
    /// melee soma only works as a controller in a melee organelle
    fn new(suite: MeleeSuite) -> Result<Self> {
        Ok(
            MeleeSoma::Init(
                Init {
                    axon: Axon::new(
                        vec![ ],
                        vec![
                            Dendrite::RequireOne(Synapse::Launcher),

                            Dendrite::Variadic(Synapse::Controller),
                            Dendrite::Variadic(Synapse::InstanceProvider),
                        ]
                    )?,

                    suite: suite,
                }
            )
        )
    }

    /// create the melee organelle
    pub fn organelle<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Organelle>
        where
            L1: Soma,
            L2: Soma,

            Signal: From<L1::Signal> + From<L2::Signal>,
            Synapse: From<L1::Synapse> + From<L2::Synapse>,

            L1::Signal: From<Signal>,
            L2::Signal: From<Signal>,

            L1::Synapse: From<Synapse>,
            L2::Synapse: From<Synapse>,
    {
        let mut organelle = Organelle::new(MeleeSoma::new(settings.suite)?);

        let launcher = organelle.add_soma(LauncherSoma::from(settings.launcher)?);

        let melee = organelle.get_main_handle();

        let player1 = organelle.add_soma(settings.players.0);
        let player2 = organelle.add_soma(settings.players.1);

        organelle.connect(melee, launcher, Synapse::Launcher);

        organelle.connect(melee, player1, Synapse::Controller);
        organelle.connect(melee, player2, Synapse::Controller);
        organelle.connect(melee, player1, Synapse::InstanceProvider);
        organelle.connect(melee, player2, Synapse::InstanceProvider);

        Ok(organelle)
    }
}

impl Soma for MeleeSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, msg: Impulse<Self::Signal, Self::Synapse>)
        -> organelle::Result<Self>
    {
        match self {
            MeleeSoma::Init(state) => state.update(msg),
            MeleeSoma::Setup(state) => state.update(msg),
            MeleeSoma::Launch(state) => state.update(msg),
            MeleeSoma::PlayerVsPlayer(state) => state.update(msg),
            MeleeSoma::PlayerVsComputer(state) => state.update(msg),
            MeleeSoma::Completed(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init {
    axon:               Axon,
    suite:              MeleeSuite,
}

impl Init {
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Start => Setup::setup(self.axon, self.suite),


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(MeleeSoma::Init(self))
        }
    }
}

pub struct Setup {
    axon:               Axon,
    suite:              Option<MeleeSuite>,

    agents:             (Handle, Handle),
    clients:            (Handle, Handle),

    game:               GameSettings,
    players:            (Option<PlayerSetup>, Option<PlayerSetup>),
}

impl Setup {
    fn setup(axon: Axon, suite: MeleeSuite) -> Result<MeleeSoma> {
        let clients = axon.var_output(Synapse::InstanceProvider)?.clone();
        let agents = axon.var_output(Synapse::Controller)?.clone();

        if clients.len() != 2 {
            bail!("expected 2 clients, got {}", clients.len())
        }

        if agents.len() != 2 {
            bail!("expected 2 agents, got {}", agents.len())
        }

        let (game, suite) = match suite {
            MeleeSuite::OneAndDone(game) => (game, None),
            MeleeSuite::EndlessRepeat(game) => {
                let suite = Some(MeleeSuite::EndlessRepeat(game.clone()));

                (game, suite)
            }
        };

        axon.effector()?.send(
            agents[0], Signal::RequestPlayerSetup(game.clone())
        );
        axon.effector()?.send(
            agents[1], Signal::RequestPlayerSetup(game.clone())
        );

        Ok(
            MeleeSoma::Setup(
                Setup {
                    axon: axon,
                    suite: suite,

                    agents: (agents[0], agents[1]),
                    clients: (clients[0], clients[1]),

                    game: game,
                    players: (None, None),
                }
            )
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::PlayerSetup(setup)) => {
                    self.on_player_setup(src, setup)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeSoma::Setup(self))
        }
    }

    fn on_player_setup(mut self, src: Handle, setup: PlayerSetup)
        -> Result<MeleeSoma>
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
                Launch::launch(
                    self.axon,
                    self.suite,
                    self.agents,
                    self.clients,
                    (setup1, setup2),
                    self.game
                )
            },

            _ => Ok(MeleeSoma::Setup(self))
        }
    }
}

pub struct Launch {
    axon:                   Axon,
    suite:                  Option<MeleeSuite>,
    launcher:               Handle,

    agents:                 (Handle, Handle),
    clients:                (Handle, Handle),

    game:                   GameSettings,
    players:                (PlayerSetup, PlayerSetup),
    instances:              HashMap<Uuid, (Url, PortSet)>,
    ports:                  Vec<GamePorts>,

    is_pvp:                 bool,
    instances_requested:    u32
}

impl Launch {
    fn launch(
        axon: Axon,
        suite: Option<MeleeSuite>,
        agents: (Handle, Handle),
        clients: (Handle, Handle),
        players: (PlayerSetup, PlayerSetup),
        game: GameSettings,
    )
        -> Result<MeleeSoma>
    {
        let is_pvp = {
            if players.0.is_player() && players.1.is_computer() {
                false
            }
            else if players.0.is_computer() && players.1.is_player() {
                false
            }
            else if players.0.is_player() && players.1.is_player() {
                true
            }
            else {
                bail!("invalid player setups")
            }
        };

        let launcher = axon.req_output(Synapse::Launcher)?;
        axon.effector()?.send(launcher, Signal::GetInstancePool);
        axon.effector()?.send(launcher, Signal::GetPortsPool);

        Ok(
            MeleeSoma::Launch(
                Launch {
                    axon: axon,
                    suite: suite,
                    launcher: launcher,

                    agents: agents,
                    clients: clients,

                    game: game,
                    players: players,
                    instances: HashMap::new(),
                    ports: vec![ ],

                    is_pvp: is_pvp,
                    instances_requested: 0,
                }
            )
        )
    }
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::InstancePool(instances)) => {
                    self.on_instance_pool(src, instances)
                },
                Impulse::Signal(src, Signal::PortsPool(ports)) => {
                    self.on_ports_pool(src, ports)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeSoma::Launch(self))
        }
    }

    fn on_instance_pool(
        mut self, src: Handle, instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<MeleeSoma>
    {
        assert_eq!(src, self.launcher);

        self.instances = instances;

        self.launch_instances()?;
        self.try_provide_instances()
    }

    fn on_ports_pool(mut self, src: Handle, ports: Vec<GamePorts>)
        -> Result<MeleeSoma>
    {
        assert_eq!(src, self.launcher);

        self.ports = ports;

        self.launch_instances()?;
        self.try_provide_instances()
    }

    fn launch_instances(&mut self) -> Result<()> {
        if self.is_pvp {
            if self.instances.len() < 2 && self.instances_requested < 2 {
                // launch as many instances as needed
                while self.instances_requested < 2 {
                    self.axon.send_req_output(
                        Synapse::Launcher, Signal::LaunchInstance
                    )?;

                    self.instances_requested += 1;
                }
            }
        }
        else {
            if self.instances.len() < 1 && self.instances_requested == 0 {
                self.axon.send_req_output(
                    Synapse::Launcher, Signal::LaunchInstance
                )?;
                self.instances_requested = 1;
            }
        }

        Ok(())
    }

    fn try_provide_instances(self) -> Result<MeleeSoma> {
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

                self.axon.effector()?.send(
                    self.clients.0,
                    Signal::ProvideInstance(*id1, url1.clone())
                );
                self.axon.effector()?.send(
                    self.clients.1,
                    Signal::ProvideInstance(*id2, url2.clone())
                );

                PlayerVsPlayer::start(
                    self.axon,
                    self.suite,
                    self.agents,
                    self.players,
                    self.game,
                    ports
                )
            }
            else {
                Ok(MeleeSoma::Launch(self))
            }
        }
        else if self.instances.len() >= 1 {
            let (id, &(ref url, _)) = self.instances.iter()
                .nth(0).unwrap()
            ;

            let ((player, player_setup), (computer, computer_setup)) = {
                if self.players.0.is_player() {
                    (
                        (self.agents.0, self.players.0),
                        (self.agents.1, self.players.1),
                    )
                }
                else {
                    assert!(self.players.1.is_player());

                    (
                        (self.agents.1, self.players.1),
                        (self.agents.0, self.players.0),
                    )
                }
            };

            self.axon.effector()?.send(
                player, Signal::ProvideInstance(*id, url.clone())
            );

            PlayerVsComputer::start(
                self.axon,
                self.suite,
                (player, player_setup),
                (computer, computer_setup),
                self.game,
            )
        }
        else {
            Ok(MeleeSoma::Launch(self))
        }
    }
}

pub struct PlayerVsPlayer {
    axon: Axon,
    suite: Option<MeleeSuite>,

    agents: (Handle, Handle),

    game: GameSettings,
    ports: GamePorts,
    players: (PlayerSetup, PlayerSetup),

    ready: (bool, bool),
    ended: (bool, bool),
}

impl PlayerVsPlayer {
    fn start(
        axon: Axon,
        suite: Option<MeleeSuite>,
        agents: (Handle, Handle),
        players: (PlayerSetup, PlayerSetup),
        game: GameSettings,
        ports: GamePorts
    )
        -> Result<MeleeSoma>
    {
        Ok(
            MeleeSoma::PlayerVsPlayer(
                PlayerVsPlayer {
                    axon: axon,
                    suite: suite,

                    agents: agents,

                    game: game,
                    ports: ports,
                    players: players,

                    ready: (false, false),
                    ended: (false, false),
                }
            )
        )
    }
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::Ready) => {
                    self.on_agent_ready(src)
                },
                Impulse::Signal(src, Signal::GameCreated) => {
                    self.on_game_created(src)
                },
                Impulse::Signal(src, Signal::GameEnded) => {
                    self.on_game_ended(src)
                },


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeSoma::PlayerVsPlayer(self))
        }
    }

    fn on_agent_ready(mut self, src: Handle) -> Result<MeleeSoma> {
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
            self.axon.effector()?.send(
                self.agents.0,
                Signal::CreateGame(
                    self.game.clone(), vec![ self.players.0, self.players.1 ]
                )
            );
        }

        Ok(MeleeSoma::PlayerVsPlayer(self))
    }

    fn on_game_created(self, src: Handle) -> Result<MeleeSoma> {
        assert_eq!(src, self.agents.0);

        self.axon.effector()?.send(
            self.agents.0,
            Signal::GameReady(self.players.0, Some(self.ports.clone()))
        );
        self.axon.effector()?.send(
            self.agents.1,
            Signal::GameReady(self.players.1, Some(self.ports.clone()))
        );

        Ok(MeleeSoma::PlayerVsPlayer(self))
    }

    fn on_game_ended(mut self, src: Handle) -> Result<MeleeSoma> {
        if src == self.agents.0 {
            self.ended.0 = true;
        }
        else if src == self.agents.1 {
            self.ended.1 = true;
        }
        else {
            bail!("expected src of GameEnded to be an agent")
        }

        if self.ended == (true, true) {
            if self.suite.is_none() {
                Completed::complete(self.axon)
            }
            else {
                Setup::setup(self.axon, self.suite.unwrap())
            }
        }
        else {
            Ok(MeleeSoma::PlayerVsPlayer(self))
        }
    }
}

/// MeleeSoma state that pits players against the built-in AI
pub struct PlayerVsComputer {
    axon:               Axon,
    suite:              Option<MeleeSuite>,

    game:               GameSettings,
    player_setup:       PlayerSetup,
    computer_setup:     PlayerSetup,

    player:             Handle,
}

impl PlayerVsComputer {
    fn start(
        axon: Axon,
        suite: Option<MeleeSuite>,
        player: (Handle, PlayerSetup),
        computer: (Handle, PlayerSetup),
        game: GameSettings,
    )
        -> Result<MeleeSoma>
    {
        Ok(
            MeleeSoma::PlayerVsComputer(
                PlayerVsComputer {
                    axon: axon,
                    suite: suite,

                    game: game,
                    player_setup: player.1,
                    computer_setup: computer.1,

                    player: player.0,
                }
            )
        )
    }
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(src, Signal::Ready) => {
                    self.on_agent_ready(src)
                },
                Impulse::Signal(src, Signal::GameCreated) => {
                    self.on_game_created(src)
                },
                Impulse::Signal(src, Signal::GameEnded) => {
                    self.on_game_ended(src)
                }


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(MeleeSoma::PlayerVsComputer(self))
        }
    }

    fn on_agent_ready(self, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of Ready to be the agent")
        }

        self.axon.effector()?.send(
            self.player,
            Signal::CreateGame(
                self.game.clone(),
                vec![ self.player_setup, self.computer_setup ]
            )
        );

        Ok(MeleeSoma::PlayerVsComputer(self))
    }

    fn on_game_created(self, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of GameCreated to be the agent")
        }

        self.axon.effector()?.send(
            self.player,
            Signal::GameReady(self.player_setup, None)
        );

        Ok(MeleeSoma::PlayerVsComputer(self))
    }

    fn on_game_ended(self, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of GameEnded to be an agent")
        }

        if self.suite.is_none() {
            Completed::complete(self.axon)
        }
        else {
            Setup::setup(self.axon, self.suite.unwrap())
        }
    }
}

pub struct Completed {
    axon:               Axon,
}

impl Completed {
    fn complete(axon: Axon) -> Result<MeleeSoma> {
        axon.effector()?.stop();

        Ok(
            MeleeSoma::Completed(Completed { axon: axon })
        )
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<MeleeSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        }
        else {
            Ok(MeleeSoma::Completed(self))
        }
    }
}
