use std::{collections::VecDeque, path::PathBuf, rc::Rc};

use ctrlc;
use futures::{
    future::{join_all, Either},
    prelude::*,
    sync,
    unsync::{mpsc, oneshot},
};
use sc2_proto::sc2api;
use tokio_core::reactor;

use constants::{sc2_bug_tag, warning_tag};
use launcher::{Launcher, LauncherSettings};
use replay::{Replay, ReplayInfo, SpectatorChoice};
use services::client_service::{ProtoClient, ProtoClientBuilder};
use wine_utils::convert_to_wine_path;
use {Error, ErrorKind, FromProto, Result};

pub type ReplaySink = mpsc::Sender<Replay>;
type ReplayStream = mpsc::Receiver<Replay>;

pub trait ReplaySpectator {
    fn spawn(&mut self, handle: &reactor::Handle) -> Result<SpectatorClient>;
}

#[derive(Debug)]
pub enum SpectatorRequest {
    WhichPlayer(Rc<ReplayInfo>, oneshot::Sender<SpectatorChoice>),
}

#[derive(Debug, Clone)]
pub struct SpectatorClient {
    tx: mpsc::Sender<SpectatorRequest>,
}

impl SpectatorClient {
    pub fn wrap(tx: mpsc::Sender<SpectatorRequest>) -> Self {
        Self { tx: tx }
    }

    fn which_player(
        &self,
        replay_info: Rc<ReplayInfo>,
    ) -> impl Future<Item = SpectatorChoice, Error = Error> {
        let (tx, rx) = oneshot::channel();
        let future = self.tx
            .clone()
            .send(SpectatorRequest::WhichPlayer(replay_info, tx))
            .map_err(|_| -> Error {
                unreachable!(
                    "{}: Unable to send SpectatorRequest",
                    sc2_bug_tag()
                )
            });

        async_block! {
            await!(future)?;

            await!(rx.map_err(|_| -> Error {
                unreachable!("{}: Unable to recv SpectatorChoice", sc2_bug_tag())
            }))
        }
    }
}

/// Build a Replay coordinator.
pub struct ReplayBuilder {
    launcher_settings: Option<LauncherSettings>,
    break_on_ctrlc: bool,
    handle: Option<reactor::Handle>,

    num_instances: usize,

    replay_tx: ReplaySink,
    replay_rx: ReplayStream,

    spectators: Vec<Box<ReplaySpectator>>,
}

impl ReplayBuilder {
    /// Start building a replay coordinator.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10);

