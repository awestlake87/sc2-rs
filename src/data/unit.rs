
use sc2_proto::raw;

use super::{ Ability, Buff, Point2, Point3 };

/// unique tag for a unit instance
pub type Tag = u64;

/// a unit (could be structure, a worker, or military)
pub struct Unit {
    /// whether the unit is shown on screen or not
    pub display_type:           DisplayType,
    /// relationship of the unit to this player
    pub alliance:               Alliance,

    /// a unique id for the instance of a unit
    pub tag:                    Tag,
    /// the type of unit
    pub unit_type:              UnitType,
    /// which player owns this unit
    pub owner:                  i32,

    /// position of the unit in the world
    pub pos:                    Point3,
    /// direction the unit faces in radians
    pub facing:                 f32,
    /// radius of the unit
    pub radius:                 f32,
    /// gives progress under construction (range [0.0, 1.0] where 1.0 is done)
    pub build_progress:         f32,

    /// whether the unit is cloaked
    pub cloak:                  CloakState,

    /// range of detector for detector units
    pub detect_range:           f32,
    /// range of radar for radar units
    pub radar_range:            f32,

    /// whether this unit is currently selected
    pub is_selected:            bool,
    /// whether this unit is visible and within the camera frustum
    pub is_on_screen:           bool,
    /// whether this unit is detected by a sensor tower
    pub is_blip:                bool,

    /// health of the unit (not set for snapshots)
    pub health:                 f32,
    /// max health of the unit (not set for snapshots)
    pub health_max:             f32,
    /// shield of the unit (not set for snapshots)
    pub shield:                 f32,
    /// energy of the unit (not set for snapshots)
    pub energy:                 f32,
    /// amount of minerals if unit is a mineral field (not set for snapshot)
    pub mineral_contents:       i32,
    /// amount of vespene if unit is a geyser (not set for snapshots)
    pub vespene_contents:       i32,
    /// whether the unit is flying (not set for snapshots)
    pub is_flying:              bool,
    /// whether the unit is burrowed (not set for snapshots)
    pub is_burrowed:            bool,
    /// time remaining for a weapon on cooldown (not set for snapshots)
    pub weapon_cooldown:        f32,

    /// orders on this unit (only valid for this player's units)
    pub orders:                 Vec<UnitOrder>,
    /// addon like a tech lab or reactor (only valid for this player's units)
    pub add_on_tag:             Tag,
    /// passengers in this transport (only valid for this player's units)
    pub passengers:             Vec<PassengerUnit>,
    /// number of cargo slots used (only valid for this player's units)
    pub cargo_space_taken:      i32,
    /// max number of cargo slots (only valid for this player's units)
    pub cargo_space_max:        i32,
    /// number of harvesters associated with town hall
    pub assigned_harvesters:    i32,
    /// number of harvesters that can be assigned to a town hall
    pub ideal_harvesters:       i32,
    /// target unit of a unit (only valid for this player's units)
    pub engaged_target_tag:     Tag,
    /// buffs on this unit (only valid for this player's units)
    pub buffs:                  Vec<Buff>,
    /// whether this unit is powered by a pylon
    pub is_powered:             bool,

    /// whether this unit is alive or not
    pub is_alive:               bool,
    /// the last time the unit was seen
    pub last_seen_game_loop:    u32,
}

impl Unit {
    /// mark this unit as dead
    pub fn mark_dead(&mut self) {
        self.is_alive = false;
    }
}

