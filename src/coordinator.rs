
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
    pub exe:                Option<PathBuf>,
    pub port:               u16,
    pub window:             Rect<u32>,
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            reactor:        None,
            exe:            None,
            port:           9168,
            window:         Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    reactor:                reactor::Handle,
    exe:                    PathBuf,
    port:                   u16,
    window:                 Rect<u32>,
    cleanups:               Vec<
        oneshot::Receiver<io::Result<process::ExitStatus>>
    >
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        let reactor = match settings.reactor {
            Some(reactor) => reactor,
            None => return Err(Error::ReactorNotSpecified)
        };

        let exe = match settings.exe {
            Some(exe) => exe,
            None => return Err(Error::ExeNotSpecified)
        };

        Ok(
            Self {
                reactor: reactor,
                exe: exe,
                port: settings.port,
                window: settings.window,
                cleanups:       vec![ ]
            }
        )
    }

    pub fn launch(&mut self) -> Result<Instance> {
        let instance = Instance::from_settings(
            InstanceSettings {
                reactor: self.reactor.clone(),
                starcraft_exe: self.exe.clone(),
                port: self.port,
                window_rect: self.window
            }
        )?;

        match instance.start() {
            Ok((cleanup, instance)) => {
                self.port += 1;
                self.cleanups.push(cleanup);
                Ok(instance)
            },
            Err(e) => Err(e)
        }
    }

    #[async]
    pub fn cleanup(self) -> Result<()> {
        for cleanup in self.cleanups {
            match await!(cleanup) {
                Ok(_) => (),
                Err(e) => eprintln!("unable to stop process {}", e)
            }
        }

        Ok(())
    }
}
