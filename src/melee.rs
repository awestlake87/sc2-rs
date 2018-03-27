use std::{self, mem};

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle;
use organelle::{Axon, Constraint, Impulse, Organelle, Soma};
use tokio_core::reactor;
use url::Url;

use super::{Error, Result};
use ctrlc_breaker::CtrlcBreakerSoma;
use data::{GameSetup, PlayerSetup};
use launcher::{GamePorts, Launcher, LauncherSettings};
use synapses::{Dendrite, Synapse, Terminal};

/// update scheme for the agents to use
#[derive(Debug, Copy, Clone)]
pub enum UpdateScheme {
    /// update as fast as possible
    Realtime,
    /// step the game with a fixed interval
    Interval(u32),
}

/// empty trait to prevent users from passing their own somas to MeleeBuilder
pub trait MeleeCompetitor {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        controller: MeleeDendrite,
    ) -> Result<()>;
}

/// build a Melee coordinator
pub struct MeleeBuilder {
    players: Vec<Box<MeleeCompetitor>>,

    launcher_settings: Option<LauncherSettings>,
    suite: Option<MeleeSuite>,
    update_scheme: UpdateScheme,
    break_on_ctrlc: bool,
    handle: Option<reactor::Handle>,
}

impl MeleeBuilder {
    /// start building a melee soma with the given agent or computer somas
    pub fn new() -> Self {
        Self {
            players: vec![],

            launcher_settings: None,
            suite: None,
            update_scheme: UpdateScheme::Realtime,
            break_on_ctrlc: false,
            handle: None,
        }
    }

    /// the settings for the launcher soma
    pub fn launcher_settings(self, settings: LauncherSettings) -> Self {
        Self {
            launcher_settings: Some(settings),
            ..self
        }
    }

    /// play one game with the given settings
    pub fn one_and_done(self, game: GameSetup) -> Self {
        Self {
            suite: Some(MeleeSuite::OneAndDone(game)),
            ..self
        }
    }

    /// keep restarting game with the given settings
    pub fn repeat_forever(self, game: GameSetup) -> Self {
        Self {
            suite: Some(MeleeSuite::EndlessRepeat(game)),
            ..self
        }
    }

    /// the method of updating the game instance
    pub fn update_scheme(self, scheme: UpdateScheme) -> Self {
        Self {
            update_scheme: scheme,
            ..self
        }
    }

    /// stop running upon CTRL-C
    ///
    /// this is only necessary with Wine. CTRL-C doesn't seem to kill it for
    /// some reason by default.
    pub fn break_on_ctrlc(self, flag: bool) -> Self {
        Self {
            break_on_ctrlc: flag,
            ..self
        }
    }

    /// the tokio core handle to use
    pub fn handle(self, handle: reactor::Handle) -> Self {
        Self {
            handle: Some(handle),
            ..self
        }
    }

    pub fn add_player<T>(mut self, player: T) -> Self
    where
        T: MeleeCompetitor + Sized + 'static,
    {
        self.players.push(Box::new(player));
        self
    }

    /// build the melee coordinator
    pub fn create(self) -> Result<Melee> {
        if self.launcher_settings.is_none() {
            bail!("missing launcher settings")
        } else if self.suite.is_none() {
            bail!("missing melee suite")
        } else if self.handle.is_none() {
            bail!("missing reactor handle")
        }

        let handle = self.handle.unwrap();

        let mut melee_clients = vec![];

        for mut player in self.players {
            let (tx, rx) = mpsc::channel(10);

            melee_clients.push(MeleeClient { tx: tx });

            player.spawn(&handle, MeleeDendrite { rx: rx })?;
        }

        let mut organelle = Organelle::new(
            MeleeSoma::axon(
                self.suite.unwrap(),
                self.update_scheme,
                Launcher::create(self.launcher_settings.unwrap())?,
                melee_clients,
            )?,
            handle.clone(),
        );

        let melee = organelle.nucleus();

        if self.break_on_ctrlc {
            organelle.add_soma(CtrlcBreakerSoma::axon());
        }

        Ok(Melee {
            handle: handle,
            organelle: organelle,
        })
    }
}

