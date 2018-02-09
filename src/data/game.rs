use std::path::PathBuf;

use sc2_proto::sc2api;

use super::super::{FromProto, Result};

/// result of the game
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Tie,
    Undecided,
}

/// game result tied to a specific player id
#[derive(Debug, Copy, Clone)]
pub struct PlayerResult {
    player_id: u32,
    result: GameResult,
}

impl PlayerResult {
    /// player that the result is associated with
    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }

    /// result of the game from the perspective of the player
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

/// different ways of specifying a map
#[derive(Debug, Clone)]
pub enum Map {
    /// specify a map on the local filesystem
    LocalMap(PathBuf),
    /// specify a known blizzard map
    BlizzardMap(String),
}

/// settings for a game
#[derive(Debug, Clone)]
pub struct GameSettings {
    /// which map to play on
    pub map: Map,
}
