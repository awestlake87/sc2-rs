use std;
use std::rc::Rc;

use futures::future;
use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{self, Axon, Constraint, Impulse, Organelle, Soma};
use sc2_proto::sc2api;
use tokio_core::reactor;
use url::Url;

use super::{Error, IntoProto, Result};
use action::{ActionControlTerminal, ActionSoma, ActionTerminal};
use client::{ClientSoma, ClientTerminal};
use data::{GameSettings, Map, PlayerSetup, Unit, Upgrade};
use launcher::GamePorts;
use melee::{MeleeCompetitor, MeleeContract, MeleeDendrite, UpdateScheme};
use observer::{ObserverControlTerminal, ObserverSoma, ObserverTerminal};
use synapses::{Dendrite, Synapse, Terminal};

/// an event from the game
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// game has loaded - not called for fast restarts
    GameLoaded,
    /// game has started
    GameStarted,
    /// game has ended
    GameEnded,

    /// a unit was destroyed
    UnitDestroyed(Rc<Unit>),
    /// a unit was created
    UnitCreated(Rc<Unit>),
    /// a unit does not have any orders
    UnitIdle(Rc<Unit>),
    /// a unit was detected
    UnitDetected(Rc<Unit>),

    /// an upgrade completed
    UpgradeCompleted(Upgrade),
    /// a unit finished constructing a building
    BuildingCompleted(Rc<Unit>),

    /// number of nydus worms detected
    NydusWormsDetected(u32),
    /// number of nukes launched
    NukesDetected(u32),

    /// step the agent or observer
    Step,
}

/// controls given to an agent
pub struct AgentControl {
    observer: ObserverTerminal,
    action: ActionTerminal,
}

impl AgentControl {
    /// observe the game data and current state
    pub fn observer(&self) -> ObserverTerminal {
        self.observer.clone()
    }

    /// dispatch commands and debug commands to the game instance
    pub fn action(&self) -> ActionTerminal {
        self.action.clone()
    }
}

pub struct AgentWrapper<T, F>
where
    T: Player + 'static,
    F: FnOnce(AgentControl) -> T,
{
    agent: Option<AgentDendrite>,
    observer: Option<ObserverTerminal>,
    action: Option<ActionTerminal>,

    factory: Option<F>,
}

impl<T, F> Soma for AgentWrapper<T, F>
where
    T: Player + 'static,
    F: FnOnce(AgentControl) -> T + 'static,
{
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(_, Synapse::Agent, Dendrite::Agent(rx)) => {
                Ok(Self {
                    agent: Some(rx),
                    ..self
                })
            },
            Impulse::AddTerminal(
                _,
                Synapse::Observer,
                Terminal::Observer(tx),
            ) => Ok(Self {
                observer: Some(tx),
                ..self
            }),
            Impulse::AddTerminal(_, Synapse::Action, Terminal::Action(tx)) => {
                Ok(Self {
                    action: Some(tx),
                    ..self
                })
            },

            Impulse::Start(_, main_tx, handle) => {
                handle.spawn(
                    self.agent
                        .unwrap()
                        .wrap(self.factory.unwrap()(AgentControl {
                            observer: self.observer.unwrap(),
                            action: self.action.unwrap(),
                        }))
                        .or_else(move |e| {
                            main_tx
                                .send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(Self {
                    agent: None,
                    observer: None,
                    action: None,

                    factory: None,
                })
            },
            _ => bail!("unexpected impulse"),
        }
    }
}

impl<T, F> AgentWrapper<T, F>
where
    T: Player + 'static,
    F: FnOnce(AgentControl) -> T + 'static,
{
    fn new(factory: F) -> Self {
        Self {
            agent: None,
            observer: None,
            action: None,

            factory: Some(factory),
        }
    }
}

/// build an agent
pub struct AgentBuilder<T: Soma + 'static> {
    soma: T,
    handle: Option<reactor::Handle>,
}

