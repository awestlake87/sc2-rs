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
mod launcher;
mod participant;
mod replay_observer;

pub mod colors;
pub mod data;

use std::path::PathBuf;
use std::rc::Rc;

pub use agent::{ Agent, Command, DebugTextTarget };
pub use coordinator::{ Coordinator, CoordinatorSettings };
pub use launcher::{ Launcher, LauncherSettings };
pub use participant::{
    Participant,
    Observation,
    Query,
    Replay,
    User,
    GameState,
    FrameData,
    GameEvent
};
pub use replay_observer::{ ReplayObserver };

use data::{ Unit, Upgrade };

error_chain! {
    foreign_links {
        Io(std::io::Error) #[doc="link io errors"];
    }
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
        /// an error occurred in agent callback
        AgentError {
            description("error occurred in agent callback")
            display("error occurred in agent callback")
        }

        /// invalid protobuf data from game instance
        InvalidProtobuf(msg: String) {
            description("unable to convert protobuf data to game data")
            display("unable to convert protobuf data: {}", msg)
        }
    }
}

trait GameEvents {
    fn on_game_full_start(&mut self) -> Result<()>;
    fn on_game_start(&mut self) -> Result<()>;
    fn on_game_end(&mut self) -> Result<()>;
    fn on_step(&mut self) -> Result<()>;
    fn on_unit_destroyed(&mut self, u: &Rc<Unit>) -> Result<()>;
    fn on_unit_created(&mut self, u: &Rc<Unit>) -> Result<()>;
    fn on_unit_idle(&mut self, u: &Rc<Unit>) -> Result<()>;
    fn on_upgrade_complete(&mut self, u: Upgrade) -> Result<()>;
    fn on_building_complete(&mut self, u: &Rc<Unit>) -> Result<()>;
    fn on_nydus_detected(&mut self) -> Result<()>;
    fn on_nuke_detected(&mut self) -> Result<()>;
    fn on_unit_detected(&mut self, u: &Rc<Unit>) -> Result<()>;
    fn should_ignore(&mut self) -> bool;
}

trait FromProto<T> where Self: Sized {
    /// convert from protobuf data
    fn from_proto(p: T) -> Result<Self>;
}

trait IntoSc2<T> {
    fn into_sc2(self) -> Result<T>;
}

impl<T, U> IntoSc2<U> for T where U: FromProto<T> {
    fn into_sc2(self) -> Result<U> {
        U::from_proto(self)
    }
}

trait IntoProto<T> {
    /// convert into protobuf data
    fn into_proto(self) -> Result<T>;
}
