use std::mem;
use std::rc::Rc;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use sc2_proto::sc2api;
use tokio_core::reactor;
use url::Url;

use super::action_service::{
    ActionBuilder,
    ActionClient,
    ActionControlClient,
    DebugClient,
};
use super::client_service::{ProtoClient, ProtoClientBuilder};
use super::melee_service::{MeleeCompetitor, MeleeRequest, UpdateScheme};
use super::observer_service::{
    ObserverBuilder,
    ObserverClient,
    ObserverControlClient,
};
use constants::sc2_bug_tag;
use data::{GameSetup, Map, PlayerSetup, Race, Unit, Upgrade};
use launcher::GamePorts;
use {Error, ErrorKind, IntoProto, Result};

/// An event from the game.
#[derive(Debug, Clone)]
pub enum Event {
    /// Game has loaded - not called for fast restarts.
    GameLoaded,
    /// Game has started.
    GameStarted,
    /// Game has ended.
    GameEnded,

    /// A unit was destroyed.
    UnitDestroyed(Rc<Unit>),
    /// A unit was created.
    UnitCreated(Rc<Unit>),
    /// A unit does not have any orders.
    UnitIdle(Rc<Unit>),
    /// A unit was detected.
    UnitDetected(Rc<Unit>),

    /// An upgrade completed.
    UpgradeCompleted(Upgrade),
    /// A unit finished constructing a building.
    BuildingCompleted(Rc<Unit>),

    /// Number of nydus worms detected.
    NydusWormsDetected(u32),
    /// Number of nukes launched.
    NukesDetected(u32),

    /// Step the agent or observer.
    Step,
}

/// Notify the coordinator that we are done with this event.
#[derive(Debug)]
pub struct EventAck {
    tx: oneshot::Sender<()>,
}

impl EventAck {
    /// Send a signal indicating that the user is done handling this event.
    #[async]
    pub fn done(self) -> Result<()> {
        self.tx.send(()).map_err(|_| -> Error {
            unreachable!("{}: Unable to ack event", sc2_bug_tag())
        })
    }
}

/// Build an agent.
pub struct AgentBuilder {
    client: Option<ProtoClientBuilder>,
    action: Option<ActionBuilder>,
    observer: Option<ObserverBuilder>,

    race: Option<Race>,

    event_tx: Option<mpsc::Sender<(Event, EventAck)>>,
    event_rx: Option<mpsc::Receiver<(Event, EventAck)>>,
}

impl AgentBuilder {
    /// Create a new AgentBuilder with the default settings.
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

    /// Set the race of the player.
    pub fn race(self, race: Race) -> Self {
        Self {
            race: Some(race),
            ..self
        }
    }

    /// Add an Observer client to observe the game state.
    pub fn add_observer_client(&self) -> ObserverClient {
        self.observer.as_ref().unwrap().add_client()
    }

    /// Add an Action client to dispatch commands.
    pub fn add_action_client(&self) -> ActionClient {
        self.action
            .as_ref()
            .unwrap()
            .add_action_client()
    }

    /// Add a Debug client to use debugging tools.
    pub fn add_debug_client(&self) -> DebugClient {
        self.action.as_ref().unwrap().add_debug_client()
    }

    /// Take the stream of game events to listen for.
    ///
    /// This should be called only once per builder! Subsequent calls will
    /// return None because Streams should not be shared.
    ///
    /// The stream item is a tuple containing the event, and a promise to be
    /// fulfilled once the user is done with the event.
    pub fn take_event_stream(
        &mut self,
    ) -> Option<mpsc::Receiver<(Event, EventAck)>> {
        mem::replace(&mut self.event_rx, None)
    }
}

impl MeleeCompetitor for AgentBuilder {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        rx: mpsc::Receiver<MeleeRequest>,
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
            rx,
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

struct Agent {
    control_rx: Option<mpsc::Receiver<MeleeRequest>>,
    client: ProtoClient,
    observer: ObserverControlClient,
    agent: AgentTerminal,
    action: ActionControlClient,
}

impl Agent {
    fn new(
        client: ProtoClient,
        action: ActionControlClient,
        observer: ObserverControlClient,
        control_rx: mpsc::Receiver<MeleeRequest>,
        events: mpsc::Sender<(Event, EventAck)>,
        race: Race,
    ) -> Agent {
        Self {
            control_rx: Some(control_rx),
            client: client,
            observer: observer,
            agent: AgentTerminal {
                tx: events,
                race: race,
            },
            action: action,
        }
    }

    fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        handle.spawn(self.run().map_err(|e| {
            panic!(
                "{}: AgentService ended unexpectedly {:#?}",
                sc2_bug_tag(),
                e
            )
        }));

        Ok(())
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let control_rx = mem::replace(&mut self.control_rx, None).unwrap();

