use std::path::PathBuf;
use std::process;

use url::Url;

use super::{ErrorKind, PortSet, Rect, Result};

#[derive(Copy, Clone)]
pub enum InstanceKind {
    Native,
    Wine,
}

#[derive(Clone)]
pub struct InstanceSettings {
    pub kind: InstanceKind,
    pub exe: Option<PathBuf>,
    pub pwd: Option<PathBuf>,
    pub address: (String, u16),
    pub window_rect: Rect<u32>,
    pub ports: PortSet,
}

pub struct Instance {
    kind: InstanceKind,
    exe: PathBuf,
    pwd: Option<PathBuf>,
    address: (String, u16),
    window_rect: Rect<u32>,
    child: Option<process::Child>,
    pub ports: PortSet,
}

impl Instance {
    pub fn from_settings(settings: InstanceSettings) -> Result<Self> {
        let exe = match settings.exe {
            Some(exe) => {
                if !exe.as_path().is_file() {
                    bail!(ErrorKind::ExeDoesNotExist(exe.clone()))
                } else {
                    exe
                }
            },
            None => bail!(ErrorKind::ExeNotSpecified),
        };

        Ok(Self {
            kind: settings.kind,
            exe: exe,
            pwd: settings.pwd,
            address: settings.address,
            window_rect: settings.window_rect,
            child: None,
            ports: settings.ports,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        let kind = self.kind;
        let exe = self.exe.clone();
        let pwd = self.pwd.clone();
        let (_, port) = self.address;
        let window = self.window_rect;

        let mut cmd = match kind {
            InstanceKind::Native => process::Command::new(exe),
            InstanceKind::Wine => {
                let mut cmd = process::Command::new("wine");
                cmd.arg(exe);
                cmd
            },
        };

        if let Some(pwd) = pwd {
            cmd.current_dir(pwd);
        }

        cmd.arg("-listen")
            .arg("127.0.0.1")
            .arg("-port")
            .arg(port.to_string())
            .arg("-displayMode")
            .arg("0")
            .arg("-windowx")
            .arg(window.x.to_string())
            .arg("-windowy")
            .arg(window.y.to_string())
            .arg("-windowWidth")
            .arg(window.w.to_string())
            .arg("-windowHeight")
            .arg(window.h.to_string());

        self.child = Some(cmd.spawn()?);

        Ok(())
    }

    pub fn get_url(&self) -> Result<Url> {
        let (host, port) = self.address.clone();
        Ok(Url::parse(&*format!("ws://{}:{}/sc2api", host, port))?)
    }

    pub fn kill(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.child {
            child.kill()?;
        }

        self.child = None;

        Ok(())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Err(e) = self.kill() {
            eprintln!("unable to drop instance {:?}", e);
        }
    }
}
