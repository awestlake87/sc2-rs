use std::path::PathBuf;

use ctrlc;
use futures::future::Either;
use futures::prelude::*;
use futures::sync;
use futures::unsync::{mpsc, oneshot};
use tokio_core::reactor;
use url::Url;

use constants::{sc2_bug_tag, warning_tag};
use data::{GameSetup, PlayerSetup};
use instance::Instance;
use launcher::{GamePorts, Launcher, LauncherSettings};
use services::UpdateScheme;
use {Error, ErrorKind, Result};

#[derive(Debug, Clone)]
pub enum Replay {
    LocalReplay(PathBuf),
}

pub type ReplaySink = mpsc::Sender<Replay>;
type ReplayStream = mpsc::Receiver<Replay>;

pub trait ReplayObserver {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        controller: mpsc::Receiver<ReplayRequest>,
    ) -> Result<()>;
}

/// Build a Replay coordinator.
pub struct ReplayBuilder {
    launcher_settings: Option<LauncherSettings>,
    break_on_ctrlc: bool,
    handle: Option<reactor::Handle>,

    max_instances: usize,

    replay_tx: ReplaySink,
    replay_rx: ReplayStream,
}

impl ReplayBuilder {
    /// Start building a replay coordinator.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10);

        Self {
            launcher_settings: None,
            break_on_ctrlc: false,
            handle: None,

            max_instances: 1,
            replay_tx: tx,
            replay_rx: rx,
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
        T: ReplayObserver + Sized + 'static,
    {
        unimplemented!()
    }

    /// Set the maximum number of instances the Replay coordinator can create.
    pub fn max_instances(self, max: usize) -> Self {
        Self {
            max_instances: max,
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

        Ok(ReplayCoordinator {
            launcher: Launcher::create(self.launcher_settings.unwrap())?,

            break_on_ctrlc: self.break_on_ctrlc,
            max_instances: self.max_instances,
            replay_rx: self.replay_rx,
        })
    }
}

pub struct ReplayCoordinator {
    launcher: Launcher,

    break_on_ctrlc: bool,

    max_instances: usize,

    replay_rx: ReplayStream,
}

impl IntoFuture for ReplayCoordinator {
    type Item = ();
    type Error = Error;
    type Future = Box<Future<Item = Self::Item, Error = Self::Error>>;

    fn into_future(self) -> Self::Future {
        Box::new(async_block! {
            if self.break_on_ctrlc {
                let (tx, rx) = sync::mpsc::channel(1);

                ctrlc::set_handler(move || {
                    if let Err(e) = tx.clone().send(()).wait() {
                        println!("{}: Unable to send Ctrl-C signal {:?}", warning_tag(), e);
                    }
                })?;

                await!(
                    self.run().select2(rx.into_future(),).then(
                        |result| match result {
                            Ok(_) => Ok(()),
                            Err(Either::A((e, _))) => Err(e),
                            Err(Either::B((_, _))) => {
                                panic!("{}: CTRL-C handler failed", sc2_bug_tag());
                            },
                        },
                    )
                )?;
            }
            Ok(())
        })
    }
}

impl ReplayCoordinator {
    #[async]
    fn run(mut self) -> Result<()> {
        let mut max_instances: Vec<Instance> =
            Vec::with_capacity(self.max_instances);

        #[async]
        for replay in self.replay_rx
            .map_err(|_| -> Error { unreachable!() })
        {
            match replay {
                Replay::LocalReplay(replay) => println!("{:?}", replay),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ReplayRequest {
}

/// Wrapper around a sender to provide a replay interface.
#[derive(Debug, Clone)]
pub struct ReplayClient {
    tx: mpsc::Sender<ReplayRequest>,
}

impl ReplayClient {}
