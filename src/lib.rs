#![warn(missing_docs)]
#![recursion_limit = "1024"]
#![feature(proc_macro, conservative_impl_trait, generators)]

//! StarCraft II API for Rust
//!
//! this API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api)

#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate ctrlc;
extern crate futures_await as futures;
extern crate glob;
extern crate nalgebra as na;
extern crate organelle;
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

mod agent;
mod client;
mod action;
mod computer;
mod ctrlc_breaker;
mod instance;
mod launcher;
mod melee;
mod observer;
mod synapses;

pub mod data;

pub use self::agent::{Agent, AgentBuilder, AgentControl, GameEvent, Player};
pub use self::computer::{Computer, ComputerBuilder};
pub use self::launcher::{Launcher, LauncherBuilder};
pub use self::melee::{Melee, MeleeBuilder, UpdateScheme};
pub use self::observer::Observation;

pub use self::action::ActionClient;
pub use self::agent::AgentClient;
pub use self::observer::ObserverClient;

use std::path::PathBuf;

error_chain! {
    links {
        Organelle(organelle::Error, organelle::ErrorKind) #[doc="organelle glue"];
    }
    foreign_links {
        Io(std::io::Error) #[doc="link io errors"];

        Ctrlc(ctrlc::Error) #[doc="link to Ctrl-C errors"];
        FutureCanceled(futures::Canceled) #[doc="link to futures"];
        UrlParse(url::ParseError) #[doc="link to url parse errors"];
        Protobuf(protobuf::ProtobufError) #[doc="link to protobuf errors"];
        Timer(tokio_timer::TimerError) #[doc="link to timer errors"];
        Tungstenite(tungstenite::Error) #[doc="link to tungstenite errors"];
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
        /// client failed to initiate close handshake
        ClientCloseFailed {
            description("unable to initiate close handshake")
            display("client close failed")
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

impl From<Error> for organelle::Error {
    fn from(e: Error) -> organelle::Error {
        organelle::Error::with_chain(e, organelle::ErrorKind::SomaError)
    }
}

trait FromProto<T>
where
    Self: Sized,
{
    /// convert from protobuf data
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
    /// convert into protobuf data
    fn into_proto(self) -> Result<T>;
}
