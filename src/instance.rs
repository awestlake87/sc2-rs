
use std::path::PathBuf;
use std::thread;
use std::process;

use super::{ Result, Error };
use utils::Rect;

#[derive(Clone)]
pub struct InstanceSettings {
    pub starcraft_exe:      PathBuf,
    pub port:               u16,
    pub window_rect:        Rect<u32>
}

pub struct Instance {
    settings:           InstanceSettings,
    process_thread:     Option<thread::JoinHandle<()>>,
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Self {
        Self {
            settings: settings,
            process_thread: None
        }
    }

    pub fn launch(&mut self) -> Result<()> {
        let settings = self.settings.clone();

        if !settings.starcraft_exe.as_path().is_file() {
            return Err(Error::ExeDoesNotExist(settings.starcraft_exe))
        }

        self.process_thread = Some(
            thread::spawn(
                move || {
                    let window = settings.window_rect;

                    let mut child = process::Command::new(
                        settings.starcraft_exe
                    )
                        .arg("-listen").arg("127.0.0.1")
                        .arg("-port").arg(settings.port.to_string())
                        .arg("-displayMode").arg("0")

                        .arg("-windowx").arg(window.x.to_string())
                        .arg("-windowy").arg(window.y.to_string())
                        .arg("-windowWidth").arg(window.w.to_string())
                        .arg("-windowHeight").arg(window.h.to_string())

                        .spawn()
                        .unwrap()
                    ;


                    child.wait();
                }
            )
        );

        Ok(())
    }

    pub fn join(mut self) -> Result<()> {
        match self.process_thread.take() {
            Some(guard) => match guard.join() {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::UnableToStopInstance(e))
            },
            None => Ok(())
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        match self.process_thread {
            Some(_) => eprintln!("fuck! I forgot to join a sc2 instance"),
            None => ()
        }
    }
}
