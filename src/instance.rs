
use std::path::PathBuf;
use std::io;
use std::time;
use std::thread;
use std::result;
use std::process;

use futures::prelude::*;
use futures::sync::{ oneshot };
use tokio_core::reactor;
use tokio_timer::Timer;
use url::Url;

use super::{ Result, Error };
use utils::Rect;
use client::{ Client };

#[derive(Clone)]
pub struct InstanceSettings {
    pub reactor:            reactor::Handle,
    pub starcraft_exe:      PathBuf,
    pub port:               u16,
    pub window_rect:        Rect<u32>
}

pub struct Instance {
    settings:           InstanceSettings,
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Result<Self> {
        if settings.starcraft_exe.as_path().is_file() {
            Ok(Self { settings: settings })
        }
        else {
            Err(Error::ExeDoesNotExist(settings.starcraft_exe))
        }
    }

    pub fn start(self)
        -> Result<(oneshot::Receiver<io::Result<process::ExitStatus>>, Self)>
    {
        let exe = self.settings.starcraft_exe.clone();
        let port = self.settings.port;
        let window = self.settings.window_rect;

        let (tx, rx) = oneshot::channel::<
            result::Result<process::ExitStatus, io::Error>
        >();

        thread::spawn(
            move || {
                let mut child = process::Command::new(exe)
                    .arg("-listen").arg("127.0.0.1")
                    .arg("-port").arg(port.to_string())
                    .arg("-displayMode").arg("0")

                    .arg("-windowx").arg(window.x.to_string())
                    .arg("-windowy").arg(window.y.to_string())
                    .arg("-windowWidth").arg(window.w.to_string())
                    .arg("-windowHeight").arg(window.h.to_string())

                    .spawn()
                    .unwrap()
                ;
                match tx.send(child.wait()) {
                    Ok(_) => (),
                    Err(e) => eprintln!(
                        "unable to send instance result: {:?}", e
                    )
                }
            }
        );

        Ok((rx, self))
    }

    #[async]
    pub fn connect(self) -> Result<Client> {
        let url = Url::parse(
            &format!("ws://localhost:{}/sc2api", self.settings.port)[..]
        ).expect("somehow I fucked up the URL");

        println!("attempting connection to {:?}", url);

        for i in 0..10 {
            match
                await!(
                    Client::connect(self.settings.reactor.clone(), url.clone())
                )
            {
                Ok(client) => return Ok(client),
                Err(_) => {
                    let timer = Timer::default();

                    match
                        await!(timer.sleep(time::Duration::from_millis(1000)))
                    {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("timeout failed: {}", e);
                        }
                    }
                }
            };

            println!("retrying {}...", i);
        };

        Err(Error::WebsockOpenFailed)
    }
}
