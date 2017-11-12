#![warn(missing_docs)]
#![recursion_limit = "1024"]

//! StarCraft II API for Rust
//!
//! this API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api)

extern crate bytes;
#[macro_use]
extern crate error_chain;
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

pub mod colors;
pub mod data;

use std::path::PathBuf;
use std::rc::Rc;

pub use agent::{ Agent };
pub use coordinator::{ Coordinator, CoordinatorSettings };
pub use participant::{
    Participant,
    Actions,
    Control,
    Observation,
    Query,
    Replay,
    User,
    Debugging,
    DebugTextTarget,
    DebugCommand
};
pub use replay_observer::{ ReplayObserver };

use data::{ Unit, Upgrade };

error_chain! {
    errors {
        /// exe was not supplied to the coordinator
        ExeNotSpecified {
            description("exe not specified")
            display("StarCraft II exe was not specified")
        }
        /// exe supplied to the coordinator does not exist
        ExeDoesNotExist(exe: PathBuf) {
            description("exe file does not exist")
            display("StarCraft II exe does not exist at {:?}", exe)
        }

        /// client failed to open connection to the game instance
        ClientOpenFailed {
            description("unable to open connection to the game instance")
            display("client open failed")
        }
        /// client failed to send a message to the game instance
        ClientSendFailed {
            description("unable to send message to the game instance")
            display("client send failed")
        }
        /// client failed to receive a message from the game instance
        ClientRecvFailed {
            description("unable to receive message from game instance")
            display("client recv failed")
        }

        /// errors received from game instance
        GameErrors(errors: Vec<String>) {
            description("errors in game response")
            display("received errors: {:?}", errors)
        }
    }
}

trait GameEvents {
    fn on_game_full_start(&mut self);
    fn on_game_start(&mut self);
    fn on_game_end(&mut self);
    fn on_step(&mut self);
    fn on_unit_destroyed(&mut self, u: &Rc<Unit>);
    fn on_unit_created(&mut self, u: &Rc<Unit>);
    fn on_unit_idle(&mut self, u: &Rc<Unit>);
    fn on_upgrade_complete(&mut self, u: Upgrade);
    fn on_building_complete(&mut self, u: &Rc<Unit>);
    fn on_nydus_detected(&mut self);
    fn on_nuke_detected(&mut self);
    fn on_unit_detected(&mut self, u: &Rc<Unit>);
    fn should_ignore(&mut self) -> bool;
}
