
use std::io;
use std::mem;
use std::path::{ PathBuf, MAIN_SEPARATOR };
use std::process;

use futures::sync::{ oneshot };
use glob::glob;
use regex::Regex;
use tokio_core::reactor;

use super::{ Result, Error };
use data::{ Rect, Player, PlayerKind, GameSettings };
use agent::{ Agent };
use instance::{ Instance, InstanceSettings, InstanceKind };
use participant::{ Participant, Control, Observer, Actions, AppState };

#[derive(Copy, Clone, PartialEq)]
enum ExeArch {
    X64,
    X32
}

#[derive(Clone)]
pub struct CoordinatorSettings {
    pub use_wine:           bool,
    pub dir:                Option<PathBuf>,
    pub port:               u16,
    pub window:             Rect<u32>,
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            use_wine:       false,
            dir:            None,
            port:           9168,
            window:         Rect::<u32> { x: 10, y: 10, w: 800, h: 600 }
        }
    }
}

pub struct Coordinator {
    use_wine:               bool,
    core:                   reactor::Core,
    exe:                    PathBuf,
    pwd:                    Option<PathBuf>,
    current_port:           u16,
    window:                 Rect<u32>,
    cleanups:               Vec<
        oneshot::Receiver<io::Result<process::ExitStatus>>
    >,
    participants:           Vec<Participant>
}

impl Coordinator {
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        let dir = match settings.dir {
            Some(dir) => dir,
            None => return Err(Error::ExeNotSpecified)
        };

        let (exe, arch) = select_exe(&dir, settings.use_wine)?;
        let pwd = select_pwd(&dir, arch);

        Ok(
            Self {
                use_wine:       settings.use_wine,
                core:           reactor::Core::new().unwrap(),
                exe:            exe,
                pwd:            pwd,
                current_port:   settings.port,
                window:         settings.window,
                cleanups:       vec![ ],
                participants:   vec![ ]
            }
        )
    }

    fn launch(&mut self) -> Result<Instance> {
        let instance = Instance::from_settings(
            InstanceSettings {
                kind: {
                    if self.use_wine {
                        InstanceKind::Wine
                    }
                    else {
                        InstanceKind::Native
                    }
                },
                exe: Some(self.exe.clone()),
                pwd: self.pwd.clone(),
                address: ("127.0.0.1".to_string(), self.current_port),
                window_rect: self.window
            }
        )?;

        self.current_port += 1;

        let cleanup = instance.start()?;

        self.cleanups.push(cleanup);

        Ok(instance)
    }

    pub fn start_game(
        &mut self,
        players: Vec<(Player, Option<Box<Agent>>)>,
        settings: GameSettings
    )
        -> Result<()>
    {
        let mut instances = vec![ ];
        let mut player_data = vec![ ];

        for &(player, _) in &players {
            match player.kind {
                PlayerKind::Computer => (),
                _ => {
                    instances.push(Some(self.launch()?));
                }
            };

            player_data.push(player);
        }

        if instances.len() < 1 {
            return Err(Error::Todo("expected at least one instance"))
        }

        let mut i = 0;
        for (player, agent) in players {
            match player.kind {
                PlayerKind::Computer => (),
                _ => match agent {
                    Some(agent) => {
                        let instance = mem::replace(&mut instances[i], None)
                            .unwrap()
                        ;

                        let client = instance.connect()?;

                        self.participants.push(
                            Participant::new(
                                instance,
                                client,
                                player,
                                agent
                            )
                        );

                        i += 1;
                    }
                    None => return Err(
                        Error::Todo(
                            "agent must be specified for non cpu player"
                        )
                    )
                }
            };
        }

        self.participants[0].create_game(&settings, &player_data)?;

        for ref mut p in &mut self.participants {
            p.join_game()?;
        }

        for ref mut p in &mut self.participants {
            match p.agent {
                Some(ref mut agent) => {
                    agent.on_game_full_start();
                },
                None => ()
            }
        }

        for ref mut p in &mut self.participants {
            match p.agent {
                Some(ref mut agent) => {
                    agent.on_game_start();
                },
                None => ()
            }
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.step_agents_realtime()?;
        Ok(())
    }

    fn step_agents_realtime(&mut self) -> Result<()> {
        let mut result = Ok(());

        for ref mut p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue;
            }

            if p.poll_leave_game() {
                continue;
            }

            if p.is_finished_game() {
                continue;
            }

            p.update_observation();
            p.issue_events();
            p.send_actions();

            if !p.is_in_game() {
                match p.agent {
                    Some(ref mut agent) => {
                        agent.on_game_end();
                    },
                    None => ()
                }
                match p.leave_game() {
                    Ok(()) => (),
                    Err(e) => {
                        result = Err(e);
                    }
                }
                continue;
            }
        }

        result
    }

    pub fn cleanup(&mut self) -> Result<()> {
        let cleanups = mem::replace(&mut self.cleanups, vec! [ ]);

        for ref mut p in &mut self.participants {
            match p.quit() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("unable to send quit: {}", e);
                }
            };

            match p.close() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("unable to close client: {}", e);
                }
            }
        };

        for cleanup in cleanups {
            match self.core.run(cleanup) {
                Ok(_) => (),
                Err(e) => eprintln!("unable to stop process {}", e)
            }
        }

        Ok(())
    }
}

fn select_exe(dir: &PathBuf, use_wine: bool) -> Result<(PathBuf, ExeArch)> {
    let separator = match MAIN_SEPARATOR {
        '\\' => "\\\\",
        '/' => "/",
        _ => panic!("unsupported path separator {}", MAIN_SEPARATOR)
    };

    let glob_iter = match glob(
        &format!(
            "{}/Versions/Base*/SC2*",
            dir.to_str().unwrap()
        )[..]
    ) {
        Ok(iter) => iter,
        Err(_) => return Err(Error::Todo("failed to read glob pattern"))
    };

    let exe_re = match Regex::new(
        &format!("Base([0-9]*){}SC2(_x64)?", separator)[..]
    ) {
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

                            if use_wine {
                                if arch == ExeArch::X32 {
                                    exe = Ok((path, arch));
                                }
                            }
                            else {
                                exe = Ok((path, arch));
                            }
                        }
                        else if current_version == v && !use_wine {
                            current_arch = match current_arch {
                                ExeArch::X64 => ExeArch::X64,
                                ExeArch::X32 => match arch {
                                    ExeArch::X64 => ExeArch::X64,
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
    let separator = match MAIN_SEPARATOR {
        '\\' => "\\\\",
        '/' => "/",
        _ => panic!("unsupported path separator {}", MAIN_SEPARATOR)
    };

    let support_dir = PathBuf::from(
        &format!(
            "{}{}Support{}",
            dir.to_str().unwrap(),
            separator,
            match arch {
                ExeArch::X64 => "64",
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
