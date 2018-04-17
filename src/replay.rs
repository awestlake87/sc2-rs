use std::path::PathBuf;

use sc2_proto::{common, sc2api};

use data::{GameResult, Race};
use {Error, FromProto, IntoSc2, Result};

pub use services::replay_service::{ReplayBuilder, ReplaySink};

#[derive(Debug, Clone)]
pub enum Replay {
    LocalReplay(PathBuf),
}

/// Information about a player in a replay.
#[derive(Debug, Copy, Clone)]
pub struct ReplayPlayerInfo {
    /// Id of the player.
    player_id: u32,
    /// Player ranking.
    mmr: i32,
    /// Player actions per minute.
    apm: i32,

    /// Actual player race.
    race: Race,
    /// Selected player race (if Random or None, race will be different).
    race_selected: Option<Race>,
    /// If the player won or lost.
    game_result: Option<GameResult>,
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
    /// Name of the map.
    map_name: String,
    /// Path to the map.
    map_path: String,
    /// Version of the game.
    game_version: String,
    /// Data version of the game.
    data_version: String,

    /// Duration in seconds.
    duration: f32,
    /// Duration in game steps.
    duration_steps: u32,

    /// Data build of the game.
    data_build: u32,
    /// Required base build of the game.
    base_build: u32,

    /// Information about specific players.
    players: Vec<ReplayPlayerInfo>,
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
