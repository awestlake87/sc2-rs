use std;

use ctrlc;
use futures::future::Either;
use futures::prelude::*;
use futures::sync;
use futures::unsync::{mpsc, oneshot};
use tokio_core::reactor;
use url::Url;

use super::{Error, Result};
use data::{GameSetup, PlayerSetup};
use launcher::{GamePorts, Launcher, LauncherSettings};

/// Update scheme for the agents to use.
#[derive(Debug, Copy, Clone)]
pub enum UpdateScheme {
    /// Update as fast as possible.
    Realtime,
    /// Step the game with a fixed interval.
    Interval(u32),
}

pub trait MeleeCompetitor {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        controller: MeleeDendrite,
    ) -> Result<()>;
}

/// Build a Melee coordinator.
pub struct MeleeBuilder {
    players: Vec<Box<MeleeCompetitor>>,

    launcher_settings: Option<LauncherSettings>,
    suite: Option<MeleeSuite>,
    update_scheme: UpdateScheme,
    break_on_ctrlc: bool,
    handle: Option<reactor::Handle>,
}

impl MeleeBuilder {
    /// Start building a Melee coordinator.
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

    /// The settings for the launcher.
    pub fn launcher_settings(self, settings: LauncherSettings) -> Self {
        Self {
            launcher_settings: Some(settings),
            ..self
        }
    }

    /// Play one game with the given settings.
    pub fn one_and_done(self, game: GameSetup) -> Self {
        Self {
            suite: Some(MeleeSuite::OneAndDone(game)),
            ..self
        }
    }

    /// Keep restarting game with the given settings.
    pub fn repeat_forever(self, game: GameSetup) -> Self {
        Self {
            suite: Some(MeleeSuite::EndlessRepeat(game)),
            ..self
        }
    }

    /// Step the game instance with a discrete interval
    pub fn step_interval(self, steps: u32) -> Self {
        Self {
            update_scheme: UpdateScheme::Interval(steps),
            ..self
        }
    }

    /// Step the bot as fast as possible
    pub fn step_realtime(self) -> Self {
        Self {
            update_scheme: UpdateScheme::Realtime,
            ..self
        }
    }

    /// Stop running upon CTRL-C.
    ///
    /// this is only necessary with Wine. CTRL-C doesn't seem to kill it for
    /// some reason by default.
    pub fn break_on_ctrlc(self, flag: bool) -> Self {
        Self {
            break_on_ctrlc: flag,
            ..self
        }
    }

    /// Add a player to the Melee coordinator.
    pub fn add_player<T>(mut self, player: T) -> Self
    where
        T: MeleeCompetitor + Sized + 'static,
    {
        self.players.push(Box::new(player));
        self
    }

    /// Provide a handle to spawn background tasks.
    pub fn handle(self, handle: &reactor::Handle) -> Self {
        Self {
            handle: Some(handle.clone()),
            ..self
        }
    }

    /// Build the Melee coordinator.
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

        assert!(melee_clients.len() == 2);

        Ok(Melee {
            suite: self.suite.unwrap(),
            update_scheme: self.update_scheme,
            launcher: Launcher::create(self.launcher_settings.unwrap())?,
            agents: melee_clients,

            break_on_ctrlc: self.break_on_ctrlc,
        })
    }
}

enum MeleeSuite {
    OneAndDone(GameSetup),
    EndlessRepeat(GameSetup),
}

pub struct Melee {
    suite: MeleeSuite,
    agents: Vec<MeleeClient>,
    update_scheme: UpdateScheme,
    launcher: Launcher,

    break_on_ctrlc: bool,
}

impl IntoFuture for Melee {
    type Item = ();
    type Error = Error;
    type Future = Box<Future<Item = Self::Item, Error = Self::Error>>;

    #[async(boxed)]
    fn into_future(self) -> Result<()> {
        if self.break_on_ctrlc {
            let (tx, rx) = sync::mpsc::channel(1);

            ctrlc::set_handler(move || {
                if let Err(e) = tx.clone().send(()).wait() {
                    eprintln!("unable to send Ctrl-C signal {:?}", e);
                }
            })?;

            await!(
                self.run().select2(rx.into_future(),).then(
                    |result| match result {
                        Ok(_) => Ok(()),
                        Err(Either::A((e, _))) => Err(e),
                        Err(Either::B((_, _))) => {
                            Err(Error::from("CTRL-C handler failed"))
                        },
                    },
                )
            )?;
        }
        Ok(())
    }
}