impl From<raw::Unit> for Unit {
    fn from(unit: raw::Unit) -> Self {
        Self {
            display_type: DisplayType::from(unit.get_display_type()),
            alliance: Alliance::from(unit.get_alliance()),

            tag: unit.get_tag(),
            unit_type: UnitType::from_id(unit.get_unit_type()),
            owner: unit.get_owner(),

            pos: {
                let pos = unit.get_pos();
                Point3::new(pos.get_x(), pos.get_y(), pos.get_z())
            },
            facing: unit.get_facing(),
            radius: unit.get_radius(),
            build_progress: unit.get_build_progress(),

            cloak: {
                if unit.has_cloak() {
                    CloakState::from(unit.get_cloak())
                }
                else {
                    CloakState::Unknown
                }
            },

            detect_range: unit.get_detect_range(),
            radar_range: unit.get_radar_range(),

            is_selected: unit.get_is_selected(),
            is_on_screen: unit.get_is_on_screen(),
            is_blip: unit.get_is_blip(),

            health: unit.get_health(),
            health_max: unit.get_health_max(),
            shield: unit.get_shield(),
            energy: unit.get_energy(),
            mineral_contents: unit.get_mineral_contents(),
            vespene_contents: unit.get_vespene_contents(),
            is_flying: unit.get_is_flying(),
            is_burrowed: unit.get_is_burrowed(),
            weapon_cooldown: unit.get_weapon_cooldown(),

            orders: {
                let mut orders = vec![ ];

                for order in unit.get_orders().iter() {
                    orders.push(UnitOrder::from(order.clone()));
                }

                orders
            },
            add_on_tag: unit.get_add_on_tag(),
            passengers: {
                let mut passengers = vec![ ];

                for passenger in unit.get_passengers().iter() {
                    passengers.push(PassengerUnit::from(passenger.clone()));
                }

                passengers
            },
            cargo_space_taken: unit.get_cargo_space_taken(),
            cargo_space_max: unit.get_cargo_space_max(),
            assigned_harvesters: unit.get_assigned_harvesters(),
            ideal_harvesters: unit.get_ideal_harvesters(),
            engaged_target_tag: unit.get_engaged_target_tag(),
            buffs: {
                let mut buffs = vec![ ];

                for buff in unit.get_buff_ids().iter() {
                    buffs.push(Buff::from_id(*buff));
                }

                buffs
            },
            is_powered: unit.get_is_powered(),

            is_alive: true,
            last_seen_game_loop: 0,
        }
    }
}
/// list of known StarCraft II unit types
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnitType {
    Invalid,

    // Terran
    TerranArmory,                 // CANCEL, HALT, CANCEL_LAST, RESEARCH_TERRANSHIPWEAPONS, RESEARCH_TERRANVEHICLEANDSHIPPLATING, RESEARCH_TERRANVEHICLEWEAPONS
    TerranAutoTurret,             // SMART, STOP, ATTACK
    TerranBanshee,                // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, BEHAVIOR_CLOAKON, BEHAVIOR_CLOAKOFF
    TerranBarracks,               // SMART, TRAIN_MARINE, TRAIN_REAPER, TRAIN_GHOST, TRAIN_MARAUDER, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    TerranBarracksFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    TerranBarracksReactor,        // CANCEL
    TerranBarracksTechLab,        // RESEARCH_STIMPACK, RESEARCH_COMBATSHIELD, RESEARCH_CONCUSSIVESHELLS, CANCEL, CANCEL_LAST
    TerranBattleCruiser,          // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_YAMATOGUN, EFFECT_TACTICALJUMP, STOP, ATTACK
    TerranBunker,                 // SMART, EFFECT_SALVAGE, CANCEL, HALT, UNLOADALL, STOP, LOAD, RALLY_UNITS, ATTACK, EFFECT_STIM
    TerranCommandCenter,          // SMART, TRAIN_SCV, MORPH_PLANETARYFORTRESS, MORPH_ORBITALCOMMAND, CANCEL, HALT, LOADALL, UNLOADALL, CANCEL_LAST, LIFT, RALLY_WORKERS
    TerranCommandCenterFlying,    // SMART, MOVE, PATROL, HOLDPOSITION, LOADALL, UNLOADALL, STOP, LAND
    TerranCyclone,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_LOCKON, CANCEL, STOP, ATTACK
    TerranEngineeringBay,         // RESEARCH_HISECAUTOTRACKING, RESEARCH_TERRANSTRUCTUREARMORUPGRADE, RESEARCH_NEOSTEELFRAME, CANCEL, HALT, CANCEL_LAST, RESEARCH_TERRANINFANTRYARMOR, RESEARCH_TERRANINFANTRYWEAPONS
    TerranFactory,                // SMART, TRAIN_SIEGETANK, TRAIN_THOR, TRAIN_HELLION, TRAIN_HELLBAT, TRAIN_CYCLONE, TRAIN_WIDOWMINE, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    TerranFactoryFlying,          // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    TerranFactoryReactor,         // CANCEL
    TerranFactoryTechLab,         // RESEARCH_INFERNALPREIGNITER, RESEARCH_DRILLINGCLAWS, RESEARCH_MAGFIELDLAUNCHERS, CANCEL, CANCEL_LAST
    TerranFusionCore,             // RESEARCH_BATTLECRUISERWEAPONREFIT, CANCEL, HALT, CANCEL_LAST
    TerranGhost,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_NUKECALLDOWN, EFFECT_EMP, EFFECT_GHOSTSNIPE, CANCEL, STOP, ATTACK, BEHAVIOR_CLOAKON, BEHAVIOR_CLOAKOFF, BEHAVIOR_HOLDFIREON, BEHAVIOR_HOLDFIREOFF
    TerranGhostAcademy,           // BUILD_NUKE, RESEARCH_PERSONALCLOAKING, CANCEL, HALT, CANCEL_LAST
    TerranHellion,                // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_HELLBAT, STOP, ATTACK
    TerranHellionTank,            // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_HELLION, STOP, ATTACK
    TerranLiberator,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_LIBERATORAGMODE, STOP, ATTACK
    TerranLiberatorAg,            // SMART, MORPH_LIBERATORAAMODE, STOP, ATTACK
    TerranMarauder,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, EFFECT_STIM
    TerranMarine,                 // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, EFFECT_STIM
    TerranMedivac,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_HEAL, EFFECT_MEDIVACIGNITEAFTERBURNERS, STOP, LOAD, UNLOADALLAT, ATTACK
    TerranMissileTurret,          // SMART, CANCEL, HALT, STOP, ATTACK
    TerranMule,                   // SMART, MOVE, PATROL, HOLDPOSITION, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_REPAIR
    TerranOrbitalCommand,         // SMART, EFFECT_CALLDOWNMULE, EFFECT_SUPPLYDROP, EFFECT_SCAN, TRAIN_SCV, CANCEL_LAST, LIFT, RALLY_WORKERS
    TerranOrbitalCommandFlying,   // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND
    TerranPlanetaryFortress,      // SMART, TRAIN_SCV, LOADALL, STOP, CANCEL_LAST, ATTACK, RALLY_WORKERS
    TerranRaven,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_POINTDEFENSEDRONE, EFFECT_HUNTERSEEKERMISSILE, EFFECT_AUTOTURRET, STOP, ATTACK
    TerranReaper,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_KD8CHARGE, STOP, ATTACK
    TerranRefinery,               // CANCEL, HALT
    TerranScv,                    // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_COMMANDCENTER, BUILD_SUPPLYDEPOT, BUILD_REFINERY, BUILD_BARRACKS, BUILD_ENGINEERINGBAY, BUILD_MISSILETURRET, BUILD_BUNKER, BUILD_SENSORTOWER, BUILD_GHOSTACADEMY, BUILD_FACTORY, BUILD_STARPORT, BUILD_ARMORY, BUILD_FUSIONCORE, HALT, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY, EFFECT_REPAIR
    TerranSensorTower,            // CANCEL, HALT
    TerranSiegeTank,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_SIEGEMODE, STOP, ATTACK
    TerranSiegeTankSieged,        // SMART, MORPH_UNSIEGE, STOP, ATTACK
    TerranStarport,               // SMART, TRAIN_MEDIVAC, TRAIN_BANSHEE, TRAIN_RAVEN, TRAIN_BATTLECRUISER, TRAIN_VIKINGFIGHTER, TRAIN_LIBERATOR, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    TerranStarportFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    TerranStarportReactor,        // CANCEL
    TerranStarportTechLab,        // RESEARCH_BANSHEECLOAKINGFIELD, RESEARCH_RAVENCORVIDREACTOR, RESEARCH_BANSHEEHYPERFLIGHTROTORS, RESEARCH_RAVENRECALIBRATEDEXPLOSIVES, RESEARCH_HIGHCAPACITYFUELTANKS, RESEARCH_ADVANCEDBALLISTICS, CANCEL, CANCEL_LAST
    TerranSupplyDepot,            // MORPH_SUPPLYDEPOT_LOWER, CANCEL, HALT
    TerranSupplyDepotLowered,     // MORPH_SUPPLYDEPOT_RAISE
    TerranThor,                   // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_THORHIGHIMPACTMODE, STOP, ATTACK
    TerranThorAp,                 // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_THOREXPLOSIVEMODE, CANCEL, STOP, ATTACK
    TerranVikingAssault,          // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_VIKINGFIGHTERMODE, STOP, ATTACK
    TerranVikingFighter,          // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_VIKINGASSAULTMODE, STOP, ATTACK
    TerranWidowMine,              // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    TerranWidowMineBurrowed,      // SMART, EFFECT_WIDOWMINEATTACK, BURROWUP
    // Terran non-interactive
    TerranKd8Charge,
    TerranNuke,
    TerranPointDefenseDrone,
    TerranReactor,
    TerranTechLab,

    // Zerg
    ZergBaneling,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_EXPLODE, BEHAVIOR_BUILDINGATTACKON, BEHAVIOR_BUILDINGATTACKOFF, BURROWDOWN, STOP, ATTACK
    ZergBanelingBurrowed,       // EFFECT_EXPLODE, BURROWUP
    ZergBanelingCocoon,         // SMART, CANCEL_LAST, RALLY_UNITS
    ZergBanelingNest,           // RESEARCH_CENTRIFUGALHOOKS, CANCEL, CANCEL_LAST
    ZergBroodling,              // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergBroodlord,              // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergBroodlordCocoon,        // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    ZergChangeling,             // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergChangelingMarine,       // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergChangelingMarineShield, // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergChangelingZealot,       // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergChangelingZergling,     // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergChangelingZerglingWings,// SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergCorruptor,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_BROODLORD, EFFECT_CAUSTICSPRAY, STOP, ATTACK
    ZergCreepTumor,             // CANCEL
    ZergCreepTumorBurrowed,     // SMART, CANCEL, BUILD_CREEPTUMOR
    ZergCreepTumorQueen,        // CANCEL
    ZergDrone,                  // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_HATCHERY, BUILD_EXTRACTOR, BUILD_SPAWNINGPOOL, BUILD_EVOLUTIONCHAMBER, BUILD_HYDRALISKDEN, BUILD_SPIRE, BUILD_ULTRALISKCAVERN, BUILD_INFESTATIONPIT, BUILD_NYDUSNETWORK, BUILD_BANELINGNEST, BUILD_ROACHWARREN, BUILD_SPINECRAWLER, BUILD_SPORECRAWLER, BURROWDOWN, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY
    ZergDroneBurrowed,          // BURROWUP
    ZergEgg,                    // SMART, CANCEL_LAST, RALLY_UNITS
    ZergEvolutionChamber,       // CANCEL, CANCEL_LAST, RESEARCH_ZERGGROUNDARMOR, RESEARCH_ZERGMELEEWEAPONS, RESEARCH_ZERGMISSILEWEAPONS
    ZergExtractor,              // CANCEL
    ZergGreaterSpire,           // CANCEL_LAST, RESEARCH_ZERGFLYERARMOR, RESEARCH_ZERGFLYERATTACK
    ZergHatchery,               // SMART, MORPH_LAIR, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    ZergHive,                   // SMART, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    ZergHydralisk,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_LURKER, BURROWDOWN, STOP, ATTACK
    ZergHydraliskBurrowed,      // BURROWUP
    ZergHydraliskDen,           // RESEARCH_GROOVEDSPINES, RESEARCH_MUSCULARAUGMENTS, MORPH_LURKERDEN, CANCEL, CANCEL_LAST
    ZergInfestationPit,         // RESEARCH_PATHOGENGLANDS, RESEARCH_NEURALPARASITE, CANCEL, CANCEL_LAST
    ZergInfestedTerransEgg,     // SMART, MOVE, PATROL, HOLDPOSITION
    ZergInfestor,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FUNGALGROWTH, EFFECT_INFESTEDTERRANS, EFFECT_NEURALPARASITE, CANCEL, BURROWDOWN, STOP, ATTACK
    ZergInfestorBurrowed,       // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FUNGALGROWTH, EFFECT_INFESTEDTERRANS, EFFECT_NEURALPARASITE, CANCEL, BURROWUP, STOP, ATTACK
    ZergInfestorTerran,         // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    ZergLair,                   // SMART, MORPH_HIVE, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    ZergLarva,                  // TRAIN_DRONE, TRAIN_ZERGLING, TRAIN_OVERLORD, TRAIN_HYDRALISK, TRAIN_MUTALISK, TRAIN_ULTRALISK, TRAIN_ROACH, TRAIN_INFESTOR, TRAIN_CORRUPTOR, TRAIN_VIPER, TRAIN_SWARMHOST
    ZergLocustMp,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergLocustMpFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_LOCUSTSWOOP, STOP, ATTACK
    ZergLurkerDenMp,            // RESEARCH_GROOVEDSPINES, RESEARCH_MUSCULARAUGMENTS, CANCEL_LAST
    ZergLurkerMp,               // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    ZergLurkerMpBurrowed,       // SMART, BURROWUP, STOP, ATTACK, BEHAVIOR_HOLDFIREON, BEHAVIOR_HOLDFIREOFF
    ZergLurkerMpEgg,            // SMART, CANCEL, RALLY_UNITS
    ZergMutalisk,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ZergNydusCanal,             // SMART, UNLOADALL, STOP, LOAD, RALLY_UNITS
    ZergNydusNetwork,           // SMART, BUILD_NYDUSWORM, CANCEL, UNLOADALL, STOP, LOAD, RALLY_UNITS
    ZergOverlord,               // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_OVERSEER, BEHAVIOR_GENERATECREEPON, BEHAVIOR_GENERATECREEPOFF, MORPH_OVERLORDTRANSPORT, CANCEL, STOP, ATTACK
    ZergOverlordCocoon,         // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    ZergOverlordTransport,      // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_OVERSEER, BEHAVIOR_GENERATECREEPON, BEHAVIOR_GENERATECREEPOFF, STOP, LOAD, UNLOADALLAT, ATTACK
    ZergOverseer,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_SPAWNCHANGELING, EFFECT_CONTAMINATE, STOP, ATTACK
    ZergQueen,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_INJECTLARVA, EFFECT_TRANSFUSION, BURROWDOWN, STOP, ATTACK, BUILD_CREEPTUMOR
    ZergQueenBurrowed,          // BURROWUP
    ZergRavager,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_CORROSIVEBILE, BURROWDOWN, STOP, ATTACK
    ZergRavagerCocoon,          // SMART, CANCEL, RALLY_UNITS
    ZergRoach,                  // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_RAVAGER, BURROWDOWN, STOP, ATTACK
    ZergRoachBurrowed,          // SMART, MOVE, PATROL, HOLDPOSITION, BURROWUP, STOP, ATTACK
    ZergRoachWarren,            // RESEARCH_GLIALREGENERATION, RESEARCH_TUNNELINGCLAWS, CANCEL, CANCEL_LAST
    ZergSpawningPool,           // RESEARCH_ZERGLINGADRENALGLANDS, RESEARCH_ZERGLINGMETABOLICBOOST, CANCEL, CANCEL_LAST
    ZergSpineCrawler,           // SMART, CANCEL, STOP, ATTACK, MORPH_UPROOT
    ZergSpineCrawlerUprooted,   // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK, MORPH_ROOT
    ZergSpire,                  // MORPH_GREATERSPIRE, CANCEL, CANCEL_LAST, RESEARCH_ZERGFLYERARMOR, RESEARCH_ZERGFLYERATTACK
    ZergSporeCrawler,           // SMART, CANCEL, STOP, ATTACK, MORPH_UPROOT
    ZergSporeCrawlerUprooted,   // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK, MORPH_ROOT
    ZergSwarmHostBurrowedMp,    // SMART, EFFECT_SPAWNLOCUSTS, BURROWUP
    ZergSwarmHostMp,            // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_SPAWNLOCUSTS, BURROWDOWN, STOP, ATTACK
    ZergTransportOverlordCocoon,// SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    ZergUltralisk,              // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    ZergUltraliskCavern,        // RESEARCH_CHITINOUSPLATING, CANCEL, CANCEL_LAST
    ZergViper,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_BLINDINGCLOUD, EFFECT_ABDUCT, EFFECT_VIPERCONSUME, EFFECT_PARASITICBOMB, STOP, ATTACK
    ZergZergling,               // SMART, MOVE, PATROL, HOLDPOSITION, TRAIN_BANELING, BURROWDOWN, STOP, ATTACK
    ZergZerglingBurrowed,       // BURROWUP
    // Zerg non-interactive
    ZergParasiticBombDummy,

    // Protoss
    ProtossAdept,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_ADEPTPHASESHIFT, CANCEL, STOP, RALLY_UNITS, ATTACK
    ProtossAdeptPhaseShift,        // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK
    ProtossArchon,                 // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK
    ProtossAssimilator,            // CANCEL
    ProtossCarrier,                // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_INTERCEPTORS, STOP, CANCEL_LAST, ATTACK
    ProtossColossus,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ProtossCyberneticScore,        // RESEARCH_WARPGATE, CANCEL, CANCEL_LAST, RESEARCH_PROTOSSAIRARMOR, RESEARCH_PROTOSSAIRWEAPONS
    ProtossDarkShrine,             // RESEARCH_SHADOWSTRIKE, CANCEL, CANCEL_LAST
    ProtossDarkTemplar,            // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK, EFFECT_BLINK
    ProtossDisruptor,              // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_PURIFICATIONNOVA, STOP, ATTACK
    ProtossDisruptorPhased,        // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ProtossFleetBeacon,            // RESEARCH_INTERCEPTORGRAVITONCATAPULT, RESEARCH_PHOENIXANIONPULSECRYSTALS, CANCEL, CANCEL_LAST
    ProtossForge,                  // CANCEL, CANCEL_LAST, RESEARCH_PROTOSSGROUNDARMOR, RESEARCH_PROTOSSGROUNDWEAPONS, RESEARCH_PROTOSSSHIELDS
    ProtossGateway,                // SMART, TRAIN_ZEALOT, TRAIN_STALKER, TRAIN_HIGHTEMPLAR, TRAIN_DARKTEMPLAR, TRAIN_SENTRY, TRAIN_ADEPT, MORPH_WARPGATE, CANCEL, CANCEL_LAST, RALLY_UNITS
    ProtossHighTemplar,            // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FEEDBACK, EFFECT_PSISTORM, STOP, RALLY_UNITS, ATTACK
    ProtossImmortal,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_IMMORTALBARRIER, STOP, ATTACK
    ProtossInterceptor,            // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ProtossMothership,             // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_PHOTONOVERCHARGE, EFFECT_TIMEWARP, STOP, ATTACK, EFFECT_MASSRECALL
    ProtossMothershipCore,         // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_MOTHERSHIP, EFFECT_PHOTONOVERCHARGE, EFFECT_TIMEWARP, CANCEL, STOP, ATTACK, EFFECT_MASSRECALL
    ProtossNexus,                  // SMART, EFFECT_CHRONOBOOST, TRAIN_PROBE, TRAIN_MOTHERSHIPCORE, CANCEL, CANCEL_LAST, RALLY_WORKERS
    ProtossObserver,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ProtossOracle,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_ORACLEREVELATION, BEHAVIOR_PULSARBEAMON, BEHAVIOR_PULSARBEAMOFF, BUILD_STASISTRAP, CANCEL, STOP, ATTACK
    ProtossOracleStasisTrap,       // CANCEL
    ProtossPhoenix,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_GRAVITONBEAM, CANCEL, STOP, ATTACK
    ProtossPhotonCannon,           // SMART, CANCEL, STOP, ATTACK
    ProtossProbe,                  // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_NEXUS, BUILD_PYLON, BUILD_ASSIMILATOR, BUILD_GATEWAY, BUILD_FORGE, BUILD_FLEETBEACON, BUILD_TWILIGHTCOUNCIL, BUILD_PHOTONCANNON, BUILD_STARGATE, BUILD_TEMPLARARCHIVE, BUILD_DARKSHRINE, BUILD_ROBOTICSBAY, BUILD_ROBOTICSFACILITY, BUILD_CYBERNETICSCORE, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY
    ProtossPylon,                  // CANCEL
    ProtossPylonOvercharged,       // SMART, STOP, ATTACK
    ProtossRoboticsBay,            // RESEARCH_GRAVITICBOOSTER, RESEARCH_GRAVITICDRIVE, RESEARCH_EXTENDEDTHERMALLANCE, CANCEL, CANCEL_LAST
    ProtossRoboticsFacility,       // SMART, TRAIN_WARPPRISM, TRAIN_OBSERVER, TRAIN_COLOSSUS, TRAIN_IMMORTAL, TRAIN_DISRUPTOR, CANCEL, CANCEL_LAST, RALLY_UNITS
    ProtossSentry,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_GUARDIANSHIELD, HALLUCINATION_ARCHON, HALLUCINATION_COLOSSUS, HALLUCINATION_HIGHTEMPLAR, HALLUCINATION_IMMORTAL, HALLUCINATION_PHOENIX, HALLUCINATION_PROBE, HALLUCINATION_STALKER, HALLUCINATION_VOIDRAY, HALLUCINATION_WARPPRISM, HALLUCINATION_ZEALOT, EFFECT_FORCEFIELD, HALLUCINATION_ORACLE, HALLUCINATION_DISRUPTOR, HALLUCINATION_ADEPT, STOP, RALLY_UNITS, ATTACK
    ProtossStalker,                // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK, EFFECT_BLINK
    ProtossStargate,               // SMART, TRAIN_PHOENIX, TRAIN_CARRIER, TRAIN_VOIDRAY, TRAIN_ORACLE, TRAIN_TEMPEST, CANCEL, CANCEL_LAST, RALLY_UNITS
    ProtossTempest,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_TEMPESTDISRUPTIONBLAST, CANCEL, STOP, ATTACK
    ProtossTemplarArchive,         // RESEARCH_PSISTORM, CANCEL, CANCEL_LAST
    ProtossTwilightCouncil,        // RESEARCH_CHARGE, RESEARCH_BLINK, RESEARCH_ADEPTRESONATINGGLAIVES, CANCEL, CANCEL_LAST
    ProtossVoidRay,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_VOIDRAYPRISMATICALIGNMENT, STOP, ATTACK
    ProtossWarpGate,               // SMART, TRAINWARP_ZEALOT, TRAINWARP_STALKER, TRAINWARP_HIGHTEMPLAR, TRAINWARP_DARKTEMPLAR, TRAINWARP_SENTRY, TRAINWARP_ADEPT, MORPH_GATEWAY
    ProtossWarpPrism,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_WARPPRISMPHASINGMODE, STOP, LOAD, UNLOADALLAT, ATTACK
    ProtossWarpPrismPhasing,       // SMART, MORPH_WARPPRISMTRANSPORTMODE, STOP, LOAD, UNLOADALLAT
    ProtossZealot,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_CHARGE, STOP, RALLY_UNITS, ATTACK

    // Neutral
    NeutralCollapsibleRockTowerDebris,
    NeutralCollapsibleRockTowerDiagonal,
    NeutralCollapsibleRockTowerPushUnit,
    NeutralCollapsibleTerranTowerDebris,
    NeutralCollapsibleTerranTowerDiagonal,
    NeutralCollapsibleTerranTowerPushUnit,
    NeutralCollapsibleTerranTowerPushUnitRampLeft,
    NeutralCollapsibleTerranTowerPushUnitRampRight,
    NeutralCollapsibleTerranTowerRampLeft,
    NeutralCollapsibleTerranTowerRampRight,
    NeutralDebrisRampLeft,
    NeutralDebrisRampRight,
    NeutralDestructibleDebris6x6,
    NeutralDestructibleDebrisRampDiagonalHugeBlur,
    NeutralDestructibleDebrisRampDiagonalHugeUlbr,
    NeutralDestructableRock6x6,
    NeutralDestructibleRockEx1DiagonalHugeBlur,
    NeutralForceField,
    NeutralKarakFemale,
    NeutralLabMineralField,
    NeutralLabMineralField750,
    NeutralMineralField,
    NeutralMineralField750,
    NeutralProtossVespeneGeyser,
    NeutralRichMineralField,
    NeutralRichMineralField750,
    NeutralScantipede,
    NeutralSpacePlatformGeyser,
    NeutralUnbuildableBricksDestructible,
    NeutralUnbuildablePlatesDestructible,
    NeutralUtilityBot,
    NeutralVespeneGeyser,
    NeutralXelNagaTower,
}

