//! Contains the public API of the structs that interact with the Observer.

use std::rc::Rc;

use futures::prelude::*;
use futures::unsync::oneshot;

use constants::sc2_bug_tag;
use data::{Unit, Upgrade};
use {Error, Result};

pub use services::observer_service::{Observation, ObserverClient};

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
///
/// This is simply a wrapper around a oneshot to simplify the acknowledgement.
/// It mainly exists because of the generics for oneshot errors, but adds some
/// clarity for its purpose.
#[derive(Debug)]
pub struct EventAck {
    tx: oneshot::Sender<()>,
}

impl EventAck {
    /// Wrap the underlying oneshot for an EventAck
    ///
    /// There may be external use-cases for creating EventAcks such as
    /// dispatching an event to a number of subscribers and then waiting until
    /// they are done using it by joining all of the oneshot receivers.
    pub fn wrap(tx: oneshot::Sender<()>) -> Self {
        Self { tx: tx }
    }
    /// Send a signal indicating that the user is done handling this event.
    #[async]
    pub fn done(self) -> Result<()> {
        self.tx.send(()).map_err(|_| -> Error {
            unreachable!("{}: Unable to ack event", sc2_bug_tag())
        })
    }
}
