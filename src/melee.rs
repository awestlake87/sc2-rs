
use std::collections::HashMap;

use organelle;
use organelle::{
    Organelle, Sheath, Soma, Neuron, Handle, Impulse, ResultExt, Dendrite
};
use url::Url;
use uuid::Uuid;

use super::{
    Result,

    Signal,
    Synapse,
    Axon,
    ClientSignal,

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
    fn sheath(suite: MeleeSuite) -> Result<Sheath<Self>> {
        Ok(
            Sheath::new(
                MeleeSoma::Init(Init { suite: suite }),
                vec![ ],
                vec![
                    Dendrite::RequireOne(Synapse::Launcher),

                    Dendrite::Variadic(Synapse::Controller),
                    Dendrite::Variadic(Synapse::InstanceProvider),
                ]
            )?
        )
    }

    /// create the melee organelle
    pub fn organelle<L1, L2>(settings: MeleeSettings<L1, L2>)
        -> Result<Organelle<Sheath<Self>>>
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
        let mut organelle = Organelle::new(MeleeSoma::sheath(settings.suite)?);

        let launcher = organelle.add_soma(
            LauncherSoma::sheath(settings.launcher)?
        );

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

impl Neuron for MeleeSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, axon: &Axon, msg: Impulse<Self::Signal, Self::Synapse>)
        -> organelle::Result<Self>
    {
        match self {
            MeleeSoma::Init(state) => state.update(axon, msg),
            MeleeSoma::Setup(state) => state.update(axon, msg),
            MeleeSoma::Launch(state) => state.update(axon, msg),
            MeleeSoma::PlayerVsPlayer(state) => state.update(axon, msg),
            MeleeSoma::PlayerVsComputer(state) => state.update(axon, msg),
            MeleeSoma::Completed(state) => state.update(axon, msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init {
    suite:              MeleeSuite,
}

impl Init {
    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Start => Setup::setup(axon, self.suite),

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message"),
        }
    }
}

pub struct Setup {
    suite:              Option<MeleeSuite>,

    agents:             (Handle, Handle),
    clients:            (Handle, Handle),

    game:               GameSettings,
    players:            (Option<PlayerSetup>, Option<PlayerSetup>),
}

impl Setup {
    fn setup(axon: &Axon, suite: MeleeSuite) -> Result<MeleeSoma> {
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
                    suite: suite,

                    agents: (agents[0], agents[1]),
                    clients: (clients[0], clients[1]),

                    game: game,
                    players: (None, None),
                }
            )
        )
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Signal(src, Signal::PlayerSetup(setup)) => {
                self.on_player_setup(axon, src, setup)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_player_setup(mut self, axon: &Axon, src: Handle, setup: PlayerSetup)
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
                    axon,
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
        axon: &Axon,
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
    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Signal(src, Signal::InstancePool(instances)) => {
                self.on_instance_pool(axon, src, instances)
            },
            Impulse::Signal(src, Signal::PortsPool(ports)) => {
                self.on_ports_pool(axon, src, ports)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_instance_pool(
        mut self,
        axon: &Axon,
        src: Handle,
        instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<MeleeSoma>
    {
        assert_eq!(src, self.launcher);

        self.instances = instances;

        self.launch_instances(axon)?;
        self.try_provide_instances(axon)
    }

    fn on_ports_pool(mut self, axon: &Axon, src: Handle, ports: Vec<GamePorts>)
        -> Result<MeleeSoma>
    {
        assert_eq!(src, self.launcher);

        self.ports = ports;

        self.launch_instances(axon)?;
        self.try_provide_instances(axon)
    }

    fn launch_instances(&mut self, axon: &Axon) -> Result<()> {
        if self.is_pvp {
            if self.instances.len() < 2 && self.instances_requested < 2 {
                // launch as many instances as needed
                while self.instances_requested < 2 {
                    axon.send_req_output(
                        Synapse::Launcher, Signal::LaunchInstance
                    )?;

                    self.instances_requested += 1;
                }
            }
        }
        else {
            if self.instances.len() < 1 && self.instances_requested == 0 {
                axon.send_req_output(
                    Synapse::Launcher, Signal::LaunchInstance
                )?;
                self.instances_requested = 1;
            }
        }

        Ok(())
    }

    fn try_provide_instances(self, axon: &Axon) -> Result<MeleeSoma> {
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

                axon.effector()?.send(
                    self.clients.0,
                    Signal::Client(
                        ClientSignal::ProvideInstance(*id1, url1.clone())
                    )
                );
                axon.effector()?.send(
                    self.clients.1,
                    Signal::Client(
                        ClientSignal::ProvideInstance(*id2, url2.clone())
                    )
                );

                PlayerVsPlayer::start(
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

            axon.effector()?.send(
                player,
                Signal::Client(ClientSignal::ProvideInstance(*id, url.clone()))
            );

            PlayerVsComputer::start(
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
    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Signal(src, Signal::Ready) => {
                self.on_agent_ready(axon, src)
            },
            Impulse::Signal(src, Signal::GameCreated) => {
                self.on_game_created(axon, src)
            },
            Impulse::Signal(src, Signal::GameEnded) => {
                self.on_game_ended(axon, src)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_agent_ready(mut self, axon: &Axon, src: Handle)
        -> Result<MeleeSoma>
    {
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
            axon.effector()?.send(
                self.agents.0,
                Signal::CreateGame(
                    self.game.clone(), vec![ self.players.0, self.players.1 ]
                )
            );
        }

        Ok(MeleeSoma::PlayerVsPlayer(self))
    }

    fn on_game_created(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
        assert_eq!(src, self.agents.0);

        axon.effector()?.send(
            self.agents.0,
            Signal::GameReady(self.players.0, Some(self.ports.clone()))
        );
        axon.effector()?.send(
            self.agents.1,
            Signal::GameReady(self.players.1, Some(self.ports.clone()))
        );

        Ok(MeleeSoma::PlayerVsPlayer(self))
    }

    fn on_game_ended(mut self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
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
                Completed::complete(axon)
            }
            else {
                Setup::setup(axon, self.suite.unwrap())
            }
        }
        else {
            Ok(MeleeSoma::PlayerVsPlayer(self))
        }
    }
}

/// MeleeSoma state that pits players against the built-in AI
pub struct PlayerVsComputer {
    suite:              Option<MeleeSuite>,

    game:               GameSettings,
    player_setup:       PlayerSetup,
    computer_setup:     PlayerSetup,

    player:             Handle,
}

impl PlayerVsComputer {
    fn start(
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
                    suite: suite,

                    game: game,
                    player_setup: player.1,
                    computer_setup: computer.1,

                    player: player.0,
                }
            )
        )
    }
    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Signal(src, Signal::Ready) => {
                self.on_agent_ready(axon, src)
            },
            Impulse::Signal(src, Signal::GameCreated) => {
                self.on_game_created(axon, src)
            },
            Impulse::Signal(src, Signal::GameEnded) => {
                self.on_game_ended(axon, src)
            },

            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }

    fn on_agent_ready(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of Ready to be the agent")
        }

        axon.effector()?.send(
            self.player,
            Signal::CreateGame(
                self.game.clone(),
                vec![ self.player_setup, self.computer_setup ]
            )
        );

        Ok(MeleeSoma::PlayerVsComputer(self))
    }

    fn on_game_created(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of GameCreated to be the agent")
        }

        axon.effector()?.send(
            self.player,
            Signal::GameReady(self.player_setup, None)
        );

        Ok(MeleeSoma::PlayerVsComputer(self))
    }

    fn on_game_ended(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
        if src != self.player {
            bail!("expected source of GameEnded to be an agent")
        }

        if self.suite.is_none() {
            Completed::complete(axon)
        }
        else {
            Setup::setup(axon, self.suite.unwrap())
        }
    }
}

pub struct Completed;

impl Completed {
    fn complete(axon: &Axon) -> Result<MeleeSoma> {
        axon.effector()?.stop();

        Ok(
            MeleeSoma::Completed(Completed { })
        )
    }

    fn update(self, _axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<MeleeSoma>
    {
        match msg {
            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message"),
        }
    }
}
