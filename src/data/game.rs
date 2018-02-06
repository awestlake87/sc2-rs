use std::path::PathBuf;

use sc2_proto::sc2api;

use super::{ImageData, Point2, Rect2};
use super::super::{FromProto, IntoSc2, Result};

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
    /// player that the result is associated with
    pub player_id: u32,
    /// result of the game from the perspective of the player
    pub result: GameResult,
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

/// settings for a game
#[derive(Debug, Clone)]
pub struct GameSettings {
    /// which map to play on
    pub map: Map,
}

/// terrain info
#[derive(Debug, Clone)]
pub struct MapInfo {
    /// width of the terrain
    pub width: i32,
    /// height of the terrain
    pub height: i32,

    /// image that reveals pathable tiles
    pub pathing_grid: ImageData,
    /// image that reveals placable tiles
    pub placement_grid: ImageData,
    /// image that reveals terrain height
    pub terrain_height: ImageData,

    /// rectangle of the playable area
    pub playable_area: Rect2,
    /// starting locations of the enemy bases
    pub enemy_start_locations: Vec<Point2>,
    /* options */
    /* player_info */
}

impl FromProto<sc2api::ResponseGameInfo> for MapInfo {
    fn from_proto(mut info: sc2api::ResponseGameInfo) -> Result<Self> {
        let mut start_raw = info.take_start_raw();

        Ok(Self {
            width: start_raw.get_map_size().get_x(),
            height: start_raw.get_map_size().get_y(),

            pathing_grid: start_raw.take_pathing_grid().into_sc2()?,
            placement_grid: start_raw.take_placement_grid().into_sc2()?,
            terrain_height: start_raw.take_terrain_height().into_sc2()?,

            playable_area: {
                let area = start_raw.get_playable_area();

                Rect2 {
                    from: Point2::new(
                        area.get_p0().get_x() as f32,
                        area.get_p0().get_y() as f32,
                    ),
                    to: Point2::new(
                        area.get_p1().get_x() as f32,
                        area.get_p1().get_y() as f32,
                    ),
                }
            },

            enemy_start_locations: start_raw
                .take_start_locations()
                .into_iter()
                .map(|p| Point2::new(p.get_x() as f32, p.get_y() as f32))
                .collect(),
        })
    }
}
