use std::env::home_dir;
use std::mem;
use std::path::{PathBuf, MAIN_SEPARATOR};

use futures::prelude::*;
use futures::unsync;
use glob::glob;
use organelle::{Axon, Constraint, Impulse, Soma};
use regex::Regex;

use super::{Error, ErrorKind, Result};
use data::Rect;
use instance::{Instance, InstanceKind, InstanceSettings};
use synapses::{Dendrite, Synapse};

/// endpoint port settings
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub struct PortSet {
    pub game_port: u16,
    pub base_port: u16,
}

/// all port settings for a game
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct GamePorts {
    pub shared_port: u16,
    pub server_ports: PortSet,
    pub client_ports: Vec<PortSet>,
}

/// sender for launcher
#[derive(Debug, Clone)]
pub struct LauncherTerminal {
    tx: unsync::mpsc::Sender<LauncherRequest>,
}

impl LauncherTerminal {
    fn new(tx: unsync::mpsc::Sender<LauncherRequest>) -> Self {
        Self { tx: tx }
    }

    /// launch an instance
    #[async]
    pub fn launch(self) -> Result<Instance> {
        let (tx, rx) = unsync::oneshot::channel();

        await!(
            self.tx
                .send(LauncherRequest::Launch(tx))
                .map_err(|_| Error::from("unable to send launch request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to receive instance")))
    }

    /// get a set of game ports
    #[async]
    pub fn get_game_ports(self) -> Result<GamePorts> {
        let (tx, rx) = unsync::oneshot::channel();

        await!(
            self.tx
                .send(LauncherRequest::Ports(tx))
                .map_err(|_| Error::from("unable to send ports request"))
        )?;

        await!(rx.map_err(|_| Error::from("unable to receive ports")))
    }
}

#[derive(Debug)]
enum LauncherRequest {
    Launch(unsync::oneshot::Sender<Instance>),
    Ports(unsync::oneshot::Sender<GamePorts>),
}

/// receiver for launcher
#[derive(Debug)]
pub struct LauncherDendrite {
    rx: unsync::mpsc::Receiver<LauncherRequest>,
}

impl LauncherDendrite {
    fn new(rx: unsync::mpsc::Receiver<LauncherRequest>) -> Self {
        Self { rx: rx }
    }
}

/// create a launcher synapse
pub fn synapse() -> (LauncherTerminal, LauncherDendrite) {
    let (tx, rx) = unsync::mpsc::channel(1);

    (LauncherTerminal::new(tx), LauncherDendrite::new(rx))
}

/// launches game instances upon request
pub struct Launcher {
    exe: PathBuf,
    pwd: Option<PathBuf>,
    current_port: u16,
    use_wine: bool,
}

/// builder used to create launcher
pub struct LauncherBuilder {
    dir: Option<PathBuf>,
    use_wine: bool,
    base_port: u16,
}

impl LauncherBuilder {
    /// create a new builder
    pub fn new() -> Self {
        Self {
            dir: None,
            use_wine: false,
            base_port: 9168,
        }
    }

    /// installation directory
    ///
    /// auto-detect if not specified
    pub fn install_dir(self, dir: PathBuf) -> Self {
        Self {
            dir: Some(dir),
            ..self
        }
    }

    /// use Wine to run the game - for unix users
    pub fn use_wine(self, flag: bool) -> Self {
        Self {
            use_wine: flag,
            ..self
        }
    }

    /// starting point for game ports
    pub fn base_port(self, port: u16) -> Self {
        Self {
            base_port: port,
            ..self
        }
    }

    /// build the settings object
    pub fn create(self) -> Result<Launcher> {
        let dir = {
            if let Some(dir) = self.dir {
                dir
            } else {
                auto_detect_starcraft(self.use_wine)?
            }
        };
        let (exe, arch) = select_exe(&dir, self.use_wine)?;
        let pwd = select_pwd(&dir, arch);

        Ok(Launcher {
            exe: exe,
            pwd: pwd,
            current_port: self.base_port,
            use_wine: self.use_wine,
        })
    }
}

impl Launcher {
    fn launch(&mut self) -> Result<Instance> {
        let mut instance = Instance::from_settings(InstanceSettings {
            kind: {
                if self.use_wine {
                    InstanceKind::Wine
                } else {
                    InstanceKind::Native
                }
            },
            exe: Some(self.exe.clone()),
            pwd: self.pwd.clone(),
            address: ("127.0.0.1".into(), self.current_port),
            window_rect: Rect::<u32> {
                x: 10,
                y: 10,
                w: 1024,
                h: 768,
            },
            ports: PortSet {
                game_port: self.current_port + 1,
                base_port: self.current_port + 2,
            },
        })?;

        self.current_port += 3;

        instance.start()?;

        Ok(instance)
    }

    #[async]
    fn listen(mut self, dendrite: LauncherDendrite) -> Result<()> {
        #[async]
        for req in dendrite.rx.map_err(|_| Error::from("streams can't fail")) {
            match req {
                LauncherRequest::Launch(tx) => {
                    tx.send(self.launch()?)
                        .map_err(|_| Error::from("unable to send instance"))?;
                },
                LauncherRequest::Ports(tx) => {
                    tx.send(self.create_game_ports())
                        .map_err(|_| Error::from("unable to send game ports"))?;
                },
            }
        }

        Ok(())
    }

    /// create a set of ports for multiplayer games
    pub fn create_game_ports(&mut self) -> GamePorts {
        let ports = GamePorts {
            shared_port: self.current_port,
            server_ports: PortSet {
                game_port: self.current_port + 1,
                base_port: self.current_port + 2,
            },
            client_ports: vec![],
        };

        self.current_port += 3;

        ports
    }
}

