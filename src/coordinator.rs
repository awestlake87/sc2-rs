
use std::mem;
use std::path::{ PathBuf, MAIN_SEPARATOR };

use glob::glob;
use regex::Regex;

use super::{ Result, Error, GameEvents };
use data::{ Rect, PlayerSetup, GameSettings, GamePorts, PortSet };
use instance::{ Instance, InstanceSettings, InstanceKind };
use participant::{
    Participant, AppState, User, Actions, Control, Observation, Replay
};

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

    pub replay_files:       Vec<PathBuf>,
    pub is_realtime:        bool,
    pub step_size:          usize,
}

impl Default for CoordinatorSettings {
    fn default() -> Self {
        Self {
            use_wine:       false,
            dir:            None,
            port:           9168,
            window:         Rect::<u32> { x: 10, y: 10, w: 800, h: 600 },

            replay_files:   vec![ ],
            is_realtime:    false,
            step_size:      1
        }
    }
}

pub struct Coordinator {
    use_wine:               bool,
    exe:                    PathBuf,
    pwd:                    Option<PathBuf>,
    current_port:           u16,
    window:                 Rect<u32>,
    participants:           Vec<Participant>,
    replay_observers:       Vec<Participant>,
    replay_files:           Vec<PathBuf>,
    players:                Vec<PlayerSetup>,
    ports:                  Option<GamePorts>,
    game_settings:          Option<GameSettings>,
    relaunched:             bool,
    is_realtime:            bool,
    step_size:              usize,
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
                use_wine:           settings.use_wine,
                exe:                exe,
                pwd:                pwd,
                current_port:       settings.port,
                window:             settings.window,
                participants:       vec![ ],
                replay_observers:   vec![ ],
                replay_files:       settings.replay_files,
                players:            vec![ ],
                ports:              None,
                game_settings:      None,
                relaunched:         false,
                is_realtime:        settings.is_realtime,
                step_size:          settings.step_size,
            }
        )
    }

    fn launch(&mut self) -> Result<Instance> {
        let mut instance = Instance::from_settings(
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

        instance.start()?;

        Ok(instance)
    }

    pub fn launch_starcraft(&mut self, players: Vec<(PlayerSetup, Option<User>)>) -> Result<()> {
        self.cleanup()?;

        let mut instances = vec![ ];

        for &(player, _) in &players {
            match player {
                PlayerSetup::Computer { .. } => (),
                _ => instances.push(Some(self.launch()?)),
            };

            self.players.push(player);
        }

        if instances.len() < 1 {
            return Err(Error::Todo("expected at least one instance"))
        }

        let mut i = 0;
        for (player, user) in players {
            match player {
                PlayerSetup::Computer { .. } => (),
                _ => {
                    let instance = mem::replace(&mut instances[i], None)
                        .unwrap()
                    ;

                    let client = instance.connect()?;

                    match player {
                        PlayerSetup::Observer => self.replay_observers.push(
                            Participant::new(
                                instance,
                                client,
                                player,
                                user
                            )
                        ),
                        PlayerSetup::Player { .. } => self.participants.push(
                            Participant::new(
                                instance,
                                client,
                                player,
                                user
                            )
                        ),
                        _ => panic!("rekt")
                    }

                    i += 1;
                }
            };
        }

        if self.participants.len() > 1 {
            let mut p = self.current_port;

            let mut ports = GamePorts {
                shared_port: p,
                server_ports: PortSet {
                    game_port: p + 1,
                    base_port: p + 2,
                },
                client_ports: vec![ ]
            };

            p += 3;

            for _ in 0..self.participants.len() {
                ports.client_ports.push(
                    PortSet {
                        game_port: p as u16,
                        base_port: p + 1 as u16,
                    }
                );

                p += 2;
            }

            self.ports = Some(ports);
            self.current_port = p;
        }

        Ok(())
    }

    pub fn start_game(&mut self, settings: GameSettings) -> Result<()> {
        assert!(self.participants.len() > 0);

        self.participants[0].create_game(
            &settings, &self.players, self.is_realtime
        )?;

        self.game_settings = Some(settings);

        for p in &mut self.participants {
            p.req_join_game(&self.ports)?;
        }

        for p in &mut self.participants {
            p.await_join_game()?;
            p.update_data()?;
        }

        for p in &mut self.participants {
            p.on_game_full_start();
        }

        for p in &mut self.participants {
            p.on_game_start();
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<bool> {
        if self.is_realtime {
            self.step_agents_realtime()?;
        }
        else {
            self.step_agents()?;
        }

        if !self.replay_observers.is_empty() {
            if self.is_realtime {
                unimplemented!("realtime replays");
            }
            else {
                self.step_replay_observers()?;
            }
        }

        Ok(!self.are_all_games_ended() || self.relaunched)
    }

    fn step_agents(&mut self) -> Result<()> {
        let mut result = Ok(());

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue;
            }

            match p.poll_leave_game() {
                Ok(true) => continue,
                _ => ()
            }

            if p.is_finished_game() {
                continue
            }

            match p.req_step(self.step_size) {
                Err(e) => {
                    eprintln!("step err: {}", e);
                    result = Err(e)
                },
                _ => ()
            }
        }

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue
            }

            // TODO: should it be awaiting steps if it's possible to skip reqs?
            match p.await_step() {
                Err(e) => {
                    eprintln!("await step err: {}", e);
                    result = Err(e)
                },
                _ => (),
            }

            if p.is_in_game() {
                match p.issue_events() {
                    Err(e) => {
                        eprintln!("issue events err: {}", e);
                        result = Err(e)
                    },
                    _ => ()
                }

                match p.send_actions() {
                    Err(e) => {
                        eprintln!("send actions err: {}", e);
                        result = Err(e)
                    },
                    _ => ()
                }

                /*TODO: match p.send_spatial_actions() {
                    Err(e) => result = Err(e),
                    _ => ()
                }*/
            }
            else {
                p.on_game_end();

                match p.leave_game() {
                    Err(e) => {
                        eprintln!("leave game err: {}", e);
                        result = Err(e)
                    },
                    _ => println!("leave game")
                }
            }
        }

        result
    }

    fn step_agents_realtime(&mut self) -> Result<()> {
        let mut result = Ok(());

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue;
            }

            match p.poll_leave_game() {
                Ok(true) => continue,
                _ => ()
            }

            if p.is_finished_game() {
                continue
            }

            match p.update_observation() {
                Err(e) => {
                    eprintln!("update observation err: {}", e);
                    result = Err(e)
                },
                _ => ()
            }
        }

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue
            }

            if p.is_in_game() {
                match p.issue_events() {
                    Err(e) => {
                        eprintln!("issue events err: {}", e);
                        result = Err(e)
                    },
                    _ => ()
                }
                match p.send_actions() {
                    Err(e) => {
                        eprintln!("send actions err: {}", e);
                        result = Err(e)
                    },
                    _ => ()
                }
                /*TODO: match p.send_spatial_actions() {
                    Err(e) => result = Err(e),
                    _ => ()
                }*/
            }
            else {
                p.on_game_end();

                match p.leave_game() {
                    Err(e) => {
                        eprintln!("leave game err: {}", e);
                        result = Err(e)
                    },
                    _ => println!("leave game")
                }
            }
        }

        result
    }

    fn start_replays(&mut self) -> Result<()> {
        let mut result = Ok(());

        for r in &mut self.replay_observers {
            let mut started = false;

            if !r.is_in_game() && r.is_ready_for_create_game() {
                let replay_files = mem::replace(
                    &mut self.replay_files, vec![ ]
                );

                for file in replay_files {
                    if !started {
                        match r.gather_replay_info(
                            &file.to_string_lossy(), true
                        ) {
                            Err(e) => result = Err(e),
                            _ => ()
                        }

                        started = {
                            if !r.should_ignore() {
                                match r.req_start_replay(
                                    &file.to_string_lossy()
                                ) {
                                    Err(e) => {
                                        result = Err(e);
                                        false
                                    },
                                    _ => true
                                }
                            }
                            else {
                                false
                            }
                        };
                        // TODO should relaunch

                        if !started {
                            self.replay_files.push(file);
                        }
                    }
                    else {
                        self.replay_files.push(file);
                    }
                }
            }
        }

        result
    }

    fn step_replay_observers(&mut self) -> Result<()> {
        let mut result = Ok(());

        for r in &mut self.replay_observers {
            if r.get_app_state() != AppState::Normal {
                continue
            }

            if r.has_response_pending() {
                if !r.poll() {
                    continue
                }

                match r.await_replay() {
                    Err(e) => result = Err(e),
                    _ => ()
                }

                match r.update_data() {
                    Err(e) => result = Err(e),
                    _ => ()
                }
            }

            if r.is_in_game() {
                match r.req_step(self.step_size) {
                    Err(e) => result = Err(e),
                    _ => ()
                }

                match r.await_step() {
                    Err(e) => result = Err(e),
                    _ => ()
                }

                if !r.is_in_game() {
                    r.on_game_end();
                }
            }
        }

        for r in &mut self.replay_observers {
            if r.get_app_state() != AppState::Normal {
                continue
            }

            match r.issue_events() {
                Err(e) => result = Err(e),
                _ => ()
            }
        }

        match result {
            Ok(_) => self.start_replays(),
            Err(e) => Err(e)
        }
    }

    fn are_all_games_ended(&self) -> bool {
        for p in self.participants.iter().chain(self.replay_observers.iter()) {
            if p.is_in_game() || p.has_response_pending() {
                return false
            }
        }

        true
    }

    pub fn cleanup(&mut self) -> Result<()> {
        for p in self.participants.iter_mut().chain(
            self.replay_observers.iter_mut()
        ) {
            match p.quit() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("unable to send quit: {}", e);
                }
            }

            match p.close() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("unable to close client: {}", e);
                }
            }

            match p.instance.kill() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("unable to terminate process: {}", e);
                }
            }
        }

        self.players.clear();
        self.participants.clear();
        self.replay_observers.clear();

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