/// coordinates matches between the given players
pub struct Melee {
    handle: reactor::Handle,
    organelle: Organelle<Axon<MeleeSoma>>,
}

impl IntoFuture for Melee {
    type Item = ();
    type Error = Error;
    type Future = Box<Future<Item = Self::Item, Error = Self::Error>>;

    #[async(boxed)]
    fn into_future(self) -> Result<()> {
        await!(self.organelle.run(self.handle))?;
        Ok(())
    }
}

/// suite of games to choose from when pitting bots against each other
enum MeleeSuite {
    OneAndDone(GameSetup),
    /// repeat this game indefinitely
    EndlessRepeat(GameSetup),
}

/// controller that pits agents against each other
pub struct MeleeSoma {
    suite: Option<MeleeSuite>,
    agents: Vec<Option<MeleeClient>>,
    update_scheme: UpdateScheme,
    launcher: Option<Launcher>,
}

impl MeleeSoma {
    /// melee soma only works as a controller in a melee organelle
    fn axon(
        suite: MeleeSuite,
        update_scheme: UpdateScheme,
        launcher: Launcher,
        melee_clients: Vec<MeleeClient>,
    ) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                suite: Some(suite),
                agents: melee_clients
                    .into_iter()
                    .map(|c| Some(c))
                    .collect(),
                update_scheme: update_scheme,
                launcher: Some(launcher),
            },
            vec![],
            vec![],
        ))
    }
}

