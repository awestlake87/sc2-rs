
use std::io;
use std::process;
use std::path::{ PathBuf };

use futures::prelude::*;
use futures::sync::{ oneshot };
use tokio_core::reactor;

use super::{ Result, Error };
use utils::Rect;
use instance::{ Instance, InstanceSettings };

#[derive(Clone)]
pub struct CoordinatorSettings {
    pub reactor:            Option<reactor::Handle>,
    pub starcraft_exe:      Option<PathBuf>,
    pub port:               u16,
    pub window_rect:        Rect<u32>,
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            reactor:        None,
            starcraft_exe:  None,
            port:           9168,
            window_rect:    Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    instance:           Instance
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        let reactor = match settings.reactor {
            Some(reactor) => reactor,
            None => return Err(Error::ReactorNotSpecified)
        };
        // will probably add some auto-detect later
        let instance = match settings.starcraft_exe {
            Some(ref exe) => Instance::from_settings(
                InstanceSettings {
                    reactor: reactor,
                    starcraft_exe: exe.clone(),
                    port: settings.port,
                    window_rect: settings.window_rect
                }
            )?,
            None => return Err(Error::ExeNotSpecified)
        };

        Ok(Self { instance: instance })
    }

    #[async]
    fn launch(self)
        -> Result<Option<oneshot::Receiver<io::Result<process::ExitStatus>>>>
    {
        match self.instance.start() {
            Ok((cleanup, instance)) => {
                match await!(instance.connect()) {
                    Ok(client) => match await!(client.create_game()) {
                        Ok(client) => match await!(client.quit()) {
                            Ok(_) => Ok(Some(cleanup)),
                            Err(e) => Err(e)
                        },
                        Err(e) => Err(e)
                    },
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    #[async]
    pub fn run(self) -> Result<()> {
        match await!(self.launch()) {
            Ok(Some(cleanup)) => match await!(cleanup) {
                Ok(result) => match result {
                    Ok(status) => {
                        if status.success() {
                            Ok(())
                        }
                        else {
                            Err(Error::InstanceExitedWithError(status))
                        }
                    },
                    Err(e) => {
                        println!("error while cleaning up instance: {}", e);

                        Err(Error::UnableToStopInstance)
                    }
                },
                Err(_) => Err(Error::UnableToStopInstance)
            },
            Ok(None) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
