
use std::path::PathBuf;

#[derive(Clone)]
pub enum Map {
    LocalMap(PathBuf),
    BlizzardMap(String),
}

#[derive(Copy, Clone)]
pub struct EndpointPorts {
    pub game_port:      u16,
    pub base_port:      u16
}

#[derive(Copy, Clone)]
pub struct GamePorts {
    pub shared_port:    u16,
    pub server_ports:   EndpointPorts,
    pub client_ports:   EndpointPorts
}

#[derive(Clone)]
pub struct GameSettings {
    pub map:            Map
}

#[derive(Clone)]
pub struct GameState {
    //*** Game State Data ***
    //unit_pool: UnitPool,
    //units_previous_map: HashMap<Tag, Unit>,
    pub current_game_loop: u32,
    pub previous_game_loop: u32,
    //raw_actions: RawActions,
    //feature_layer_actions: SpatialActions,
    //power_sources: Vec<PowerSource>,
    //upgrades: Vec<UpgradeID>,
    //upgrades_previous: Vec(UpgradeID),
}

#[derive(Copy, Clone, Debug)]
pub struct GameInfo {
    //*** Game Info Data ***
    //game_info: GameInfo,
    pub game_info_cached: bool,
    //use gen ability set init val to true
    pub use_generalized_ability: bool,
}
//proto interface is client
//observation is self
//response observation is ???
//control interface is self

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

pub struct GameData {
    //*** Game Data ***
    //abilities: Abilities,
    //unit_types: UnitTypes,
    //upgrade_ids: Upgrades,
    //buff_ids: Buffs,
}

pub struct Score {
    //*** Score ***
    //score: Score
}
