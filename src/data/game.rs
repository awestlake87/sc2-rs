
use std::path::PathBuf;

use sc2_proto::sc2api;

use super::{ Point2 };

#[derive(Clone)]
pub enum Map {
    LocalMap(PathBuf),
    BlizzardMap(String),
}

#[derive(Copy, Clone)]
pub struct PortSet {
    pub game_port:      u16,
    pub base_port:      u16
}

#[derive(Clone)]
pub struct GamePorts {
    pub shared_port:    u16,
    pub server_ports:   PortSet,
    pub client_ports:   Vec<PortSet>
}

#[derive(Clone)]
pub struct GameSettings {
    pub map:            Map,
    pub is_realtime:    bool,
    pub step_size:      usize,
}

#[derive(Clone)]
pub struct GameState {
    pub current_game_loop: u32,
    pub previous_game_loop: u32,
}

#[derive(Clone)]
pub struct GameInfo {
    pub width:                      i32,
    pub height:                     i32,
    //pathing_grid
    //terrain_height
    //placement_grid
    pub playable_min:               Point2,
    pub playable_max:               Point2,
    pub enemy_start_locations:       Vec<Point2>,
    //options
    //player_info
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,

            playable_min: Point2::new(0.0, 0.0),
            playable_max: Point2::new(0.0, 0.0),

            enemy_start_locations: vec![ ],
        }
    }
}

impl From<sc2api::ResponseGameInfo> for GameInfo {
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

            playable_min: playable_min,
            playable_max: playable_max,

            enemy_start_locations: start_locations,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerData {
    //*** Player Data ***
    pub minerals: u32,
    pub vespene: u32,
    pub food_cap: u32,
    pub food_used: u32,
    pub food_army: u32,
    pub food_workers: u32,
    pub idle_worker_count: u32,
    pub army_count: u32,
    pub warp_gate_count: u32,
    pub larva_count: u32,
    //camera_pos: Point2D,
    //start_location: Point3D,
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

pub struct GameData {
    //*** Game Data ***
    //abilities: Abilities,
    //unit_types: UnitTypes,
    //upgrade_ids: Upgrades,
    //buff_ids: Buffs,
}
