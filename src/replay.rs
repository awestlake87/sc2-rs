use std::{mem, path::PathBuf, rc::Rc};

use futures::{
    prelude::*,
    unsync::{mpsc, oneshot},
};
use sc2_proto::{common, sc2api};
use tokio_core::reactor;

use constants::sc2_bug_tag;
use data::{GameResult, Race};
use services::replay_service::{
    ReplaySpectator,
    SpectatorClient,
    SpectatorRequest,
};
use {Error, ErrorKind, FromProto, IntoSc2, Result};

pub use services::replay_service::{ReplayBuilder, ReplaySink};

#[derive(Debug, Clone)]
pub enum Replay {
    LocalReplay(PathBuf),
}

/// Information about a player in a replay.
#[derive(Debug, Copy, Clone)]
pub struct ReplayPlayerInfo {
    player_id: u32,
    mmr: i32,
    apm: i32,

    race: Race,
    race_selected: Option<Race>,
    game_result: Option<GameResult>,
}

impl ReplayPlayerInfo {
    /// Id of the player.
    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }

    /// Player ranking.
    pub fn get_mmr(&self) -> i32 {
        self.mmr
    }

    /// Player actions per minute.
    pub fn get_apm(&self) -> i32 {
        self.apm
    }

    /// Actual player race.
    pub fn get_race(&self) -> Race {
        self.race
    }

    /// Selected player race (if Random or None, race will be different).
    pub fn get_race_selected(&self) -> Option<Race> {
        self.race_selected
    }

    /// If the player won or lost.
    pub fn get_game_result(&self) -> Option<GameResult> {
        self.game_result
    }
}

impl FromProto<sc2api::PlayerInfoExtra> for ReplayPlayerInfo {
    fn from_proto(info: sc2api::PlayerInfoExtra) -> Result<Self> {
        Ok(Self {
            player_id: info.get_player_info().get_player_id(),

            race: info.get_player_info()
                .get_race_actual()
                .into_sc2()?,
            race_selected: {
                if info.get_player_info().has_race_requested() {
                    let proto_race =
                        info.get_player_info().get_race_requested();

                    if proto_race != common::Race::NoRace {
                        Some(proto_race.into_sc2()?)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },

            mmr: info.get_player_mmr(),
            apm: info.get_player_apm(),

            game_result: {
                if info.has_player_result() {
                    Some(info.get_player_result()
                        .get_result()
                        .into_sc2()?)
                } else {
                    None
                }
            },
        })
    }
}

/// Information about a replay file.
#[derive(Debug, Clone)]
pub struct ReplayInfo {
    map_name: String,
    map_path: String,
    game_version: String,
    data_version: String,

    duration: f32,
    duration_steps: u32,

    data_build: u32,
    base_build: u32,

    players: Vec<ReplayPlayerInfo>,
}

impl ReplayInfo {
    /// Name of the map.
    pub fn get_map_name(&self) -> &str {
        &*self.map_name
    }

    /// Path to the map.
    pub fn get_map_path(&self) -> &str {
        &*self.map_path
    }

    /// Version of the game.
    pub fn get_game_version(&self) -> &str {
        &*self.game_version
    }

    /// Data version of the game.
    pub fn get_data_version(&self) -> &str {
        &*self.data_version
    }

    /// Duration in seconds.
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    /// Duration in game steps.
    pub fn get_duration_in_steps(&self) -> u32 {
        self.duration_steps
    }

    /// Data build of the game.
    pub fn get_data_build(&self) -> u32 {
        self.data_build
    }

    /// Required base build of the game.
    pub fn get_base_build(&self) -> u32 {
        self.base_build
    }

    /// Information about specific players.
    pub fn get_players(&self) -> &[ReplayPlayerInfo] {
        &*self.players
    }
}

impl FromProto<sc2api::ResponseReplayInfo> for ReplayInfo {
    fn from_proto(mut info: sc2api::ResponseReplayInfo) -> Result<Self> {
        Ok(Self {
            map_name: info.take_map_name(),
            map_path: info.take_local_map_path(),
            game_version: info.take_game_version(),
            data_version: info.take_data_version(),

            duration: info.get_game_duration_seconds(),
            duration_steps: info.get_game_duration_loops(),

            data_build: info.get_data_build(),
            base_build: info.get_base_build(),

            players: {
                let mut player_info = vec![];

                for p in info.take_player_info().into_iter() {
                    player_info.push(p.into_sc2()?);
                }

                player_info
            },
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpectatorChoice {
    WatchPlayer(u32),
    Pass,
}

pub struct SpectatorBuilder {
    req_tx: Option<mpsc::Sender<SpectatorRequest>>,
    req_rx: Option<mpsc::Receiver<SpectatorRequest>>,

    player_picker: Option<Box<FnMut(&ReplayInfo) -> SpectatorChoice>>,
}

impl SpectatorBuilder {
    pub fn new() -> Self {
        let (req_tx, req_rx) = mpsc::channel(1);

        Self {
            req_tx: Some(req_tx),
            req_rx: Some(req_rx),

            player_picker: None,
        }
    }

    pub fn player_picker<F>(self, picker: F) -> Self
    where
        F: FnMut(&ReplayInfo) -> SpectatorChoice + 'static,
    {
        Self {
            player_picker: Some(Box::new(picker)),
            ..self
        }
    }
}

impl ReplaySpectator for SpectatorBuilder {
    fn spawn(&mut self, handle: &reactor::Handle) -> Result<SpectatorClient> {
        if self.player_picker.is_none() {
            bail!(ErrorKind::MissingRequirement(
                "Spectator requires player picker".to_string()
            ))
        }

        handle.spawn(
            SpectatorService::new(
                mem::replace(&mut self.req_rx, None).unwrap(),
                mem::replace(&mut self.player_picker, None).unwrap(),
            ).run()
                .map_err(|e| {
                    unreachable!(
                        "{}: SpectatorService ended unexpectedly - {:#?}",
                        sc2_bug_tag(),
                        e
                    )
                }),
        );

        Ok(SpectatorClient::wrap(
            mem::replace(&mut self.req_tx, None).unwrap(),
        ))
    }
}

struct SpectatorService {
    req_rx: mpsc::Receiver<SpectatorRequest>,
    player_picker: Box<FnMut(&ReplayInfo) -> SpectatorChoice>,
}

impl SpectatorService {
    fn new(
        req_rx: mpsc::Receiver<SpectatorRequest>,
        player_picker: Box<FnMut(&ReplayInfo) -> SpectatorChoice>,
    ) -> Self {
        Self {
            req_rx,
            player_picker: player_picker,
        }
    }

    #[async]
    fn run(mut self) -> Result<()> {
        #[async]
        for req in self.req_rx
            .map_err(|_| -> Error { unreachable!() })
        {
            match req {
                SpectatorRequest::WhichPlayer(info, rsp) => {
                    let choice = (*self.player_picker)(&info);

                    rsp.send(choice).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to respond with SpectatorChoice",
                            sc2_bug_tag()
                        )
                    })?;
                },
            }
        }

        Ok(())
    }
}
