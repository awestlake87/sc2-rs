
use std::path::PathBuf;

use sc2_proto::sc2api;

use super::super::{ Result, FromProto };
use super::{ Point2, Rect2 };

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
    pub player_id:              u32,
    /// result of the game from the perspective of the player
    pub result:                 GameResult
}

impl FromProto<sc2api::Result> for GameResult {
    fn from_proto(r: sc2api::Result) -> Result<GameResult> {
        Ok(
            match r {
                sc2api::Result::Victory => GameResult::Win,
                sc2api::Result::Defeat => GameResult::Loss,
                sc2api::Result::Tie => GameResult::Tie,
                sc2api::Result::Undecided => GameResult::Undecided,
            }
        )
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
    pub game_port:      u16,
    pub base_port:      u16
}

/// all port settings for a game
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct GamePorts {
    pub shared_port:    u16,
    pub server_ports:   PortSet,
    pub client_ports:   Vec<PortSet>
}

/// settings for a game
#[derive(Debug, Clone)]
pub struct GameSettings {
    /// which map to play on
    pub map:            Map,
}

/// current game state
#[derive(Debug, Clone)]
pub struct GameState {
    /// current step
    pub current_game_loop: u32,
    /// previous step
    pub previous_game_loop: u32,
}

/// terrain info
#[derive(Debug, Clone)]
pub struct TerrainInfo {
    /// width of the terrain
    pub width:                      i32,
    /// height of the terrain
    pub height:                     i32,

    //pathing_grid
    //terrain_height
    //placement_grid

    /// rectangle of the playable area
    pub playable_area:              Rect2,
    /// starting locations of the enemy bases
    pub enemy_start_locations:      Vec<Point2>,
    //options
    //player_info
}

impl Default for TerrainInfo {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,

            playable_area: Rect2 {
                from: Point2::new(0.0, 0.0), to: Point2::new(0.0, 0.0)
            },

            enemy_start_locations: vec![ ],
        }
    }
}

impl From<sc2api::ResponseGameInfo> for TerrainInfo {
    fn from(info: sc2api::ResponseGameInfo) -> Self {
        let mut w = 0;
        let mut h = 0;
        let mut playable_min = Point2::new(0.0, 0.0);
        let mut playable_max = Point2::new(0.0, 0.0);
        let mut start_locations = vec![ ];

        if info.has_start_raw() {
            let start_raw = info.get_start_raw();

            if start_raw.has_map_size() &&
                start_raw.get_map_size().has_x() &&
                start_raw.get_map_size().has_y()
            {
                w = start_raw.get_map_size().get_x();
                h = start_raw.get_map_size().get_y();
            }

            if start_raw.has_playable_area() {
                let area = start_raw.get_playable_area();

                if area.has_p0() {
                    playable_min.x = area.get_p0().get_x() as f32;
                    playable_min.y = area.get_p0().get_y() as f32;
                }
                if area.has_p1() {
                    playable_max.x = area.get_p1().get_x() as f32;
                    playable_max.y = area.get_p1().get_y() as f32;
                }
            }

            for p in start_raw.get_start_locations() {
                start_locations.push(
                    Point2::new(p.get_x() as f32, p.get_y() as f32)
                );
            }
        }

        Self {
            width: w,
            height: h,

            playable_area: Rect2 { from: playable_min, to: playable_max },

            enemy_start_locations: start_locations,
        }
    }
}

/// current player data as used by the observation interface
#[derive(Debug, Copy, Clone)]
pub struct PlayerData {
    /// current mineral count
    pub minerals: u32,
    /// current vespene count
    pub vespene: u32,
    /// current food capacity
    pub food_cap: u32,
    /// current food used
    pub food_used: u32,
    /// current food used by army
    pub food_army: u32,
    /// current food used by workers
    pub food_workers: u32,
    /// number of idle workers
    pub idle_worker_count: u32,
    /// number of military units
    pub army_count: u32,
    /// number of warp gates
    pub warp_gate_count: u32,
    /// number of larva
    pub larva_count: u32,
}

impl From<sc2api::PlayerCommon> for PlayerData {
    fn from(data: sc2api::PlayerCommon) -> Self {
        Self {
            minerals: data.get_minerals(),
            vespene: data.get_vespene(),
            food_used: data.get_food_used(),
            food_cap: data.get_food_cap(),
            food_army: data.get_food_army(),
            food_workers: data.get_food_workers(),
            idle_worker_count: data.get_idle_worker_count(),
            army_count: data.get_army_count(),
            warp_gate_count: data.get_warp_gate_count(),
            larva_count: data.get_larva_count()
        }
    }
}
