use ctrlc;
use futures::prelude::*;
use futures::sync;
use organelle::{Axon, Impulse, Soma};

use super::{Error, Result, Synapse};

/// soma that stops the organelle upon Ctrl-C
pub struct CtrlcBreakerSoma;

impl CtrlcBreakerSoma {
    /// create a new Ctrl-C breaker soma
    pub fn axon() -> Axon<Self> {
        Axon::new(Self {}, vec![], vec![])
    }
}

impl Soma for CtrlcBreakerSoma {
    type Synapse = Synapse;
    type Error = Error;
    type Future = Box<Future<Item = Self, Error = Self::Error>>;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::Start(tx, handle) => {
                let (sync_tx, sync_rx) = sync::mpsc::channel(1);

                ctrlc::set_handler(move || {
                    if let Err(e) = sync_tx.clone().send(()).wait() {
                        eprintln!("unable to send Ctrl-C signal {:?}", e);
                    }
                })?;

                handle.spawn(
                    sync_rx
                        .and_then(move |_| {
                            tx.clone().send(Impulse::Stop).map_err(|_| ())
                        })
                        .into_future()
                        .map(|_| ())
                        .map_err(|_| ()),
                );

                Ok(self)
            },

            _ => Ok(self),
        }
    }
}
