
struct Observer {
    player_id: u32,

    //*** Game State Data ***
    //unit_pool: UnitPool,
    //units_previous_map: HashMap<Tag, Unit>,
    current_game_loop: u32,
    previous_game_loop: u32,
    //raw_actions: RawActions,
    //feature_layer_actions: SpatialActions,
    //power_sources: Vec<PowerSource>,
    //upgrades: Vec<UpgradeID>,
    //upgrades_previous: Vec(UpgradeID),

    //*** Game Info Data ***
    //game_info: GameInfo,
    game_info_cached: bool,
    //use gen ability set init val to true
    use_generalized_ability: bool,

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

    //*** Game Data ***
    //abilities: Abilities,
    //unit_types: UnitTypes,
    //upgrade_ids: Upgrades,
    //buff_ids: Buffs,

    //*** Score ***
    //score: Score,

    //*** Cached Data ***
    abilities_cached: bool;
    unit_types_cached: bool;
    upgrades_cached: bool;
    buffs_cached: bool;
}
