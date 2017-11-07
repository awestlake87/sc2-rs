#![warn(missing_docs)]

extern crate bytes;
extern crate futures;
extern crate glob;
extern crate nalgebra as na;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate sc2_proto;
extern crate tokio_core;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

mod agent;
mod client;
mod coordinator;
pub mod data;
mod instance;
mod participant;
mod replay_observer;

use std::result;
use std::fmt;
use std::path::PathBuf;

pub use agent::{ Agent };
pub use coordinator::{ Coordinator, CoordinatorSettings };
pub use participant::{
    Participant, Actions, Control, Observer, Query, Replay, User
};
pub use replay_observer::{ ReplayObserver };

/// type used for all results in the API
pub type Result<T> = result::Result<T, Error>;


/// type used for all errors in the API
#[derive(Debug)]
pub enum Error {
    /// the executable supplied to the coordinator does not exist
    ExeDoesNotExist(PathBuf),
    /// no executable was supplied to the coordinator
    ExeNotSpecified,

    /// failed to open the connection to the game instance
    WebsockOpenFailed,
    /// failed to send a message to the game instance
    WebsockSendFailed,
    /// failed to receive a message from the game instance
    WebsockRecvFailed,

    /// an error variant has not been created for this situation yet
    ///
    /// this should not be used in a release, it's just a placeholder until we
    /// figure out a good system for error handling
    Todo(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExeDoesNotExist(ref path) => write!(
                f, "starcraft exe {:?} does not exist", path
            ),
            Error::ExeNotSpecified => write!(
                f, "starcraft exe not specified"
            ),

            Error::WebsockOpenFailed => write!(f, "websocket open failed"),
            Error::WebsockSendFailed => write!(f, "websocket send failed"),
            Error::WebsockRecvFailed => write!(f, "websocket recv failed"),

            Error::Todo(ref msg) => write!(f, "todo {:?}", *msg)
        }
    }
}
