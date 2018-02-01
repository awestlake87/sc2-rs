use std::{self, mem};

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle;
use organelle::{Axon, Constraint, Impulse, Organelle, Soma};
use tokio_core::reactor;
use url::Url;

use super::{Error, Result};
use data::{GamePorts, GameSettings, PlayerSetup};
use launcher::{LauncherSettings, LauncherSoma, LauncherTerminal};
use synapses::{Dendrite, Synapse, Terminal};

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
    pub launcher: LauncherSettings,
    /// the player organelles
    pub players: (L1, L2),
    /// the suite of games to choose from
    pub suite: MeleeSuite,
}

/// create a melee synapse
pub fn synapse() -> (MeleeTerminal, MeleeDendrite) {
    let (tx, rx) = mpsc::channel(1);

    (MeleeTerminal::new(tx), MeleeDendrite::new(rx))
}

/// controller that pits agents against each other
pub struct MeleeSoma {
    suite: Option<MeleeSuite>,
    launcher: Option<LauncherTerminal>,
    agents: Vec<Option<MeleeTerminal>>,
}

impl MeleeSoma {
    /// melee soma only works as a controller in a melee organelle
    fn axon(suite: MeleeSuite) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                suite: Some(suite),
                launcher: None,
                agents: vec![],
            },
            vec![],
            vec![
                Constraint::One(Synapse::Launcher),
                Constraint::Variadic(Synapse::Melee),
            ],
        ))
    }

    /// create the melee organelle
    pub fn organelle<L1, L2>(
        settings: MeleeSettings<L1, L2>,
        handle: reactor::Handle,
    ) -> Result<Organelle<Axon<Self>>>
    where
        L1: Soma,
        L2: Soma,

        L1::Synapse: From<Synapse> + Into<Synapse>,
        L2::Synapse: From<Synapse> + Into<Synapse>,

        <L1::Synapse as organelle::Synapse>::Terminal: From<Terminal>
            + Into<Terminal>,
        <L1::Synapse as organelle::Synapse>::Dendrite: From<Dendrite>
            + Into<Dendrite>,

        <L2::Synapse as organelle::Synapse>::Terminal: From<Terminal>
            + Into<Terminal>,
        <L2::Synapse as organelle::Synapse>::Dendrite: From<Dendrite>
            + Into<Dendrite>,
    {
        let mut organelle =
            Organelle::new(MeleeSoma::axon(settings.suite)?, handle);

        let launcher =
            organelle.add_soma(LauncherSoma::axon(settings.launcher)?);

        let melee = organelle.nucleus();

        let player1 = organelle.add_soma(settings.players.0);
        let player2 = organelle.add_soma(settings.players.1);

        organelle.connect(melee, launcher, Synapse::Launcher)?;

        organelle.connect(melee, player1, Synapse::Melee)?;
        organelle.connect(melee, player2, Synapse::Melee)?;

        Ok(organelle)
    }
}

impl Soma for MeleeSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, msg: Impulse<Self::Synapse>) -> Result<Self> {
        match msg {
            Impulse::AddTerminal(
                _,
                Synapse::Launcher,
                Terminal::Launcher(tx),
            ) => {
                self.launcher = Some(tx);
                Ok(self)
            },
            Impulse::AddTerminal(_, Synapse::Melee, Terminal::Melee(tx)) => {
                self.agents.push(Some(tx));
                Ok(self)
            },
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
    launcher: LauncherTerminal,
    agents: (MeleeTerminal, MeleeTerminal),
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
            let launch1 = launcher.clone().launch();
            let launch2 = launcher.clone().launch();

            await!(launch1.join(launch2))?
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

        let mut ports = await!(launcher.clone().get_game_ports())?;

        ports.client_ports.push(instances.0.ports);
        ports.client_ports.push(instances.1.ports);

        {
            let join1 =
                agents.0.clone().join_game(player1, Some(ports.clone()));
            let join2 = agents.1.clone().join_game(player2, Some(ports));

            await!(join1.join(join2))?;
        }

        {
            let run1 = agents.0.clone().run_game();
            let run2 = agents.1.clone().run_game();

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
        unimplemented!();
    }

    Ok(())
}

#[derive(Debug)]
enum MeleeRequest {
    PlayerSetup(GameSettings, oneshot::Sender<PlayerSetup>),
    Connect(Url, oneshot::Sender<()>),

    CreateGame(GameSettings, Vec<PlayerSetup>, oneshot::Sender<()>),
    JoinGame(PlayerSetup, Option<GamePorts>, oneshot::Sender<()>),
    RunGame(oneshot::Sender<()>),
}

/// wrapper around a sender to provide a melee interface
#[derive(Debug, Clone)]
pub struct MeleeTerminal {
    tx: mpsc::Sender<MeleeRequest>,
}

/// interface to be enforced by melee dendrites
pub trait MeleeContract: Sized {
    /// errors from the dendrite
    type Error: std::error::Error + Send + Into<Error>;

    /// fetch the player setup from the agent
    fn get_player_setup(
        self,
        game: GameSettings,
    ) -> Box<Future<Item = (Self, PlayerSetup), Error = Self::Error>>;

    /// connect to an instance
    fn connect(self, url: Url)
        -> Box<Future<Item = Self, Error = Self::Error>>;

    /// create a game
    fn create_game(
        self,
        game: GameSettings,
        players: Vec<PlayerSetup>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// join a game
    fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Box<Future<Item = Self, Error = Self::Error>>;

    /// run the game
    fn run_game(self) -> Box<Future<Item = Self, Error = Self::Error>>;
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
                        dendrite.join_game(setup, ports).map_err(|e| e.into())
                    )?;

                    tx.send(()).unwrap();
                },
                MeleeRequest::RunGame(tx) => {
                    dendrite =
                        await!(dendrite.run_game().map_err(|e| e.into()))?;

                    tx.send(()).unwrap();
                },
            }
        }

        Ok(())
    }
}

impl MeleeTerminal {
    fn new(tx: mpsc::Sender<MeleeRequest>) -> Self {
        Self { tx: tx }
    }

    /// get a player setup from the agent
    #[async]
    pub fn get_player_setup(self, game: GameSettings) -> Result<PlayerSetup> {
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
        game: GameSettings,
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
    pub fn run_game(self) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(MeleeRequest::RunGame(tx))
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