impl Soma for MeleeSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, msg: Impulse<Self::Synapse>) -> Result<Self> {
        match msg {
            Impulse::Start(_, main_tx, handle) => {
                assert!(self.launcher.is_some());
                assert!(self.suite.is_some());

                if self.agents.len() != 2 {
                    bail!("expected 2 agents, got {}", self.agents.len())
                }

                assert!(self.agents[0].is_some() && self.agents[1].is_some());

                let main_tx2 = main_tx.clone();

                handle.spawn(
                    run_melee(
                        mem::replace(&mut self.suite, None).unwrap(),
                        self.update_scheme,
                        mem::replace(&mut self.launcher, None).unwrap(),
                        (
                            mem::replace(&mut self.agents[0], None).unwrap(),
                            mem::replace(&mut self.agents[1], None).unwrap(),
                        ),
                        main_tx2,
                    ).or_else(move |e| {
                        main_tx
                            .send(Impulse::Error(e.into()))
                            .map(|_| ())
                            .map_err(|_| ())
                    }),
                );

                Ok(self)
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

#[async]
fn run_melee(
    suite: MeleeSuite,
    update_scheme: UpdateScheme,
    mut launcher: Launcher,
    agents: (MeleeClient, MeleeClient),
    main_tx: mpsc::Sender<Impulse<Synapse>>,
) -> Result<()> {
    let (game, _suite) = match suite {
        MeleeSuite::OneAndDone(game) => (game, None),
        MeleeSuite::EndlessRepeat(game) => {
            let suite = Some(MeleeSuite::EndlessRepeat(game.clone()));

            (game, suite)
        },
    };

    let player1 = await!(agents.0.clone().get_player_setup(game.clone()))?;
    let player2 = await!(agents.1.clone().get_player_setup(game.clone()))?;

    let is_pvp = {
        if player1.is_player() && player2.is_computer() {
            false
        } else if player1.is_computer() && player2.is_player() {
            false
        } else if player1.is_player() && player2.is_player() {
            true
        } else {
            bail!("invalid player setups")
        }
    };

    if is_pvp {
        // launch both at the same time
        let instances = {
            let instance1 = launcher.launch()?;
            let instance2 = launcher.launch()?;

            (instance1, instance2)
        };

        // connect to both at the same time
        {
            let connect1 = agents.0.clone().connect(instances.0.get_url()?);
            let connect2 = agents.1.clone().connect(instances.1.get_url()?);

            await!(connect1.join(connect2))?;
        }

        await!(
            agents
                .0
                .clone()
                .create_game(game.clone(), vec![player1, player2])
        )?;

        let mut ports = launcher.create_game_ports();

        ports.client_ports.push(instances.0.ports);
        ports.client_ports.push(instances.1.ports);

        {
            let join1 = agents
                .0
                .clone()
                .join_game(player1, Some(ports.clone()));
            let join2 = agents.1.clone().join_game(player2, Some(ports));

            await!(join1.join(join2))?;
        }

        {
            let run1 = agents.0.clone().run_game(update_scheme);
            let run2 = agents.1.clone().run_game(update_scheme);

            await!(run1.join(run2))?;
        }

        // just quit for now
        await!(
            main_tx
                .send(Impulse::Stop)
                .map(|_| ())
                .map_err(|_| Error::from("unable to stop"))
        )?;
    } else {
        let (player, computer) = if player1.is_computer() {
            ((agents.1, player2), (agents.0, player1))
        } else if player2.is_computer() {
            ((agents.0, player1), (agents.1, player2))
        } else {
            unreachable!()
        };

        assert!(player.1.is_player() && computer.1.is_computer());

        let instance = launcher.launch()?;

        await!(player.0.clone().connect(instance.get_url()?))?;
        await!(
            player
                .0
                .clone()
                .create_game(game.clone(), vec![player.1, computer.1])
        )?;
        await!(player.0.clone().join_game(player.1, None))?;

        await!(player.0.clone().run_game(update_scheme))?;

        // just quit for now
        await!(
            main_tx
                .send(Impulse::Stop)
                .map(|_| ())
                .map_err(|_| Error::from("unable to stop"))
        )?;
    }

    Ok(())
}

#[derive(Debug)]
enum MeleeRequest {
    PlayerSetup(GameSetup, oneshot::Sender<PlayerSetup>),
    Connect(Url, oneshot::Sender<()>),

    CreateGame(GameSetup, Vec<PlayerSetup>, oneshot::Sender<()>),
    JoinGame(
        PlayerSetup,
        Option<GamePorts>,
        oneshot::Sender<()>,
    ),
    RunGame(UpdateScheme, oneshot::Sender<()>),
}

/// wrapper around a sender to provide a melee interface
#[derive(Debug, Clone)]
pub struct MeleeClient {
    tx: mpsc::Sender<MeleeRequest>,
}

/// interface to be enforced by melee dendrites
pub trait MeleeContract: Sized {
    /// errors from the dendrite
    type Error: std::error::Error + Send + Into<Error>;

    /// fetch the player setup from the agent
    fn get_player_setup(
        self,
        game: GameSetup,
    ) -> Box<Future<Item = (Self, PlayerSetup), Error = Self::Error>>;

    /// connect to an instance
    fn connect(self, url: Url)
        -> Box<Future<Item = Self, Error = Self::Error>>;

    /// create a game
    fn create_game(
        self,
        game: GameSetup,
        players: Vec<PlayerSetup>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// join a game
    fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// run the game
    fn run_game(
        self,
        update_scheme: UpdateScheme,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;
}

/// wrapper around a receiver to provide a controlled interface
#[derive(Debug)]
pub struct MeleeDendrite {
    rx: mpsc::Receiver<MeleeRequest>,
}

impl MeleeDendrite {
    fn new(rx: mpsc::Receiver<MeleeRequest>) -> Self {
        Self { rx: rx }
    }

    /// wrap a dendrite and use the contract to respond to any requests
    #[async]
    pub fn wrap<T>(self, mut dendrite: T) -> Result<()>
    where
        T: MeleeContract + 'static,
    {
        #[async]
        for req in self.rx.map_err(|_| -> Error { unreachable!() }) {
            match req {
                MeleeRequest::PlayerSetup(game, tx) => {
                    let result = await!(dendrite.get_player_setup(game))
                        .map_err(|e| e.into())?;

                    tx.send(result.1).unwrap();

                    dendrite = result.0;
                },
                MeleeRequest::Connect(url, tx) => {
                    dendrite =
                        await!(dendrite.connect(url)).map_err(|e| e.into())?;

                    tx.send(()).unwrap();
                },
                MeleeRequest::CreateGame(game, players, tx) => {
                    dendrite = await!(
                        dendrite
                            .create_game(game, players)
                            .map_err(|e| e.into())
                    )?;

                    tx.send(()).unwrap();
                },
                MeleeRequest::JoinGame(setup, ports, tx) => {
                    dendrite = await!(
                        dendrite
                            .join_game(setup, ports)
                            .map_err(|e| e.into())
                    )?;

                    tx.send(()).unwrap();
                },
                MeleeRequest::RunGame(update_scheme, tx) => {
                    dendrite = await!(
                        dendrite
                            .run_game(update_scheme)
                            .map_err(|e| e.into())
                    )?;

                    tx.send(()).unwrap();
                },
            }
        }

        Ok(())
    }
}

impl MeleeClient {
    fn new(tx: mpsc::Sender<MeleeRequest>) -> Self {
        Self { tx: tx }
    }

    /// get a player setup from the agent
    #[async]
    pub fn get_player_setup(self, game: GameSetup) -> Result<PlayerSetup> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::PlayerSetup(game, tx))
                .map_err(|_| Error::from("unable to request player setup"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to receive player setup")))
    }

    /// tell agent to connect to instance
    #[async]
    pub fn connect(self, url: Url) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::Connect(url, tx))
                .map_err(|_| Error::from("unable to request player setup"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to receive player setup")))
    }

    /// tell agent to create a game
    #[async]
    pub fn create_game(
        self,
        game: GameSetup,
        players: Vec<PlayerSetup>,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::CreateGame(game, players, tx))
                .map_err(|_| Error::from("unable to create game"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to create game")))
    }

    /// tell agent to join a game
    #[async]
    pub fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::JoinGame(setup, ports, tx))
                .map_err(|_| Error::from("unable to join game"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to join game")))
    }

    /// run the game to completion
    #[async]
    pub fn run_game(self, update_scheme: UpdateScheme) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::RunGame(update_scheme, tx))
                .map_err(|_| Error::from("unable to run game"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to run game")))
    }
}

// /// MeleeSoma state that pits players against the built-in AI
// pub struct PlayerVsComputer {
//     suite: Option<MeleeSuite>,

//     game: GameSettings,
//     player_setup: PlayerSetup,
//     computer_setup: PlayerSetup,

//     player: Handle,
// }

// impl PlayerVsComputer {
//     fn start(
//         suite: Option<MeleeSuite>,
//         player: (Handle, PlayerSetup),
//         computer: (Handle, PlayerSetup),
//         game: GameSettings,
//     ) -> Result<MeleeSoma> {
//         Ok(MeleeSoma::PlayerVsComputer(PlayerVsComputer {
//             suite: suite,

//             game: game,
//             player_setup: player.1,
//             computer_setup: computer.1,

//             player: player.0,
//         }))
//     }
//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<MeleeSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::Ready) => {
//                 self.on_agent_ready(axon, src)
//             },
//             Impulse::Signal(src, Signal::GameCreated) => {
//                 self.on_game_created(axon, src)
//             },
//             Impulse::Signal(src, Signal::GameEnded) => {
//                 self.on_game_ended(axon, src)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }

//     fn on_agent_ready(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
//         if src != self.player {
//             bail!("expected source of Ready to be the agent")
//         }

//         axon.effector()?.send(
//             self.player,
//             Signal::CreateGame(
//                 self.game.clone(),
//                 vec![self.player_setup, self.computer_setup],
//             ),
//         );

//         Ok(MeleeSoma::PlayerVsComputer(self))
//     }

//     fn on_game_created(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
//         if src != self.player {
//             bail!("expected source of GameCreated to be the agent")
//         }

//         axon.effector()?
//             .send(self.player, Signal::GameReady(self.player_setup, None));

//         Ok(MeleeSoma::PlayerVsComputer(self))
//     }

//     fn on_game_ended(self, axon: &Axon, src: Handle) -> Result<MeleeSoma> {
//         if src != self.player {
//             bail!("expected source of GameEnded to be an agent")
//         }

//         if self.suite.is_none() {
//             Completed::complete(axon)
//         } else {
//             Setup::setup(axon, self.suite.unwrap())
//         }
//     }
// }

// pub struct Completed;

// impl Completed {
//     fn complete(axon: &Axon) -> Result<MeleeSoma> {
//         axon.effector()?.stop();

//         Ok(MeleeSoma::Completed(Completed {}))
//     }

//     fn update(
//         self,
//         _axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<MeleeSoma> {
//         match msg {
// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }
