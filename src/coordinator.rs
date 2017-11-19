
use std::mem;
use std::path::{ PathBuf };

use super::{ Result, ResultExt, ErrorKind };
use data::{ PlayerSetup, GameSettings, GamePorts };
use launcher::{ Launcher };
use participant::{
    Participant,
    AppState,
    User
};

/// settings for the coordinator
pub struct CoordinatorSettings {
    /// object in charge of launching game instances
    pub launcher:           Launcher,

    /// a list of replay files to distribute amongst the replay observers
    pub replay_files:       Vec<PathBuf>,
    /// whether the game is stepped in realtime
    pub is_realtime:        bool,
    /// number of steps to request each time in a non-realtime game
    pub step_size:          usize,
}

/// central struct in charge of managing participants, games, and replays
pub struct Coordinator {
    launcher:               Launcher,

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
    /// construct a coordinator from settings
    pub fn from_settings(settings: CoordinatorSettings) -> Result<Self> {
        Ok(
            Self {
                launcher:           settings.launcher,
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

    /// launch any game instances necessary for the following players
    pub fn launch_starcraft(
        &mut self, players: Vec<(PlayerSetup, Option<User>)>
    )
        -> Result<()>
    {
        self.cleanup()?;

        let mut instances = vec![ ];

        for &(player, _) in &players {
            match player {
                PlayerSetup::Computer { .. } => (),
                _ => instances.push(Some(self.launcher.launch()?)),
            };

            self.players.push(player);
        }

        if instances.len() < 1 {
            bail!("expected at least one instance")
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
            let mut ports = self.launcher.create_game_ports();

            for p in &self.participants {
                ports.client_ports.push(p.instance.ports);
            }

            self.ports = Some(ports);
        }

        Ok(())
    }

    /// start a game with the given settings
    ///
    /// this will only work if one or more players was supplied to the
    /// coordinator. the coordinator cannot start a game between two computers,
    /// and replay observers are handled separately.
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

            let frame = p.update_observation()?;
            let commands = p.start(frame)?;
            p.send_commands(commands)?;
        }

        Ok(())
    }

    /// trigger an update on all participants and replay observers
    ///
    /// this will step the game if non-realtime, or update each agent with
    /// realtime game data. any idle replay observers will be assigned a replay
    /// if one is queued, and any running replays will be stepped.
    ///
    /// returns Ok(true) if update loop should continue, Ok(false) if update
    /// loop is finished (mainly for replays) and an Err if something went
    /// wrong
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
        let mut errors = vec![ ];

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue
            }

            match p.poll_leave_game() {
                Ok(true) => continue,
                Ok(false) => (),

                Err(e) => errors.push(e)
            }

            if p.is_finished_game() {
                continue
            }

            match p.req_step(self.step_size) {
                Err(e) => errors.push(e),
                _ => ()
            }
        }

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue
            }

            // TODO: should it be awaiting steps if it's possible to skip reqs?
            let frame = match p.await_step() {
                Ok(frame) => frame,
                Err(e) => {
                    errors.push(e);
                    continue
                },
            };

            if p.is_in_game() {
                let commands = match p.update(frame) {
                    Ok(commands) => commands,
                    Err(e) => {
                        errors.push(e);
                        continue
                    },
                };

                match p.send_commands(commands) {
                    Err(e) => errors.push(e),
                    _ => ()
                }
            }
            else {
                match p.end(frame) {
                    Err(e) => errors.push(e),
                    _ => ()
                }

                match p.leave_game() {
                    Err(e) => errors.push(e),
                    _ => ()
                }
            }
        }


        let mut result = Ok(());

        for e in errors.drain(..) {
            result = if let Ok(()) = result {
                Err(e)
            }
            else {
                result.chain_err(move || ErrorKind::from(e))
            }
        }

        result
    }

    fn step_agents_realtime(&mut self) -> Result<()> {
        let mut errors = vec![ ];

        for p in &mut self.participants {
            if p.get_app_state() != AppState::Normal {
                continue;
            }

            match p.poll_leave_game() {
                Ok(true) => continue,
                Ok(false) => (),

                Err(e) => errors.push(e)
            }

            let frame = match p.update_observation() {
                Ok(frame) => frame,
                Err(e) => {
                    errors.push(e);
                    continue
                }
            };

            if p.is_in_game() {
                let commands = match p.update(frame) {
                    Ok(commands) => commands,
                    Err(e) => {
                        errors.push(e);
                        continue
                    }
                };

                match p.send_commands(commands) {
                    Err(e) => errors.push(e),
                    _ => ()
                }
            }
            else {
                match p.end(frame) {
                    Err(e) => errors.push(e),
                    _ => ()
                }

                match p.leave_game() {
                    Err(e) => errors.push(e),
                    _ => ()
                }
            }
        }

        let mut result = Ok(());

        for e in errors.drain(..) {
            result = if let Ok(()) = result {
                Err(e)
            }
            else {
                result.chain_err(move || ErrorKind::from(e))
            }
        }

        result
    }

    fn start_replays(&mut self) -> Result<()> {
        let mut errors = vec![ ];

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
                            Err(e) => errors.push(e),
                            _ => ()
                        }

                        started = {
                            if !r.should_ignore() {
                                match r.req_start_replay(
                                    &file.to_string_lossy()
                                ) {
                                    Err(e) => {
                                        errors.push(e);
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

        let mut result = Ok(());

        for e in errors.drain(..) {
            result = if let Ok(()) = result {
                Err(e)
            }
            else {
                result.chain_err(move || ErrorKind::from(e))
            }
        }

        result
    }

    fn step_replay_observers(&mut self) -> Result<()> {
        let mut errors = vec![ ];

        for r in &mut self.replay_observers {
            if r.get_app_state() != AppState::Normal {
                continue
            }

            if r.has_response_pending() {
                if !r.poll() {
                    continue
                }

                match r.await_replay() {
                    Err(e) => errors.push(e),
                    _ => ()
                }

                match r.update_data() {
                    Err(e) => errors.push(e),
                    _ => ()
                }

                match r.update_observation() {
                    Ok(frame) => match r.start(frame) {
                        Err(e) => errors.push(e),
                        _ => ()
                    },
                    Err(e) => errors.push(e)
                }
            }

            if r.is_in_game() {
                match r.req_step(self.step_size) {
                    Err(e) => errors.push(e),
                    _ => ()
                }

                match r.await_step() {
                    Ok(frame) => {
                        if r.is_in_game() {
                            match r.update(frame) {
                                Err(e) => errors.push(e),
                                _ => ()
                            }
                        }
                        else {
                            match r.end(frame) {
                                Err(e) => errors.push(e),
                                _ => ()
                            }
                        }
                    },
                    Err(e) => errors.push(e)
                }
            }
        }



        if errors.is_empty() {
            self.start_replays()
        }
        else {
            let mut result = Ok(());

            for e in errors.drain(..) {
                result = if let Ok(()) = result {
                    Err(e)
                }
                else {
                    result.chain_err(move || ErrorKind::from(e))
                }
            }

            result
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

    /// cleanly shut down all managed participants
    fn cleanup(&mut self) -> Result<()> {
        let mut errors = vec![ ];

        for p in self.participants.iter_mut().chain(
            self.replay_observers.iter_mut()
        ) {
            match p.quit() {
                Err(e) => errors.push(e),
                _ => ()
            }

            match p.close() {
                Err(e) => errors.push(e),
                _ => ()
            }
        }

        self.players.clear();
        self.participants.clear();
        self.replay_observers.clear();

        let mut result = Ok(());

        for e in errors.drain(..) {
            result = if let Ok(()) = result {
                Err(e)
            }
            else {
                result.chain_err(move || ErrorKind::from(e))
            }
        }

        result
    }
}

impl Drop for Coordinator {
    fn drop(&mut self) {
        if let Err(e) = self.cleanup() {
            eprintln!("unable to cleanup coordinator {:#?}", e);
        }
    }
}
