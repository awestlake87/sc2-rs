
use std::result;
use std::io;
use std::fmt;
use std::path::PathBuf;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    ExeDoesNotExist(Option<PathBuf>),
    ExeNotSpecified,

    UnableToStartProcess(io::Error),

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

            Error::UnableToStartProcess(ref err) => write!(
                f, "unable to start process {:?}", *err
            ),

            Error::WebsockSendFailed => write!(f, "websocket send failed"),

            Error::Todo(ref msg) => write!(f, "todo {:?}", *msg)
        }
    }
}
