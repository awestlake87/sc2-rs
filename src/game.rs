
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

pub struct GameState {
    //*** Game State Data ***
    //unit_pool: UnitPool,
    //units_previous_map: HashMap<Tag, Unit>,
    pub current_game_loop: u32,
    //previous_game_loop: u32,
    //raw_actions: RawActions,
    //feature_layer_actions: SpatialActions,
    //power_sources: Vec<PowerSource>,
    //upgrades: Vec<UpgradeID>,
    //upgrades_previous: Vec(UpgradeID),
}

pub struct GameInfo {
    //*** Game Info Data ***
    //game_info: GameInfo,
    game_info_cached: bool,
    //use gen ability set init val to true
    use_generalized_ability: bool,
}
//proto interface is client
//observation is self
//response observation is ???
//control interface is self

pub struct PlayerData {
    //*** Player Data ***
    minerals: i32,
    vespene: i32,
    food_cap: i32,
    food_used: i32,
    food_army: i32,
    food_workers: i32,
    idle_worker_count: i32,
    army_count: i32,
    warp_gate_count: i32,
    larva_count: i32,
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
