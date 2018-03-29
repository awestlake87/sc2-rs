use std;
use std::mem;
use std::rc::Rc;

use futures::future;
use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use sc2_proto::sc2api;
use tokio_core::reactor;
use url::Url;

use super::{Error, IntoProto, Result};
use action::{ActionBuilder, ActionClient, ActionControlClient};
use client::{ProtoClient, ProtoClientBuilder};
use data::{GameSetup, Map, PlayerSetup, Race, Unit, Upgrade};
use launcher::GamePorts;
use melee::{MeleeCompetitor, MeleeContract, MeleeDendrite, UpdateScheme};
use observer::{ObserverBuilder, ObserverClient, ObserverControlClient};

/// an event from the game
#[derive(Debug, Clone)]
pub enum Event {
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

/// notify the coordinator that we are done with this event.
#[derive(Debug)]
pub struct EventAck {
    tx: oneshot::Sender<()>,
}

impl EventAck {
    #[async]
    pub fn done(self) -> Result<()> {
        self.tx
            .send(())
            .map_err(|_| Error::from("unable to ack event"))
    }
}

/// build an agent
pub struct AgentBuilder {
    client: Option<ProtoClientBuilder>,
    action: Option<ActionBuilder>,
    observer: Option<ObserverBuilder>,

    race: Option<Race>,

    event_tx: Option<mpsc::Sender<(Event, EventAck)>>,
    event_rx: Option<mpsc::Receiver<(Event, EventAck)>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        let client = ProtoClientBuilder::new();
        let action = ActionBuilder::new().proto_client(client.add_client());
        let observer = ObserverBuilder::new().proto_client(client.add_client());

        let (tx, rx) = mpsc::channel(10);

        Self {
            client: Some(client),
            action: Some(action),
            observer: Some(observer),

            race: None,

            event_tx: Some(tx),
            event_rx: Some(rx),
        }
    }

    pub fn race(self, race: Race) -> Self {
        Self {
            race: Some(race),
            ..self
        }
    }

    pub fn add_observer_client(&self) -> ObserverClient {
        self.observer.as_ref().unwrap().add_client()
    }

    pub fn add_action_client(&self) -> ActionClient {
        self.action.as_ref().unwrap().add_client()
    }

    pub fn take_event_stream(
        &mut self,
    ) -> Result<mpsc::Receiver<(Event, EventAck)>> {
        match mem::replace(&mut self.event_rx, None) {
            Some(rx) => Ok(rx),
            None => bail!("agent stream has already been taken"),
        }
    }
}

impl MeleeCompetitor for AgentBuilder {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        controller: MeleeDendrite,
    ) -> Result<()> {
        let tx = mem::replace(&mut self.event_tx, None).unwrap();

        let agent = Agent::new(
            self.client.as_ref().unwrap().add_client(),
            self.action
                .as_ref()
                .unwrap()
                .add_control_client(),
            self.observer
                .as_ref()
                .unwrap()
                .add_control_client(),
            controller,
            tx,
            mem::replace(&mut self.race, None).unwrap_or(Race::Random),
        );

        mem::replace(&mut self.client, None)
            .unwrap()
            .spawn(handle)?;
        mem::replace(&mut self.action, None)
            .unwrap()
            .spawn(handle)?;
        mem::replace(&mut self.observer, None)
            .unwrap()
            .spawn(handle)?;

        agent.spawn(handle)?;

        Ok(())
    }
}

/// manages a player soma
struct Agent {
    controller: Option<MeleeDendrite>,
    client: Option<ProtoClient>,
    observer: Option<ObserverControlClient>,
    agent: Option<AgentTerminal>,
    action: Option<ActionControlClient>,
}

impl Agent {
    fn new(
        client: ProtoClient,
        action: ActionControlClient,
        observer: ObserverControlClient,
        controller: MeleeDendrite,
        events: mpsc::Sender<(Event, EventAck)>,
        race: Race,
    ) -> Agent {
        Self {
            controller: Some(controller),
            client: Some(client),
            observer: Some(observer),
            agent: Some(AgentTerminal {
                tx: events,
                race: race,
            }),
            action: Some(action),
        }
    }

    fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        handle.spawn(
            self.controller
                .unwrap()
                .wrap(AgentMeleeDendrite {
                    client: self.client.unwrap(),
                    observer: self.observer.unwrap(),
                    action: self.action.unwrap(),
                    agent: self.agent.unwrap(),
                })
                .map_err(|e| panic!("{:#?}", e)),
        );

        Ok(())
    }
}

pub struct AgentMeleeDendrite {
    client: ProtoClient,
    observer: ObserverControlClient,
    action: ActionControlClient,
    agent: AgentTerminal,
}

impl MeleeContract for AgentMeleeDendrite {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, game: GameSetup) -> Result<(Self, PlayerSetup)> {
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
        settings: GameSetup,
        players: Vec<PlayerSetup>,
    ) -> Result<Self> {
        let mut req = sc2api::Request::new();

        match settings.get_map() {
            &Map::LocalMap(ref path) => {
                req.mut_create_game()
                    .mut_local_map()
                    .set_map_path(match path.clone()
                        .into_os_string()
                        .into_string()
                    {
                        Ok(s) => s,
                        Err(_) => bail!("invalid path string"),
                    });
            },
            &Map::BlizzardMap(ref map) => {
                req.mut_create_game()
                    .set_battlenet_map_name(map.clone());
            },
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                PlayerSetup::Computer(race, difficulty) => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.into_proto()?);
                },
                PlayerSetup::Player(race) => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.into_proto()?);
                }, /*PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }*/
            }

            req.mut_create_game()
                .mut_player_setup()
                .push(setup);
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
            PlayerSetup::Computer(race, _) => {
                req.mut_join_game().set_race(race.into_proto()?);
            },
            PlayerSetup::Player(race) => {
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

        await!(
            self.agent
                .clone()
                .handle_event(Event::GameLoaded)
        )?;

        Ok(self)
    }

    #[async(boxed)]
    fn run_game(self, update_scheme: UpdateScheme) -> Result<Self> {
        await!(self.observer.clone().reset())?;

        let (initial_events, _) = await!(self.observer.clone().step())?;

        await!(
            self.agent
                .clone()
                .handle_event(Event::GameStarted)
        )?;
        for e in initial_events {
            await!(self.agent.clone().handle_event(e))?;
        }

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

            await!(self.agent.clone().handle_event(Event::Step))?;

            if game_ended {
                await!(
                    self.agent
                        .clone()
                        .handle_event(Event::GameEnded)
                )?;
                break;
            }

            await!(self.action.clone().step())?;
        }

        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct AgentTerminal {
    tx: mpsc::Sender<(Event, EventAck)>,
    race: Race,
}

impl AgentTerminal {
    #[async]
    fn get_player_setup(self, game: GameSetup) -> Result<PlayerSetup> {
        Ok(PlayerSetup::Player(self.race))
    }

    #[async]
    fn handle_event(self, event: Event) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send((event, EventAck { tx: tx }))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send event"))
        )?;

        if let Err(e) = await!(rx) {
            // ACK went out of scope, we can assume this means they are done
            // using the event
        }

        Ok(())
    }
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
