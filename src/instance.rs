
use std::path::PathBuf;
use std::io;
use std::time;
use std::thread;
use std::process;

use futures::sync::{ oneshot };
use url::Url;

use super::{ Result, Error };
use utils::Rect;
use client::{ Client };

#[derive(Copy, Clone)]
pub enum InstanceKind {
    Native,
    Wine
}

#[derive(Clone)]
pub struct InstanceSettings {
    pub kind:               InstanceKind,
    pub exe:                Option<PathBuf>,
    pub pwd:                Option<PathBuf>,
    pub address:            (String, u16),
    pub window_rect:        Rect<u32>
}

pub struct Instance {
    kind:           InstanceKind,
    exe:            PathBuf,
    pwd:            Option<PathBuf>,
    address:        (String, u16),
    window_rect:    Rect<u32>
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Result<Self> {
        let exe = match settings.exe {
            Some(exe) => {
                if !exe.as_path().is_file() {
                    return Err(Error::ExeDoesNotExist(exe.clone()))
                }
                else {
                    exe
                }
            }
            None => return Err(Error::ExeNotSpecified)
        };

        Ok(
            Self {
                kind:           settings.kind,
                exe:            exe,
                pwd:            settings.pwd,
                address:        settings.address,
                window_rect:    settings.window_rect
            }
        )
    }

    pub fn start(&self)
        -> Result<oneshot::Receiver<io::Result<process::ExitStatus>>>
    {
        let kind = self.kind;
        let exe = self.exe.clone();
        let pwd = self.pwd.clone();
        let (_, port) = self.address;
        let window = self.window_rect;

        let (tx, rx) = oneshot::channel();

        thread::spawn(
            move || {
                let mut cmd = match kind {
                    InstanceKind::Native => process::Command::new(exe),
                    InstanceKind::Wine => {
                        let mut cmd = process::Command::new("wine");
                        cmd.arg(exe);
                        cmd
                    }
                };

                match pwd {
                    Some(pwd) => {
                        cmd.current_dir(pwd);
                    },
                    None => ()
                };

                cmd.arg("-listen").arg("127.0.0.1")
                    .arg("-port").arg(port.to_string())
                    .arg("-displayMode").arg("0")

                    .arg("-windowx").arg(window.x.to_string())
                    .arg("-windowy").arg(window.y.to_string())
                    .arg("-windowWidth").arg(window.w.to_string())
                    .arg("-windowHeight").arg(window.h.to_string())
                ;

                let mut child = cmd.spawn().unwrap();

                match tx.send(child.wait()) {
                    Ok(_) => (),
                    Err(e) => eprintln!(
                        "unable to send instance result: {:?}", e
                    )
                }
            }
        );

        Ok(rx)
    }

    pub fn connect(&self) -> Result<Client> {
        let (host, port) = self.address.clone();

        let url = Url::parse(
            &format!("ws://{}:{}/sc2api", host, port)[..]
        ).expect("somehow I fucked up the URL");

        println!("attempting connection to {:?}", url);

        for i in 0..10 {
            match Client::connect(url.clone()) {
                Ok(client) => return Ok(client),
                Err(_) => ()
            };
            thread::sleep(time::Duration::from_millis(3000));
            println!("retrying {}...", i);
        };

        Err(Error::WebsockOpenFailed)
    }
}
