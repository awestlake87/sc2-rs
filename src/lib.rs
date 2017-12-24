#![warn(missing_docs)]
#![recursion_limit = "1024"]

//! StarCraft II API for Rust
//!
//! this API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api)

#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate cortical;
extern crate ctrlc;
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
extern crate uuid;

mod agent;
mod client;
mod coordinator;
mod instance;
mod frame;
mod launcher;
mod lobes;
mod participant;
mod replay_observer;

pub mod colors;
pub mod data;

use std::path::PathBuf;

use futures::prelude::*;

pub use agent::{ Agent };
pub use coordinator::{ Coordinator, CoordinatorSettings };
pub use frame::{ Command, FrameData, GameEvent, DebugTextTarget };
pub use launcher::{ Launcher, LauncherSettings };
pub use lobes::{ Message, MeleeLobe, MeleeSettings, MeleeSuite };
pub use participant::{ User };
pub use replay_observer::{ ReplayObserver };

error_chain! {
    links {
        Cortical(cortical::Error, cortical::ErrorKind) #[doc="cortical glue"];
    }
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
