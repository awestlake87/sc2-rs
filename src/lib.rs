#![feature(conservative_impl_trait)]

extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

pub mod utils;

pub mod agent;
pub mod client;
pub mod coordinator;
mod instance;


use std::result;
use std::io;
use std::fmt;
use std::path::PathBuf;
use std::any::Any;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    ExeDoesNotExist(PathBuf),
    ExeNotSpecified,

    UnableToStartInstance(io::Error),
    UnableToStopInstance(Box<Any + Send + 'static>),

    WebsockOpenFailed,
    WebsockSendFailed,

    Todo(&'static str),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExeDoesNotExist(ref path) => write!(
                f, "starcraft exe {:?} does not exist", path
            ),
            Error::ExeNotSpecified => write!(
                f, "starcraft exe not specified"
            ),

            Error::UnableToStartInstance(ref e) => write!(
                f, "unable to launch instance {:?}", *e
            ),
            Error::UnableToStopInstance(ref e) => write!(
                f, "unable to stop instance {:?}", *e
            ),

            Error::WebsockOpenFailed => write!(f, "websocket open failed"),
            Error::WebsockSendFailed => write!(f, "websocket send failed"),

            Error::Todo(ref msg) => write!(f, "todo {:?}", *msg)
        }
    }
}