impl Melee {
    #[async]
    fn run(mut self) -> Result<()> {
        let (game, _suite) = match self.suite {
            MeleeSuite::OneAndDone(game) => (game, None),
            MeleeSuite::EndlessRepeat(game) => {
                let suite = Some(MeleeSuite::EndlessRepeat(game.clone()));

                (game, suite)
            },
        };

        let player1 = await!(
            self.agents[0]
                .clone()
                .get_player_setup(game.clone())
        )?;
        let player2 = await!(
            self.agents[1]
                .clone()
                .get_player_setup(game.clone())
        )?;

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
                let instance1 = self.launcher.launch()?;
                let instance2 = self.launcher.launch()?;

                (instance1, instance2)
            };

            // connect to both at the same time
            {
                let connect1 = self.agents[0]
                    .clone()
                    .connect(instances.0.get_url()?);
                let connect2 = self.agents[1]
                    .clone()
                    .connect(instances.1.get_url()?);

                await!(connect1.join(connect2))?;
            }

            await!(
                self.agents[0]
                    .clone()
                    .create_game(game.clone(), vec![player1, player2])
            )?;

            let mut ports = self.launcher.create_game_ports();

            ports.client_ports.push(instances.0.ports);
            ports.client_ports.push(instances.1.ports);

            {
                let join1 = self.agents[0]
                    .clone()
                    .join_game(player1, Some(ports.clone()));
                let join2 = self.agents[1]
                    .clone()
                    .join_game(player2, Some(ports));

                await!(join1.join(join2))?;
            }

            {
                let run1 = self.agents[0]
                    .clone()
                    .run_game(self.update_scheme);
                let run2 = self.agents[1]
                    .clone()
                    .run_game(self.update_scheme);

                await!(run1.join(run2))?;
            }

            // just quit for now
            return Ok(());
        } else {
            let (player, computer) = if player1.is_computer() {
                (
                    (self.agents[1].clone(), player2),
                    (self.agents[0].clone(), player1),
                )
            } else if player2.is_computer() {
                (
                    (self.agents[0].clone(), player1),
                    (self.agents[1].clone(), player2),
                )
            } else {
                unreachable!()
            };

            assert!(player.1.is_player() && computer.1.is_computer());

            let instance = self.launcher.launch()?;

            await!(player.0.clone().connect(instance.get_url()?))?;
            await!(
                player
                    .0
                    .clone()
                    .create_game(game.clone(), vec![player.1, computer.1])
            )?;
            await!(player.0.clone().join_game(player.1, None))?;

            await!(player.0.clone().run_game(self.update_scheme))?;

            // just quit for now
            return Ok(());
        }
    }
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

/// Wrapper around a sender to provide a melee interface.
#[derive(Debug, Clone)]
pub struct MeleeClient {
    tx: mpsc::Sender<MeleeRequest>,
}

/// Interface to be enforced by melee dendrites.
pub trait MeleeContract: Sized {
    /// Errors from the dendrite.
    type Error: std::error::Error + Send + Into<Error>;

    /// Fetch the player setup from the agent.
    fn get_player_setup(
        self,
        game: GameSetup,
    ) -> Box<Future<Item = (Self, PlayerSetup), Error = Self::Error>>;

    /// Connect to an instance.
    fn connect(self, url: Url)
        -> Box<Future<Item = Self, Error = Self::Error>>;

    /// Create a game.
    fn create_game(
        self,
        game: GameSetup,
        players: Vec<PlayerSetup>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// Join a game.
    fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// Run the game.
    fn run_game(
        self,
        update_scheme: UpdateScheme,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;
}

/// Wrapper around a receiver to provide a controlled interface.
#[derive(Debug)]
pub struct MeleeDendrite {
    rx: mpsc::Receiver<MeleeRequest>,
}

impl MeleeDendrite {
    /// Wrap a dendrite and use the contract to respond to any requests.
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
    /// Get a player setup from the agent.
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

    /// Tell agent to connect to instance.
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

    /// Tell agent to create a game.
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

    /// Tell agent to join a game.
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

    /// Run the game to completion.
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