impl<T: Soma + 'static> AgentBuilder<T> {
    /// wrap the given soma in an agent
    #[cfg(feature = "with-organelle")]
    pub fn soma(agent: T) -> Self {
        Self {
            soma: agent,
            handle: None,
        }
    }

    /// the tokio core handle to use
    pub fn handle(self, handle: reactor::Handle) -> Self {
        Self {
            handle: Some(handle),
            ..self
        }
    }

    /// create the agent
    pub fn create(self) -> Result<Agent>
    where
        T::Synapse: From<Synapse> + Into<Synapse>,
        <T::Synapse as organelle::Synapse>::Terminal: From<Terminal>
            + Into<Terminal>,
        <T::Synapse as organelle::Synapse>::Dendrite: From<Dendrite>
            + Into<Dendrite>,
    {
        if self.handle.is_none() {
            bail!("missing tokio core handle")
        }

        let handle = self.handle.unwrap();

        let mut organelle = Organelle::new(AgentSoma::axon()?, handle);

        let agent = organelle.nucleus();
        let player = organelle.add_soma(self.soma);

        let client = organelle.add_soma(ClientSoma::axon()?);
        let observer = organelle.add_soma(ObserverSoma::axon()?);
        let action = organelle.add_soma(ActionSoma::axon()?);

        organelle.connect(agent, client, Synapse::Client)?;
        organelle.connect(observer, client, Synapse::Client)?;
        organelle.connect(action, client, Synapse::Client)?;

        organelle.connect(agent, observer, Synapse::ObserverControl)?;
        organelle.connect(agent, action, Synapse::ActionControl)?;

        organelle.connect(agent, player, Synapse::Agent)?;
        organelle.connect(player, observer, Synapse::Observer)?;
        organelle.connect(player, action, Synapse::Action)?;

        Ok(Agent { 0: organelle })
    }
}

impl<T, F> AgentBuilder<AgentWrapper<T, F>>
where
    T: Player + 'static,
    F: FnOnce(AgentControl) -> T,
{
    /// wrap a factory to be called with the agent controls when ready
    pub fn factory(factory: F) -> AgentBuilder<AgentWrapper<T, F>> {
        AgentBuilder::<AgentWrapper<T, F>> {
            soma: AgentWrapper::new(factory),
            handle: None,
        }
    }
}

/// a wrapper around a player to mediate interactions with game instance
///
/// exposes internal sc2 interfaces to players
pub struct Agent(Organelle<Axon<AgentSoma>>);

impl MeleeCompetitor for Agent {
    type Soma = Organelle<Axon<AgentSoma>>;

    fn into_soma(self) -> Self::Soma {
        self.0
    }
}

/// manages a player soma
pub struct AgentSoma {
    controller: Option<MeleeDendrite>,
    client: Option<ClientTerminal>,
    observer: Option<ObserverControlTerminal>,
    agent: Option<AgentTerminal>,
    action: Option<ActionControlTerminal>,
}

impl AgentSoma {
    fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                controller: None,
                client: None,
                observer: None,
                agent: None,
                action: None,
            },
            vec![Constraint::One(Synapse::Melee)],
            vec![
                Constraint::One(Synapse::Client),
                Constraint::One(Synapse::ObserverControl),
                Constraint::One(Synapse::ActionControl),
                Constraint::One(Synapse::Agent),
            ],
        ))
    }
}

