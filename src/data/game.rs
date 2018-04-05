use std::path::PathBuf;

use sc2_proto::sc2api;

use super::super::{FromProto, Result};

/// Result of the game.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Tie,
    Undecided,
}

/// Game result tied to a specific player id.
#[derive(Debug, Copy, Clone)]
pub struct PlayerResult {
    player_id: u32,
    result: GameResult,
}

impl PlayerResult {
    /// Player that the result is associated with.
    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }

    /// Result of the game from the perspective of the player.
    pub fn get_result(&self) -> GameResult {
        self.result
    }
}

impl FromProto<sc2api::Result> for GameResult {
    fn from_proto(r: sc2api::Result) -> Result<GameResult> {
        Ok(match r {
            sc2api::Result::Victory => GameResult::Win,
            sc2api::Result::Defeat => GameResult::Loss,
            sc2api::Result::Tie => GameResult::Tie,
            sc2api::Result::Undecided => GameResult::Undecided,
        })
    }
}

/// Different ways of specifying a map.
#[derive(Debug, Clone)]
pub enum Map {
    /// Specify a map on the local filesystem.
    LocalMap(PathBuf),
    /// Specify a known blizzard map.
    BlizzardMap(String),
}

/// Settings for a game.
#[derive(Debug, Clone)]
pub struct GameSetup {
    map: Map,
}

impl GameSetup {
    /// Create a game setup for the given map.
    pub fn new(map: Map) -> Self {
        Self { map: map }
    }

    /// Get the map.
    pub fn get_map(&self) -> &Map {
        &self.map
    }
}
