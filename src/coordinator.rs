
use std::path::{ PathBuf };

use utils::Rect;
use agent::Agent;
use instance::{ Instance, InstanceSettings };

use super::{ Result, Error };

#[derive(Clone)]
pub struct CoordinatorSettings {
    pub starcraft_exe:      Option<PathBuf>,
    pub port:               u16,
    pub window_rect:        Rect<u32>
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            starcraft_exe: None,
            port: 9168,
            window_rect: Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    settings:           CoordinatorSettings,
    instance:           Instance,
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        // will probably add some auto-detect later
        let mut instance = match settings.starcraft_exe {
            Some(ref exe) => Instance::from_settings(
                InstanceSettings {
                    starcraft_exe: exe.clone(),
                    port: settings.port,
                    window_rect: settings.window_rect
                }
            ),
            None => return Err(Error::ExeNotSpecified)
        };

        instance.launch()?;

        Ok(Self { settings: settings, instance: instance })
    }
}
