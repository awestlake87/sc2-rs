
use std::cmp;
use std::io;
use std::mem;
use std::path::{ PathBuf };
use std::process;

use futures::prelude::*;
use futures::sync::{ oneshot };
use glob::glob;
use regex::Regex;
use tokio_core::reactor;

use super::{ Result, Error };
use utils::Rect;
use client::{ Client };
use game::{ GameSettings };
use instance::{ Instance, InstanceSettings, InstanceKind };
use player::{ Player };

#[derive(Copy, Clone)]
enum ExeArch {
    X64,
    X32
}

#[derive(Clone)]
pub struct CoordinatorSettings {
    pub dir:                Option<PathBuf>,
    pub port:               u16,
    pub window:             Rect<u32>,
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            dir:            None,
            port:           9168,
            window:         Rect::<u32> { x: 120, y: 100, w: 1024, h: 768 }
        }
    }
}

pub struct Coordinator {
    core:                   Option<reactor::Core>,
    dir:                    PathBuf,
    exe:                    PathBuf,
    pwd:                    Option<PathBuf>,
    port:                   u16,
    window:                 Rect<u32>,
    cleanups:               Vec<
        oneshot::Receiver<io::Result<process::ExitStatus>>
    >,
    clients:                Vec<Option<Client>>
}

fn select_exe(dir: &PathBuf) -> Result<(PathBuf, ExeArch)> {
    let glob_iter = match glob(
        &format!(
            "{}/Versions/Base*/SC2*",
            dir.to_str().unwrap()
        )[..]
    ) {
        Ok(iter) => iter,
        Err(_) => return Err(Error::Todo("failed to read glob pattern"))
    };

    let exe_re = match Regex::new(".*Base([0-9]*)/SC2(.*)(\\.exe)?") {
        Ok(re) => re,
        Err(_) => return Err(Error::Todo("failed to parse regex"))
    };

    let mut current_version = 0;
    let mut current_arch = ExeArch::X32;
    let mut exe: Result<(PathBuf, ExeArch)> = Err(
        Error::Todo("exe not found")
    );

    for entry in glob_iter {
        match entry {
            Ok(path) => {
                let path_clone = path.clone();
                let path_str = match path_clone.to_str() {
                    Some(s) => s,
                    None => {
                        eprintln!("unable to convert path to string");
                        continue;
                    }
                };

                match exe_re.captures(&path_str[..]) {
                    Some(caps) => {
                        let v = match caps.get(1).unwrap().as_str().parse() {
                            Ok(v) => v,
                            Err(_) => {
                                eprintln!("unable to parse version as int");
                                continue;
                            }
                        };

                        let arch = match caps.get(2) {
                            Some(a) => match a.as_str() {
                                "_x64" => ExeArch::X64,
                                _ => {
                                    eprintln!("unrecognized suffix");
                                    continue;
                                }
                            },
                            None => ExeArch::X32
                        };

                        if current_version < v {
                            current_version = v;
                            exe = Ok((path, arch));
                        }
                        else if current_version == v {
                            current_arch = match current_arch {
                                ExeArch::X64 => ExeArch::X64,
                                ExeArch::X32 => match arch {
                                    ExeArch::X64 => ExeArch::X32,
                                    _ => ExeArch::X32
                                }
                            };

                            exe = Ok((path, current_arch));
                        };
                    }
                    _ => ()
                }
            }
            _ => ()
        };
    };

    exe
}

fn select_pwd(dir: &PathBuf, arch: ExeArch) -> Option<PathBuf> {
    let support_dir = PathBuf::from(
        &format!(
            "{}/Support{}",
            dir.to_str().unwrap(),
            match arch {
                ExeArch::X64 => "_x64",
                ExeArch::X32 => ""
            }
        )[..]
    );

    if support_dir.is_dir() {
        Some(support_dir)
    }
    else {
        None
    }
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        let dir = match settings.dir {
            Some(dir) => dir,
            None => return Err(Error::ExeNotSpecified)
        };

        let (exe, arch) = select_exe(&dir)?;
        let pwd = select_pwd(&dir, arch);

        Ok(
            Self {
                core: Some(reactor::Core::new().unwrap()),
                dir: dir,
                exe: exe,
                pwd: pwd,
                port: settings.port,
                window: settings.window,
                cleanups:       vec![ ],
                clients:        vec![ ]
            }
        )
    }

    pub fn start_instance(&mut self, instance: Instance) -> Result<Instance> {
        let (cleanup, instance) = instance.start()?;

        match cleanup {
            Some(cleanup) => {
                self.cleanups.push(cleanup);
            }
            _ => ()
        };

        Ok(instance)
    }

    pub fn launch(&mut self) -> Result<Instance> {
        let instance = Instance::from_settings(
            InstanceSettings {
                kind: InstanceKind::Local,
                reactor: match self.core {
                    Some(ref mut core) => core.handle(),
                    None => return Err(Error::Todo("jkasdfjsd"))
                },
                exe: Some(self.exe.clone()),
                pwd: self.pwd.clone(),
                address: ("127.0.0.1".to_string(), self.port),
                window_rect: self.window
            }
        )?;

        self.port += 1;
        self.start_instance(instance)
    }

    pub fn remote(&mut self, host: String, port: u16) -> Result<Instance> {
        let instance = Instance::from_settings(
            InstanceSettings {
                kind: InstanceKind::Remote,
                reactor: match self.core {
                    Some(ref mut core) => core.handle(),
                    None => return Err(Error::Todo("lalsldas"))
                },
                address: (host, port),
                exe: None,
                pwd: None,
                window_rect: self.window
            }
        )?;

        self.start_instance(instance)
    }

    pub fn start_game(&mut self, players: Vec<Player>, settings: GameSettings)
        -> Result<()>
    {
        let mut core = match mem::replace(&mut self.core, None) {
            Some(core) => core,
            None => return Err(Error::Todo("ajaksjdask"))
        };
        let mut clients = mem::replace(&mut self.clients, vec! [ ]);

        if players.len() < 1 {
            return Err(Error::Todo("expected at least one player"))
        }

        let start_game = async_block! {
            for player in players {
                let client = match await!(player.instance.connect()) {
                    Ok(client) => Some(client),
                    Err(e) => return Err(e)
                };

                clients.push(client);
            };

            Ok(clients)
        };

        match core.run(start_game) {
            Ok(clients) => {
                mem::replace(&mut self.core, Some(core));
                mem::replace(&mut self.clients, clients);

                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn cleanup(&mut self) -> Result<()> {
        let mut core = match mem::replace(&mut self.core, None) {
            Some(core) => core,
            None => return Err(Error::Todo("ajaksjdask"))
        };
        let clients = mem::replace(&mut self.clients, vec! [ ]);
        let cleanups = mem::replace(&mut self.cleanups, vec! [ ]);

        let cleanup = async_block! {
            for client in clients {
                match client {
                    Some(client) => match await!(client.quit()) {
                        Ok(_) => (),
                        Err(e) => return Err(e)
                    },
                    None => ()
                };
            };

            for cleanup in cleanups {
                match await!(cleanup) {
                    Ok(_) => (),
                    Err(e) => eprintln!("unable to stop process {}", e)
                }
            }

            Ok(())
        };

        core.run(cleanup)
    }
}
