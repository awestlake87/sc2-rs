#![warn(missing_docs)]

//! StarCraft II API for Rust
//!
//! this API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api)

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
mod instance;
mod participant;
mod replay_observer;

pub mod data;

use std::result;
use std::fmt;
use std::path::PathBuf;

pub use agent::{ Agent };
pub use coordinator::{ Coordinator, CoordinatorSettings };
pub use participant::{
    Participant, Actions, Control, Observation, Query, Replay, User
};
pub use replay_observer::{ ReplayObserver };

use data::{ Unit, Upgrade };

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

trait GameEvents {
    fn on_game_full_start(&mut self);
    fn on_game_start(&mut self);
    fn on_game_end(&mut self);
    fn on_step(&mut self);
    fn on_unit_destroyed(&mut self, u: &Unit);
    fn on_unit_created(&mut self, u: &Unit);
    fn on_unit_idle(&mut self, u: &Unit);
    fn on_upgrade_complete(&mut self, u: Upgrade);
    fn on_building_complete(&mut self, u: &Unit);
    fn on_nydus_detected(&mut self);
    fn on_nuke_detected(&mut self);
    fn on_unit_detected(&mut self, u: &Unit);
    fn should_ignore(&mut self) -> bool;
}