impl Soma for AgentSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(_, Synapse::Melee, Dendrite::Melee(rx)) => {
                Ok(Self {
                    controller: Some(rx),
                    ..self
                })
            },
            Impulse::AddTerminal(_, Synapse::Client, Terminal::Client(tx)) => {
                Ok(Self {
                    client: Some(tx),
                    ..self
                })
            },
            Impulse::AddTerminal(
                _,
                Synapse::ObserverControl,
                Terminal::ObserverControl(tx),
            ) => Ok(Self {
                observer: Some(tx),
                ..self
            }),
            Impulse::AddTerminal(
                _,
                Synapse::ActionControl,
                Terminal::ActionControl(tx),
            ) => Ok(Self {
                action: Some(tx),
                ..self
            }),
            Impulse::AddTerminal(_, Synapse::Agent, Terminal::Agent(tx)) => {
                self.agent = Some(tx);

                Ok(self)
            },
            Impulse::Start(_, tx, handle) => {
                handle.spawn(
                    self.controller
                        .unwrap()
                        .wrap(AgentMeleeDendrite {
                            client: self.client.unwrap(),
                            observer: self.observer.unwrap(),
                            action: self.action.unwrap(),
                            agent: self.agent.unwrap(),
                        })
                        .or_else(move |e| {
                            tx.send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(Self {
                    controller: None,
                    client: None,
                    observer: None,
                    agent: None,
                    action: None,
                })
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

pub struct AgentMeleeDendrite {
    client: ClientTerminal,
    observer: ObserverControlTerminal,
    action: ActionControlTerminal,
    agent: AgentTerminal,
}

impl MeleeContract for AgentMeleeDendrite {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(
        self,
        game: GameSettings,
    ) -> Result<(Self, PlayerSetup)> {
        let setup = await!(self.agent.clone().get_player_setup(game))?;

        Ok((self, setup))
    }

    #[async(boxed)]
    fn connect(self, url: Url) -> Result<Self> {
        await!(self.client.clone().connect(url))?;
        Ok(self)
    }

    #[async(boxed)]
    fn create_game(
        self,
        settings: GameSettings,
        players: Vec<PlayerSetup>,
    ) -> Result<Self> {
        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(ref path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.clone().into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => bail!("invalid path string"),
                    },
                );
            },
            Map::BlizzardMap(ref map) => {
                req.mut_create_game().set_battlenet_map_name(map.clone());
            },
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                PlayerSetup::Computer {
                    difficulty, race, ..
                } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Player { race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                }, /*PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }*/
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(false);

        await!(self.client.clone().request(req))?;

        Ok(self)
    }

    #[async(boxed)]
    fn join_game(
        self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> Result<Self> {
        let mut req = sc2api::Request::new();

        match setup {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.into_proto()?);
            }, //_ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        if let Some(ports) = ports {
            req.mut_join_game()
                .set_shared_port(ports.shared_port as i32);

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

        await!(self.client.clone().request(req))?;

        await!(self.agent.clone().handle_event(GameEvent::GameLoaded))?;

        Ok(self)
    }

    #[async(boxed)]
    fn run_game(self, update_scheme: UpdateScheme) -> Result<Self> {
        await!(self.observer.clone().reset())?;

        await!(self.agent.clone().handle_event(GameEvent::GameStarted))?;

        loop {
            match update_scheme {
                UpdateScheme::Realtime => (),
                UpdateScheme::Interval(interval) => {
                    let mut req = sc2api::Request::new();
                    req.mut_step().set_count(interval);

                    await!(self.client.clone().request(req))?;
                },
            }

            let (events, game_ended) = await!(self.observer.clone().step())?;

            for e in events {
                await!(self.agent.clone().handle_event(e))?;
            }

            await!(self.agent.clone().handle_event(GameEvent::Step))?;

            if game_ended {
                await!(self.agent.clone().handle_event(GameEvent::GameEnded))?;
                break;
            }

            await!(self.action.clone().step())?;
        }

        Ok(self)
    }
}

#[derive(Debug)]
enum AgentRequest {
    PlayerSetup(GameSettings, oneshot::Sender<PlayerSetup>),
    Event(GameEvent, oneshot::Sender<()>),
}

#[derive(Debug, Clone)]
pub struct AgentTerminal {
    tx: mpsc::Sender<AgentRequest>,
}

impl AgentTerminal {
    #[async]
    fn get_player_setup(self, game: GameSettings) -> Result<PlayerSetup> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(AgentRequest::PlayerSetup(game, tx))
                .map(|_| ())
                .map_err(|_| Error::from(
                    "unable to send player setup request"
                ))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv player setup")))
    }

    #[async]
    fn handle_event(self, event: GameEvent) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(AgentRequest::Event(event, tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send event"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to recv event response")))
    }
}