/// soma in charge of launching game instances and assigning ports
pub struct LauncherSoma {
    launcher: Option<Launcher>,
    dendrite: Option<LauncherDendrite>,
}

impl LauncherSoma {
    /// create a launcher from settings
    pub fn axon(launcher: Launcher) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                launcher: Some(launcher),
                dendrite: None,
            },
            vec![Constraint::One(Synapse::Launcher)],
            vec![],
        ))
    }
}

impl Soma for LauncherSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, msg: Impulse<Self::Synapse>) -> Result<Self> {
        match msg {
            Impulse::AddDendrite(
                _,
                Synapse::Launcher,
                Dendrite::Launcher(dendrite),
            ) => {
                self.dendrite = Some(dendrite);

                Ok(self)
            },
            Impulse::Start(_, tx, handle) => {
                assert!(self.launcher.is_some());
                assert!(self.dendrite.is_some());

                handle.spawn(
                    mem::replace(&mut self.launcher, None)
                        .unwrap()
                        .listen(mem::replace(&mut self.dendrite, None).unwrap())
                        .or_else(move |e| {
                            tx.send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(self)
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ExeArch {
    X64,
    X32,
}

fn auto_detect_starcraft(use_wine: bool) -> Result<PathBuf> {
    if cfg!(windows) {
        let path_x86 = PathBuf::from("C:\\Program Files (x86)\\StarCraft II");
        let path = PathBuf::from("C:\\Program Files\\StarCraft II");

        if path_x86.is_dir() {
            Ok(path_x86)
        } else if path.is_dir() {
            Ok(path)
        } else {
            bail!(ErrorKind::ExeNotSpecified)
        }
    } else if use_wine {
        if let Some(home) = home_dir() {
            let path_x86 =
                home.join(".wine/drive_c/Program Files (x86)/StarCraft II");
            let path = home.join(".wine/drive_c/Program Files/StarCraft II");

            if path_x86.is_dir() {
                Ok(path_x86)
            } else if path.is_dir() {
                Ok(path)
            } else {
                bail!(ErrorKind::ExeNotSpecified)
            }
        } else {
            bail!(ErrorKind::ExeNotSpecified)
        }
    } else {
        bail!(ErrorKind::ExeNotSpecified)
    }
}

fn select_exe(dir: &PathBuf, use_wine: bool) -> Result<(PathBuf, ExeArch)> {
    if cfg!(target_os = "windows") && use_wine {
        bail!("wine not supported on windows")
    }

    let separator = match MAIN_SEPARATOR {
        '\\' => "\\\\",
        '/' => "/",
        _ => panic!("unsupported path separator {}", MAIN_SEPARATOR),
    };

    let glob_iter = match glob(
        &format!("{}/Versions/Base*/SC2*", dir.to_str().unwrap())[..],
    ) {
        Ok(iter) => iter,
        Err(_) => bail!("failed to read glob pattern"),
    };

    let exe_re =
        match Regex::new(&format!("Base([0-9]*){}SC2(_x64)?", separator)[..]) {
            Ok(re) => re,
            Err(_) => bail!("failed to parse regex"),
        };

    let mut current_version = 0;
    let mut current_arch = ExeArch::X32;
    let mut exe: Result<(PathBuf, ExeArch)> = Err("exe not found".into());

    for entry in glob_iter {
        match entry {
            Ok(path) => {
                let path_clone = path.clone();
                let path_str = match path_clone.to_str() {
                    Some(s) => s,
                    None => {
                        eprintln!("unable to convert path to string");
                        continue;
                    },
                };

                match exe_re.captures(&path_str[..]) {
                    Some(caps) => {
                        let v = match caps.get(1).unwrap().as_str().parse() {
                            Ok(v) => v,
                            Err(_) => {
                                eprintln!("unable to parse version as int");
                                continue;
                            },
                        };

                        let arch = match caps.get(2) {
                            Some(a) => match a.as_str() {
                                "_x64" => ExeArch::X64,
                                _ => {
                                    eprintln!("unrecognized suffix");
                                    continue;
                                },
                            },
                            None => ExeArch::X32,
                        };

                        if current_version < v {
                            current_version = v;

                            if use_wine {
                                if arch == ExeArch::X32 {
                                    exe = Ok((path, arch));
                                }
                            } else {
                                exe = Ok((path, arch));
                            }
                        } else if current_version == v && !use_wine {
                            current_arch = match current_arch {
                                ExeArch::X64 => ExeArch::X64,
                                ExeArch::X32 => match arch {
                                    ExeArch::X64 => ExeArch::X64,
                                    _ => ExeArch::X32,
                                },
                            };

                            exe = Ok((path, current_arch));
                        };
                    },
                    _ => (),
                }
            },
            _ => (),
        };
    }

    exe
}

fn select_pwd(dir: &PathBuf, arch: ExeArch) -> Option<PathBuf> {
    let separator = match MAIN_SEPARATOR {
        '\\' => "\\\\",
        '/' => "/",
        _ => panic!("unsupported path separator {}", MAIN_SEPARATOR),
    };

    let support_dir = PathBuf::from(
        &format!(
            "{}{}Support{}",
            dir.to_str().unwrap(),
            separator,
            match arch {
                ExeArch::X64 => "64",
                ExeArch::X32 => "",
            }
        )[..],
    );

    if support_dir.is_dir() {
        Some(support_dir)
    } else {
        None
    }
}