impl UnitType {
    /// convert from raw u32 id in protobufs
    pub fn from_id(id: u32) -> Self {
        match id {
            29  => UnitType::TerranArmory,
            31  => UnitType::TerranAutoTurret,
            55  => UnitType::TerranBanshee,
            21  => UnitType::TerranBarracks,
            46  => UnitType::TerranBarracksFlying,
            38  => UnitType::TerranBarracksReactor,
            37  => UnitType::TerranBarracksTechLab,
            57  => UnitType::TerranBattleCruiser,
            24  => UnitType::TerranBunker,
            18  => UnitType::TerranCommandCenter,
            36  => UnitType::TerranCommandCenterFlying,
            692 => UnitType::TerranCyclone,
            22  => UnitType::TerranEngineeringBay,
            27  => UnitType::TerranFactory,
            43  => UnitType::TerranFactoryFlying,
            40  => UnitType::TerranFactoryReactor,
            39  => UnitType::TerranFactoryTechLab,
            30  => UnitType::TerranFusionCore,
            50  => UnitType::TerranGhost,
            26  => UnitType::TerranGhostAcademy,
            53  => UnitType::TerranHellion,
            484 => UnitType::TerranHellionTank,
            689 => UnitType::TerranLiberator,
            734 => UnitType::TerranLiberatorAg,
            51  => UnitType::TerranMarauder,
            48  => UnitType::TerranMarine,
            54  => UnitType::TerranMedivac,
            23  => UnitType::TerranMissileTurret,
            268 => UnitType::TerranMule,
            132 => UnitType::TerranOrbitalCommand,
            134 => UnitType::TerranOrbitalCommandFlying,
            130 => UnitType::TerranPlanetaryFortress,
            56  => UnitType::TerranRaven,
            49  => UnitType::TerranReaper,
            20  => UnitType::TerranRefinery,
            45  => UnitType::TerranScv,
            25  => UnitType::TerranSensorTower,
            33  => UnitType::TerranSiegeTank,
            32  => UnitType::TerranSiegeTankSieged,
            28  => UnitType::TerranStarport,
            44  => UnitType::TerranStarportFlying,
            42  => UnitType::TerranStarportReactor,
            41  => UnitType::TerranStarportTechLab,
            19  => UnitType::TerranSupplyDepot,
            47  => UnitType::TerranSupplyDepotLowered,
            52  => UnitType::TerranThor,
            691 => UnitType::TerranThorAp,
            34  => UnitType::TerranVikingAssault,
            35  => UnitType::TerranVikingFighter,
            498 => UnitType::TerranWidowMine,
            500 => UnitType::TerranWidowMineBurrowed,
            830 => UnitType::TerranKd8Charge,
            58  => UnitType::TerranNuke,
            11  => UnitType::TerranPointDefenseDrone,
            6   => UnitType::TerranReactor,
            5   => UnitType::TerranTechLab,

            9   => UnitType::ZergBaneling,
            115 => UnitType::ZergBanelingBurrowed,
            8   => UnitType::ZergBanelingCocoon,
            96  => UnitType::ZergBanelingNest,
            289 => UnitType::ZergBroodling,
            114 => UnitType::ZergBroodlord,
            113 => UnitType::ZergBroodlordCocoon,
            12  => UnitType::ZergChangeling,
            15  => UnitType::ZergChangelingMarine,
            14  => UnitType::ZergChangelingMarineShield,
            13  => UnitType::ZergChangelingZealot,
            17  => UnitType::ZergChangelingZergling,
            16  => UnitType::ZergChangelingZerglingWings,
            112 => UnitType::ZergCorruptor,
            87  => UnitType::ZergCreepTumor,
            137 => UnitType::ZergCreepTumorBurrowed,
            138 => UnitType::ZergCreepTumorQueen,
            104 => UnitType::ZergDrone,
            116 => UnitType::ZergDroneBurrowed,
            103 => UnitType::ZergEgg,
            90  => UnitType::ZergEvolutionChamber,
            88  => UnitType::ZergExtractor,
            102 => UnitType::ZergGreaterSpire,
            86  => UnitType::ZergHatchery,
            101 => UnitType::ZergHive,
            107 => UnitType::ZergHydralisk,
            117 => UnitType::ZergHydraliskBurrowed,
            91  => UnitType::ZergHydraliskDen,
            94  => UnitType::ZergInfestationPit,
            150 => UnitType::ZergInfestedTerransEgg,
            111 => UnitType::ZergInfestor,
            127 => UnitType::ZergInfestorBurrowed,
            7   => UnitType::ZergInfestorTerran,
            100 => UnitType::ZergLair,
            151 => UnitType::ZergLarva,
            489 => UnitType::ZergLocustMp,
            693 => UnitType::ZergLocustMpFlying,
            504 => UnitType::ZergLurkerDenMp,
            502 => UnitType::ZergLurkerMp,
            503 => UnitType::ZergLurkerMpBurrowed,
            501 => UnitType::ZergLurkerMpEgg,
            108 => UnitType::ZergMutalisk,
            142 => UnitType::ZergNydusCanal,
            95  => UnitType::ZergNydusNetwork,
            106 => UnitType::ZergOverlord,
            128 => UnitType::ZergOverlordCocoon,
            893 => UnitType::ZergOverlordTransport,
            129 => UnitType::ZergOverseer,
            126 => UnitType::ZergQueen,
            125 => UnitType::ZergQueenBurrowed,
            688 => UnitType::ZergRavager,
            687 => UnitType::ZergRavagerCocoon,
            110 => UnitType::ZergRoach,
            118 => UnitType::ZergRoachBurrowed,
            97  => UnitType::ZergRoachWarren,
            89  => UnitType::ZergSpawningPool,
            98  => UnitType::ZergSpineCrawler,
            139 => UnitType::ZergSpineCrawlerUprooted,
            92  => UnitType::ZergSpire,
            99  => UnitType::ZergSporeCrawler,
            140 => UnitType::ZergSporeCrawlerUprooted,
            493 => UnitType::ZergSwarmHostBurrowedMp,
            494 => UnitType::ZergSwarmHostMp,
            892 => UnitType::ZergTransportOverlordCocoon,
            109 => UnitType::ZergUltralisk,
            93  => UnitType::ZergUltraliskCavern,
            499 => UnitType::ZergViper,
            105 => UnitType::ZergZergling,
            119 => UnitType::ZergZerglingBurrowed,
            824 => UnitType::ZergParasiticBombDummy,

            311 => UnitType::ProtossAdept,
            801 => UnitType::ProtossAdeptPhaseShift,
            141 => UnitType::ProtossArchon,
            61  => UnitType::ProtossAssimilator,
            79  => UnitType::ProtossCarrier,
            4   => UnitType::ProtossColossus,
            72  => UnitType::ProtossCyberneticScore,
            69  => UnitType::ProtossDarkShrine,
            76  => UnitType::ProtossDarkTemplar,
            694 => UnitType::ProtossDisruptor,
            733 => UnitType::ProtossDisruptorPhased,
            64  => UnitType::ProtossFleetBeacon,
            63  => UnitType::ProtossForge,
            62  => UnitType::ProtossGateway,
            75  => UnitType::ProtossHighTemplar,
            83  => UnitType::ProtossImmortal,
            85  => UnitType::ProtossInterceptor,
            10  => UnitType::ProtossMothership,
            488 => UnitType::ProtossMothershipCore,
            59  => UnitType::ProtossNexus,
            82  => UnitType::ProtossObserver,
            495 => UnitType::ProtossOracle,
            732 => UnitType::ProtossOracleStasisTrap,
            78  => UnitType::ProtossPhoenix,
            66  => UnitType::ProtossPhotonCannon,
            84  => UnitType::ProtossProbe,
            60  => UnitType::ProtossPylon,
            894 => UnitType::ProtossPylonOvercharged,
            70  => UnitType::ProtossRoboticsBay,
            71  => UnitType::ProtossRoboticsFacility,
            77  => UnitType::ProtossSentry,
            74  => UnitType::ProtossStalker,
            67  => UnitType::ProtossStargate,
            496 => UnitType::ProtossTempest,
            68  => UnitType::ProtossTemplarArchive,
            65  => UnitType::ProtossTwilightCouncil,
            80  => UnitType::ProtossVoidRay,
            133 => UnitType::ProtossWarpGate,
            81  => UnitType::ProtossWarpPrism,
            136 => UnitType::ProtossWarpPrismPhasing,
            73  => UnitType::ProtossZealot,

            490 => UnitType::NeutralCollapsibleRockTowerDebris,
            588 => UnitType::NeutralCollapsibleRockTowerDiagonal,
            561 => UnitType::NeutralCollapsibleRockTowerPushUnit,
            485 => UnitType::NeutralCollapsibleTerranTowerDebris,
            589 => UnitType::NeutralCollapsibleTerranTowerDiagonal,
            562 => UnitType::NeutralCollapsibleTerranTowerPushUnit,
            559 => UnitType::NeutralCollapsibleTerranTowerPushUnitRampLeft,
            560 => UnitType::NeutralCollapsibleTerranTowerPushUnitRampRight,
            590 => UnitType::NeutralCollapsibleTerranTowerRampLeft,
            591 => UnitType::NeutralCollapsibleTerranTowerRampRight,
            486 => UnitType::NeutralDebrisRampLeft,
            487 => UnitType::NeutralDebrisRampRight,
            365 => UnitType::NeutralDestructibleDebris6x6,
            377 => UnitType::NeutralDestructibleDebrisRampDiagonalHugeBlur,
            376 => UnitType::NeutralDestructibleDebrisRampDiagonalHugeUlbr,
            371 => UnitType::NeutralDestructableRock6x6,
            641 => UnitType::NeutralDestructibleRockEx1DiagonalHugeBlur,
            135 => UnitType::NeutralForceField,
            324 => UnitType::NeutralKarakFemale,
            665 => UnitType::NeutralLabMineralField,
            666 => UnitType::NeutralLabMineralField750,
            341 => UnitType::NeutralMineralField,
            483 => UnitType::NeutralMineralField750,
            608 => UnitType::NeutralProtossVespeneGeyser,
            146 => UnitType::NeutralRichMineralField,
            147 => UnitType::NeutralRichMineralField750,
            335 => UnitType::NeutralScantipede,
            343 => UnitType::NeutralSpacePlatformGeyser,
            473 => UnitType::NeutralUnbuildableBricksDestructible,
            474 => UnitType::NeutralUnbuildablePlatesDestructible,
            330 => UnitType::NeutralUtilityBot,
            342 => UnitType::NeutralVespeneGeyser,
            149 => UnitType::NeutralXelNagaTower,

            _ => UnitType::Invalid
        }
    }
}

