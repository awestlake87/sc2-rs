#![warn(missing_docs)]
#![recursion_limit = "1024"]
#![feature(proc_macro, generators)]

//! StarCraft II API for Rust
//!
//! This API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api).

#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate ctrlc;
extern crate futures_await as futures;
extern crate glob;
extern crate nalgebra as na;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate sc2_proto;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;
extern crate uuid;

mod action;
mod agent;
mod client;
mod computer;
mod instance;
mod launcher;
mod melee;
mod observer;

pub mod data;

pub use self::action::{ActionClient, DebugClient};
pub use self::agent::{AgentBuilder, Event, EventAck};
pub use self::computer::ComputerBuilder;
pub use self::launcher::LauncherSettings;
pub use self::melee::MeleeBuilder;
pub use self::observer::{Observation, ObserverClient};

use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(std::io::Error) #[doc="Link io errors."];

        Ctrlc(ctrlc::Error) #[doc="Link to Ctrl-C errors."];
        FutureCanceled(futures::Canceled) #[doc="Link to futures."];
        UrlParse(url::ParseError) #[doc="Link to url parse errors."];
        Protobuf(protobuf::ProtobufError) #[doc="Link to protobuf errors."];
        Timer(tokio_timer::TimerError) #[doc="Link to timer errors."];
        Tungstenite(tungstenite::Error) #[doc="Link to tungstenite errors."];
    }
    errors {
        /// Executable was not supplied to the coordinator.
        ExeNotSpecified {
            description("Executable was not supplied to the coordinator")
            display("StarCraft II exe was not specified")
        }
        /// Executable supplied to the coordinator does not exist.
        ExeDoesNotExist(exe: PathBuf) {
            description("Executable supplied to the coordinator does not exist")
            display("StarCraft II exe does not exist at {:?}", exe)
        }

        /// Client failed to open connection to the game instance.
        ClientOpenFailed(msg: String) {
            description("Client failed to open connection to the game instance")
            display("Client open failed - {}", msg)
        }
        /// Client failed to send a message to the game instance.
        ClientSendFailed(msg: String) {
            description("Client failed to send a message to the game instance")
            display("Client send failed - {}", msg)
        }
        /// Client failed to receive a message from the game instance.
        ClientRecvFailed(msg: String) {
            description("Client failed to receive a message from the game instance")
            display("Client recv failed - {}", msg)
        }
        /// Client failed to initiate close handshake.
        ClientCloseFailed(msg: String) {
            description("Client failed to complete close handshake")
            display("Client close failed - {}", msg)
        }

        /// Errors received from game instance.
        GameErrors(errors: Vec<String>) {
            description("Errors received from game instance")
            display("Received errors: {:?}", errors)
        }

        /// Invalid protobuf data from game instance.
        InvalidProtobuf(msg: String) {
            description("Invalid protobuf data from game instance")
            display("Unable to convert protobuf data: {}", msg)
        }
    }
}

trait FromProto<T>
where
    Self: Sized,
{
    /// Convert from protobuf data.
    fn from_proto(p: T) -> Result<Self>;
}

trait IntoSc2<T> {
    fn into_sc2(self) -> Result<T>;
}

impl<T, U> IntoSc2<U> for T
where
    U: FromProto<T>,
{
    fn into_sc2(self) -> Result<U> {
        U::from_proto(self)
    }
}

trait IntoProto<T> {
    /// Convert into protobuf data
    fn into_proto(self) -> Result<T>;
}