        #[async]
        for req in control_rx.map_err(|_| -> Error { unreachable!() }) {
            match req {
                MeleeRequest::PlayerSetup(game, tx) => {
                    let setup = await!(self.get_player_setup(game))?;
                    tx.send(setup).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to rsp PlayerSetup",
                            sc2_bug_tag()
                        )
                    })?;
                },
                MeleeRequest::Connect(url, tx) => {
                    await!(self.connect(url))?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to ack connect", sc2_bug_tag())
                    })?;
                },

                MeleeRequest::CreateGame(game, players, tx) => {
                    await!(self.create_game(game, players))?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to create game", sc2_bug_tag())
                    })?;
                },
                MeleeRequest::JoinGame(player, ports, tx) => {
                    await!(self.join_game(player, ports))?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to join game", sc2_bug_tag())
                    })?;
                },
                MeleeRequest::RunGame(update_scheme, tx) => {
                    await!(self.run_game(update_scheme))?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to run game", sc2_bug_tag())
                    })?;
                },
                MeleeRequest::LeaveGame(tx) => {
                    await!(self.leave_game())?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to leave game", sc2_bug_tag())
                    })?;
                },

                MeleeRequest::Disconnect(tx) => {
                    await!(self.disconnect())?;
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to disconnect", sc2_bug_tag())
                    })?;
                },
            }
        }

        Ok(())
    }

    fn get_player_setup(
        &self,
        game: GameSetup,
    ) -> impl Future<Item = PlayerSetup, Error = Error> {
        let future = self.agent.get_player_setup(game);

        async_block! {
            await!(future)
        }
    }

    fn connect(&self, url: Url) -> impl Future<Item = (), Error = Error> {
        let future = self.client.connect(url);

        async_block! {
            await!(future)
        }
    }

    fn create_game(
        &self,
        settings: GameSetup,
        players: Vec<PlayerSetup>,
    ) -> impl Future<Item = (), Error = Error> {
        let client = self.client.clone();

        async_block! {
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
                            Err(_) => bail!(ErrorKind::InvalidMapPath(format!(
                                "{:?} cannot be converted to an OS string",
                                *path
                            ))),
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
            await!(client.request(req))?;

            Ok(())
        }
    }

    fn join_game(
        &self,
        setup: PlayerSetup,
        ports: Option<GamePorts>,
    ) -> impl Future<Item = (), Error = Error> {
        let client = self.client.clone();
        let event_future = self.agent.handle_event(Event::GameLoaded);

        async_block! {
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

            await!(client.request(req))?;
            await!(event_future)?;

            Ok(())
        }
    }

    fn run_game(
        &self,
        update_scheme: UpdateScheme,
    ) -> impl Future<Item = (), Error = Error> {
        let observer = self.observer.clone();
        let agent = self.agent.clone();
        let action = self.action.clone();
        let client = self.client.clone();

        async_block! {
            await!(observer.clone().reset())?;

            let (initial_events, _) = await!(observer.clone().step())?;

            await!(
                agent
                    .clone()
                    .handle_event(Event::GameStarted)
            )?;
            for e in initial_events {
                await!(agent.clone().handle_event(e))?;
            }

            loop {
                match update_scheme {
                    UpdateScheme::Realtime => (),
                    UpdateScheme::Interval(interval) => {
                        let mut req = sc2api::Request::new();
                        req.mut_step().set_count(interval);

                        await!(client.clone().request(req))?;
                    },
                }

                let (events, game_ended) = await!(observer.clone().step())?;

                for e in events {
                    await!(agent.clone().handle_event(e))?;
                }

                await!(agent.clone().handle_event(Event::Step))?;

                if game_ended {
                    await!(
                        agent
                            .clone()
                            .handle_event(Event::GameEnded)
                    )?;
                    break;
                }

                await!(action.clone().step())?;
            }

            Ok(())
        }
    }

    fn leave_game(&self) -> impl Future<Item = (), Error = Error> {
        let mut req = sc2api::Request::new();
        req.mut_leave_game();

        let future = self.client.request(req);

        async_block! {
            await!(future)?;

            Ok(())
        }
    }

    fn disconnect(&self) -> impl Future<Item = (), Error = Error> {
        let future = self.client.disconnect();

        async_block! {
            await!(future)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentTerminal {
    tx: mpsc::Sender<(Event, EventAck)>,
    race: Race,
}

impl AgentTerminal {
    fn get_player_setup(
        &self,
        _game: GameSetup,
    ) -> impl Future<Item = PlayerSetup, Error = Error> {
        let race = self.race;

        async_block! {
            Ok(PlayerSetup::Player(race))
        }
    }

    fn handle_event(
        &self,
        event: Event,
    ) -> impl Future<Item = (), Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            if let Err(_) = await!(
                sender
                    .send((event, EventAck { tx: tx }))
            ) {
                // This is not really an error, it just means that the user's
                // event stream has been closed or dropped. For now I'm just
                // dropping the event and continuing.
                //
                // It might be worth adding a warning for this later.
            }

            if let Err(_) = await!(rx) {
                // ACK went out of scope, we can assume this means they are done
                // using the event.
                //
                // It might be worth adding a warning for this later.
            }

            Ok(())
        }
    }
}