        Self {
            launcher_settings: None,
            break_on_ctrlc: false,
            handle: None,

            num_instances: 1,
            replay_tx: tx,
            replay_rx: rx,

            spectators: vec![],
        }
    }

    /// The settings for the launcher.
    pub fn launcher_settings(self, settings: LauncherSettings) -> Self {
        Self {
            launcher_settings: Some(settings),
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

    /// Add a spectator to the replay coordinator.
    pub fn add_spectator<T>(mut self, spectator: T) -> Self
    where
        T: ReplaySpectator + Sized + 'static,
    {
        self.spectators.push(Box::new(spectator));

        self
    }

    /// The number of instances to create for replays.
    pub fn num_instances(self, num: usize) -> Self {
        assert!(
            num > 0,
            "Replay coordinator needs at least 1 instance"
        );

        Self {
            num_instances: num,
            ..self
        }
    }

    /// Provide a handle to spawn background tasks.
    pub fn handle(self, handle: &reactor::Handle) -> Self {
        Self {
            handle: Some(handle.clone()),
            ..self
        }
    }

    /// Add a sink to send replays to the coordinator
    pub fn add_replay_sink(&self) -> ReplaySink {
        self.replay_tx.clone()
    }

    /// Build the Replay coordinator.
    pub fn create(self) -> Result<ReplayCoordinator> {
        if self.launcher_settings.is_none() {
            bail!(ErrorKind::MissingRequirement(
                "ReplayBuilder needs LauncherSettings".to_string()
            ))
        } else if self.handle.is_none() {
            bail!(ErrorKind::MissingRequirement(
                "ReplayBuilder needs a reactor handle".to_string()
            ))
        }

        let handle = self.handle.unwrap();
        let launcher =
            Launcher::create(self.launcher_settings.unwrap(), handle.clone())?;

        let mut spectator_clients = vec![];

        for mut spectator in self.spectators {
            spectator_clients.push(spectator.spawn(&handle)?);
        }

        Ok(ReplayCoordinator {
            handle: handle,
            launcher: launcher,
            break_on_ctrlc: self.break_on_ctrlc,
            num_instances: self.num_instances,

            replay_rx: self.replay_rx,

            spectators: spectator_clients,
        })
    }
}

pub struct ReplayCoordinator {
    handle: reactor::Handle,
    launcher: Launcher,
    break_on_ctrlc: bool,
    num_instances: usize,

    replay_rx: ReplayStream,

    spectators: Vec<SpectatorClient>,
}

impl IntoFuture for ReplayCoordinator {
    type Item = ();
    type Error = Error;
    type Future = Box<Future<Item = Self::Item, Error = Self::Error>>;

    fn into_future(self) -> Self::Future {
        let break_on_ctrlc = self.break_on_ctrlc;

        let (tx, rx) = sync::mpsc::channel(1);
        let future = self.run()
            .select2(rx.into_future())
            .then(|result| match result {
                Ok(_) => Ok(()),
                Err(Either::A((e, _))) => Err(e),
                Err(Either::B((_, _))) => {
                    panic!("{}: CTRL-C handler failed", sc2_bug_tag());
                },
            });

        Box::new(async_block! {
            if break_on_ctrlc {
                ctrlc::set_handler(move || {
                    if let Err(e) = tx.clone().send(()).wait() {
                        println!(
                            "{}: Unable to send Ctrl-C signal {:?}",
                            warning_tag(),
                            e
                        );
                    }
                })?;
            }

            await!(future)?;

            Ok(())
        })
    }
}

impl ReplayCoordinator {
    #[async]
    fn run(mut self) -> Result<()> {
        // Sanity check because we should have checked for this earlier.
        debug_assert!(
            self.num_instances > 0,
            "Replay coordinator needs at least 1 instance"
        );

        let num_spectators = self.spectators.len();

        let (idle_instances_tx, mut idle_instances_rx) =
            mpsc::channel(self.num_instances);
        let (idle_spectators_tx, mut idle_spectators_rx) =
            mpsc::channel(num_spectators);

        let mut connect_futures = vec![];

        for _ in 0..self.num_instances {
            let instance = self.launcher.launch()?;
            let client_service = ProtoClientBuilder::new();
            let client = client_service.add_client();

            client_service.spawn(&self.handle)?;

            connect_futures.push(client.connect(instance.get_url()?));

            await!(
                idle_instances_tx
                    .clone()
                    .send((instance, ReplayClient::wrap(client)))
                    .map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to queue idle instance",
                            sc2_bug_tag()
                        )
                    })
            )?;
        }

        for spectator in self.spectators {
            await!(
                idle_spectators_tx
                    .clone()
                    .send(spectator)
                    .map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to queue idle spectator",
                            sc2_bug_tag()
                        )
                    })
            )?;
        }

        await!(join_all(connect_futures))?;

        let mut exhausted_spectators = 0;
        let mut buffered_replays = vec![];

        while exhausted_spectators < num_spectators {
            let (instance, client) = await_item!(idle_instances_rx)
                .map_err(|_| -> Error { unreachable!() })?
                .unwrap_or_else(|| {
                    unreachable!("idle instances stream should not end")
                });
            let spectator = await_item!(idle_spectators_rx)
                .map_err(|_| -> Error { unreachable!() })?
                .unwrap_or_else(|| {
                    unreachable!("because of loop precondition")
                });

            let mut handled = false;

            for i in 0..buffered_replays.len() {
                let replay_info = Rc::clone(&buffered_replays[0]);

                let choice =
                    await!(spectator.which_player(Rc::clone(&replay_info)))?;

                match choice {
                    SpectatorChoice::WatchPlayer(player_id) => {
                        handled = true;
                        buffered_replays.swap_remove(i);
                        break;
                    },
                    SpectatorChoice::Pass => continue,
                }
            }

            if !handled {
                while let Some(replay) = await_item!(self.replay_rx)
                    .map_err(|_| -> Error { unreachable!() })?
                {
                    let replay = match replay {
                        Replay::LocalReplay(path) => {
                            if self.launcher.using_wine() {
                                Replay::LocalReplay(await!(
                                    convert_to_wine_path(
                                        path,
                                        self.handle.clone()
                                    )
                                )?)
                            } else {
                                Replay::LocalReplay(path)
                            }
                        },
                    };

                    let replay_info =
                        await!(client.get_replay_info(replay, true))?;

                    let choice = await!(
                        spectator.which_player(Rc::clone(&replay_info))
                    )?;

                    match choice {
                        SpectatorChoice::WatchPlayer(player_id) => {
                            println!("run game for {:#?}", replay_info);
                            handled = true;
                            break;
                        },
                        SpectatorChoice::Pass => {
                            buffered_replays.push(replay_info)
                        },
                    }
                }
            }

            if !handled {
                exhausted_spectators += 1;
            } else {
                await!(
                    idle_spectators_tx
                        .clone()
                        .send(spectator)
                        .map_err(|_| -> Error { unreachable!() })
                )?;
            }

            await!(
                idle_instances_tx
                    .clone()
                    .send((instance, client))
                    .map_err(|_| -> Error { unreachable!() })
            )?;
        }

        Ok(())
    }
}

/// Wrapper around a ProtoClient to communicate with game instance.
#[derive(Debug, Clone)]
struct ReplayClient {
    client: ProtoClient,
}

impl ReplayClient {
    fn wrap(client: ProtoClient) -> Self {
        Self { client: client }
    }

    fn get_replay_info(
        &self,
        replay: Replay,
        download_data: bool,
    ) -> impl Future<Item = Rc<ReplayInfo>, Error = Error> {
        let mut req = sc2api::Request::new();

        match replay {
            Replay::LocalReplay(path) => req.mut_replay_info()
                .set_replay_path(path.to_string_lossy().to_string()),
        }

        req.mut_replay_info()
            .set_download_data(download_data);

        let future = self.client.request(req);

        async_block! {
            let mut rsp = await!(future)?;

            Ok(Rc::from(ReplayInfo::from_proto(rsp.take_replay_info())?))
        }
    }
}

impl Into<ProtoClient> for ReplayClient {
    fn into(self) -> ProtoClient {
        self.client
    }
}