/// whether the unit is shown on screen or not
#[derive(PartialEq, Copy, Clone)]
pub enum DisplayType {
    /// unit will be visible
    Visible,
    /// unit is represented by a snapshot in fog-of-war
    ///
    /// the actual unit may be in a different location or state
    Snapshot,
    /// unit will be hidden to enemies
    Hidden,
}

impl From<raw::DisplayType> for DisplayType {
    fn from(display: raw::DisplayType) -> Self {
        match display {
            raw::DisplayType::Visible   => DisplayType::Visible,
            raw::DisplayType::Snapshot  => DisplayType::Snapshot,
            raw::DisplayType::Hidden    => DisplayType::Hidden,
        }
    }
}

/// relationship to this player
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Alliance {
    /// unit is owned by this player
    Domestic,
    /// unit is an ally of this player
    Ally,
    /// unit is neutral to this player (usually a non-player unit)
    Neutral,
    /// unit is an enemy of the player
    Enemy,
}

impl From<raw::Alliance> for Alliance {
    fn from(alliance: raw::Alliance) -> Self {
        match alliance {
            raw::Alliance::Domestic => Alliance::Domestic,
            raw::Alliance::Ally     => Alliance::Ally,
            raw::Alliance::Neutral  => Alliance::Neutral,
            raw::Alliance::Enemy    => Alliance::Enemy
        }
    }
}

