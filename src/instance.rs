
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

#[derive(Copy, Clone)]
pub enum InstanceKind {
    Remote,
    Local
}

#[derive(Clone)]
pub struct InstanceSettings {
    pub kind:               InstanceKind,
    pub reactor:            reactor::Handle,
    pub exe:                Option<PathBuf>,
    pub pwd:                Option<PathBuf>,
    pub address:            (String, u16),
    pub window_rect:        Rect<u32>
}

pub struct Instance {
    settings:               InstanceSettings
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Result<Self> {
        match settings.kind {
            InstanceKind::Local => {
                match settings.exe {
                    Some(ref exe) => {
                        if !exe.as_path().is_file() {
                            return Err(Error::ExeDoesNotExist(exe.clone()))
                        }
                    }
                    None => return Err(Error::ExeNotSpecified)
                }

                Ok(Self { settings: settings })
            },
            InstanceKind::Remote => Ok(Self { settings: settings })
        }
    }

    pub fn start(self)
        -> Result<
            (Option<oneshot::Receiver<io::Result<process::ExitStatus>>>, Self)
        >
    {
        match self.settings.kind {
            InstanceKind::Remote => return Ok((None, self)),
            _ => ()
        }

        let exe = self.settings.exe.clone().unwrap().clone();
        let pwd = self.settings.pwd.clone();
        let (_, port) = self.settings.address;
        let window = self.settings.window_rect;

        let (tx, rx) = oneshot::channel::<
            result::Result<process::ExitStatus, io::Error>
        >();

        thread::spawn(
            move || {
                let mut child = match pwd {
                    Some(pwd) => process::Command::new(exe)
                        .arg("-listen").arg("127.0.0.1")
                        .arg("-port").arg(port.to_string())
                        .arg("-displayMode").arg("0")

                        .arg("-windowx").arg(window.x.to_string())
                        .arg("-windowy").arg(window.y.to_string())
                        .arg("-windowWidth").arg(window.w.to_string())
                        .arg("-windowHeight").arg(window.h.to_string())

                        .current_dir(pwd)

                        .spawn()
                        .unwrap(),
                    None => process::Command::new(exe)
                        .arg("-listen").arg("127.0.0.1")
                        .arg("-port").arg(port.to_string())
                        .arg("-displayMode").arg("0")

                        .arg("-windowx").arg(window.x.to_string())
                        .arg("-windowy").arg(window.y.to_string())
                        .arg("-windowWidth").arg(window.w.to_string())
                        .arg("-windowHeight").arg(window.h.to_string())

                        .spawn()
                        .unwrap()
                };

                match tx.send(child.wait()) {
                    Ok(_) => (),
                    Err(e) => eprintln!(
                        "unable to send instance result: {:?}", e
                    )
                }
            }
        );

        Ok((Some(rx), self))
    }

    #[async]
    pub fn connect(self) -> Result<Client> {
        let (host, port) = self.settings.address;

        let url = Url::parse(
            &format!("ws://{}:{}/sc2api", host, port)[..]
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
