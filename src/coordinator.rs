
use std::path::{ PathBuf };

use futures::{ Future };
use tokio_core::reactor;

use super::{ Result, Error };
use utils::Rect;
use agent::Agent;
use client::{ Client };
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
            reactor: None,
            starcraft_exe: None,
            port: 9168,
            window_rect: Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    core:               reactor::Core,
    instance:           Instance,
    client:             Option<Client>,
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        let core = reactor::Core::new().unwrap();

        // will probably add some auto-detect later
        let instance = match settings.starcraft_exe {
            Some(ref exe) => Instance::from_settings(
                InstanceSettings {
                    reactor: core.handle(),
                    starcraft_exe: exe.clone(),
                    port: settings.port,
                    window_rect: settings.window_rect
                }
            )?,
            None => return Err(Error::ExeNotSpecified)
        };

        Ok(
            Self {
                core: core,
                instance: instance,
                client: None,
            }
        )
    }

    pub fn run(&mut self) {
        self.core.run(self.instance.run()).unwrap();
        /*match self.core {
            Some(core) => {
                core.run(self.instance.run())
                Ok()
            },
            None => self.instance.run()
        }*/
    }
}