/// contract for a player soma to obey
pub trait Player: Sized {
    /// the type of error that can occur upon failure
    type Error: std::error::Error + Send + Into<Error>;

    /// Use the game settings to decide on a player setup
    fn get_player_setup(
        self,
        game: GameSettings,
    ) -> Box<Future<Item = (Self, PlayerSetup), Error = Self::Error>>;

    /// Called whenever the agent needs to handle a game event
    fn on_event(
        self,
        _e: GameEvent,
    ) -> Box<Future<Item = Self, Error = Self::Error>>
    where
        Self: 'static,
    {
        Box::new(future::ok(self))
    }
}

/// receiver for agent requests
#[derive(Debug)]
pub struct AgentDendrite {
    rx: mpsc::Receiver<AgentRequest>,
}

impl AgentDendrite {
    /// wrap a struct that obeys the agent contract and forward requests to
    /// it
    #[async]
    pub fn wrap<T: Player + 'static>(self, mut player: T) -> Result<()> {
        #[async]
        for req in self.rx.map_err(|_| -> Error { unreachable!() }) {
            match req {
                AgentRequest::PlayerSetup(game, tx) => {
                    let result = await!(player.get_player_setup(game))
                        .map_err(|e| e.into())?;

                    player = result.0;

                    tx.send(result.1).map_err(|_| {
                        Error::from("unable to get player setup")
                    })?;
                },
                AgentRequest::Event(e, tx) => {
                    player = await!(player.on_event(e).map_err(|e| e.into()))?;

                    tx.send(())
                        .map_err(|_| Error::from("unable to ack event"))?;
                },
            }
        }

        Ok(())
    }
}

pub fn synapse() -> (AgentTerminal, AgentDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (AgentTerminal { tx: tx }, AgentDendrite { rx: rx })
}

// pub struct LeaveGame {
//     transactor: Transactor,
// }

// impl LeaveGame {
//     fn leave(axon: &Axon) -> Result<AgentSoma> {
//         let mut req = sc2api::Request::new();

//         req.mut_leave_game();

//         let transactor = Transactor::send(axon, ClientRequest::new(req))?;

//         Ok(AgentSoma::LeaveGame(LeaveGame {
//             transactor: transactor,
//         }))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(src, Signal::ClientResult(result)) => {
//                 self.transactor.expect(src, result)?;

//                 Reset::reset(axon)
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }

// pub struct Reset;

// impl Reset {
//     fn reset(axon: &Axon) -> Result<AgentSoma> {
//         axon.send_req_output(Synapse::Client, Signal::ClientDisconnect)?;

//         Ok(AgentSoma::Reset(Reset {}))
//     }

//     fn update(
//         self,
//         axon: &Axon,
//         msg: Impulse<Signal, Synapse>,
//     ) -> Result<AgentSoma> {
//         match msg {
//             Impulse::Signal(_, Signal::ClientError(_)) => {
//                 // client does not close cleanly anyway right now, so just
//                 // ignore the error and wait for ClientClosed.
//                 Ok(AgentSoma::Reset(self))
//             },
//             Impulse::Signal(_, Signal::ClientClosed) => {
//                 axon.send_req_input(Synapse::Controller, Signal::GameEnded)?;
//                 axon.send_req_output(Synapse::Agent, Signal::GameEnded)?;

//                 Setup::setup()
//             },

// Impulse::Signal(_, msg) => bail!("unexpected message {:#?}",
// msg),             _ => bail!("unexpected protocol message"),
//         }
//     }
// }
