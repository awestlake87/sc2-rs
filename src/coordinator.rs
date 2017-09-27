
use std::path::{ PathBuf };

use futures::{ Future };
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
            reactor: None,
            starcraft_exe: None,
            port: 9168,
            window_rect: Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    reactor:               reactor::Handle,
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
                    reactor: reactor.clone(),
                    starcraft_exe: exe.clone(),
                    port: settings.port,
                    window_rect: settings.window_rect
                }
            )?,
            None => return Err(Error::ExeNotSpecified)
        };

        Ok(
            Self {
                reactor: reactor,
                instance: instance,
            }
        )
    }

    pub fn run(&mut self) -> oneshot::Receiver<Result<()>> {
        let (tx, rx) = oneshot::channel::<Result<()>>();

        self.reactor.spawn(
            self.instance.run().then(
                move |result| match result {
                    Ok(run_result) => match run_result {
                        Ok(status) => {
                            if status.success() {
                                tx.send(Ok(()))
                            }
                            else {
                                tx.send(
                                    Err(
                                        Error::InstanceExitedWithError(
                                            status
                                        )
                                    )
                                )
                            }
                        },
                        Err(_) => tx.send(Err(Error::UnableToStopInstance))
                    }
                    Err(_) => tx.send(Err(Error::UnableToStartInstance))
                }
            ).then(
                |_| Ok(())
            )
        );

        self.reactor.spawn(
            self.instance.connect().then(
                |result| match result {
                    Ok(mut client) => {
                        client.quit()
                    }
                    Err(_) => Err(Error::WebsockSendFailed)
                }
            ).map_err(
                |e| eprintln!("client failed {}", e)
            )
        );

        rx
    }
}