/// unit cloak state
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CloakState {
    /// unit is invisible to enemies until detected
    Cloaked,
    /// cloaked, but detected
    CloakedDetected,
    /// no cloaking
    NotCloaked,
    /// unable to determine cloak state
    Unknown,
}

impl From<raw::CloakState> for CloakState {
    fn from(cloak: raw::CloakState) -> Self {
        match cloak {
            raw::CloakState::Cloaked => CloakState::Cloaked,
            raw::CloakState::CloakedDetected => CloakState::CloakedDetected,
            raw::CloakState::NotCloaked => CloakState::NotCloaked
        }
    }
}

/// target of a unit order
pub enum UnitOrderTarget {
    /// targets another unit
    UnitTag(Tag),
    /// targets a location
    Location(Point2)
}

/// an order that is active on a unit
pub struct UnitOrder {
    /// ability that triggered the order
    pub ability:                Ability,
    /// target unit of the order
    pub target:                 Option<UnitOrderTarget>,
    /// progress of the order
    pub progress:               f32,
}

impl From<raw::UnitOrder> for UnitOrder {
    /// convert from protobuf data
    fn from(order: raw::UnitOrder) -> Self {
        Self {
            ability: Ability::from_id(order.get_ability_id()),
            target: {
                if order.has_target_unit_tag() {
                    Some(UnitOrderTarget::UnitTag(order.get_target_unit_tag()))
                }
                else if order.has_target_world_space_pos() {
                    let target_pos = order.get_target_world_space_pos();

                    Some(
                        UnitOrderTarget::Location(
                            Point2::new(target_pos.get_x(), target_pos.get_y())
                        )
                    )
                }
                else {
                    None
                }
            },
            progress: order.get_progress()
        }
    }
}

/// a passenger on a transport
pub struct PassengerUnit {
    /// tag of the unit in transport
    pub tag:                    Tag,
    /// health of the unit in transport
    pub health:                 f32,
    /// max health of the unit in transport
    pub health_max:             f32,
    /// shield of the unit in transport
    pub shield:                 f32,
    /// energy of the unit in transport
    pub energy:                 f32,
    /// type of unit in transport
    pub unit_type:              UnitType,
}

impl From<raw::PassengerUnit> for PassengerUnit {
    fn from(passenger: raw::PassengerUnit) -> Self {
        Self {
            tag: passenger.get_tag(),
            health: passenger.get_health(),
            health_max: passenger.get_health_max(),
            shield: passenger.get_shield(),
            energy: passenger.get_energy(),
            unit_type: UnitType::from_id(passenger.get_unit_type())
        }
    }
}
