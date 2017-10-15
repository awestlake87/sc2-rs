#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate bytes;
extern crate futures_await as futures;
extern crate glob;
extern crate nalgebra as na;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate sc2_proto;
extern crate tokio_core;
extern crate tungstenite;
extern crate url;

pub mod agent;
pub mod client;
pub mod coordinator;
pub mod data;
mod instance;
pub mod participant;
pub mod utils;

use std::result;
use std::fmt;
use std::path::PathBuf;
use std::process;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ExeDoesNotExist(PathBuf),
    ExeNotSpecified,
    ReactorNotSpecified,

    UnableToStartInstance,
    UnableToStopInstance,
    InstanceExitedWithError(process::ExitStatus),

    WebsockOpenFailed,
    WebsockSendFailed,
    WebsockRecvFailed,

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
            Error::ReactorNotSpecified => write!(
                f, "reactor not specified"
            ),

            Error::UnableToStartInstance => write!(
                f, "unable to start instance"
            ),
            Error::UnableToStopInstance => write!(
                f, "unable to stop instance"
            ),
            Error::InstanceExitedWithError(status) => match status.code() {
                Some(code) => write!(
                    f, "instance exited with status: {}", code
                ),
                None => write!(f, "instance exited with error")
            },

            Error::WebsockOpenFailed => write!(f, "websocket open failed"),
            Error::WebsockSendFailed => write!(f, "websocket send failed"),
            Error::WebsockRecvFailed => write!(f, "websocket recv failed"),

            Error::Todo(ref msg) => write!(f, "todo {:?}", *msg)
        }
    }
}
