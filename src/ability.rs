
pub enum Attack {
    Attack,             // Target: Unit, Point.
    AttackBuilding,     // Target: Unit, Point.
    AttackRedirect,     // Target: Unit, Point.
}

pub enum Behavior {
    BuildingAttackOff,  // Target: None.
    BuildingAttackOn,  // Target: None.
    CloakOff,  // Target: None.
    CloakOffBanshee,   // Target: None.
    CloakOffGhost,   // Target: None.
    CloakOn,  // Target: None.
    CloakOnBanshee,   // Target: None.
    CloakOnGhost,   // Target: None.
    GenerateCreepOff,  // Target: None.
    GenerateCreepOn,  // Target: None.
    HoldFireOff,  // Target: None.
    HoldFireOffLurker,  // Target: None.
    HoldFireOn,  // Target: None.
    HoldFireOnGhost,    // Target: None.
    HoldFireOnLurker,  // Target: None.
    PulsarBeamOff,  // Target: None.
    PulsarBeamOn,  // Target: None.
}

pub enum Build {
    Armory,   // Target: Point.
    Assimilator,   // Target: Unit.
    BanelingNest,  // Target: Point.
    Barracks,   // Target: Point.
    Bunker,   // Target: Point.
    CommandCenter,   // Target: Point.
    CreepTumor,  // Target: Point.
    CreepTumorQueen,  // Target: Point.
    CreepTumorTumor,  // Target: Point.
    CyberneticsCore,   // Target: Point.
    DarkShrine,   // Target: Point.
    EngineeringBay,   // Target: Point.
    EvolutionChamber,  // Target: Point.
    Extractor,  // Target: Unit.
    Factory,   // Target: Point.
    FleetBeacon,   // Target: Point.
    Forge,   // Target: Point.
    FusionCore,   // Target: Point.
    Gateway,   // Target: Point.
    GhostAcademy,   // Target: Point.
    Hatchery,  // Target: Point.
    HydraliskDen,  // Target: Point.
    InfestationPit,  // Target: Point.
    Interceptors,  // Target: None.
    MissileTurret,   // Target: Point.
    Nexus,   // Target: Point.
    Nuke,   // Target: None.
    NydusNetwork,  // Target: Point.
    NydusWorm,  // Target: Point.
    PhotonCannon,   // Target: Point.
    Pylon,   // Target: Point.
    Reactor,  // Target: None.
    ReactorBarracks,   // Target: None.
    ReactorFactory,   // Target: None.
    ReactorStarport,   // Target: None.
    Refinery,   // Target: Unit.
    RoachWarren,  // Target: Point.
    RoboticsBay,   // Target: Point.
    RoboticsFacility,   // Target: Point.
    SensorTower,   // Target: Point.
    SpawningPool,  // Target: Point.
    SpineCrawler,  // Target: Point.
    Spire,  // Target: Point.
    SporeCrawler,  // Target: Point.
    StarGate,   // Target: Point.
    Starport,   // Target: Point.
    StasisTrap,  // Target: Point.
    SupplyDepot,   // Target: Point.
    TechLab,  // Target: None.
    TechLabBarracks,   // Target: None.
    TechLabFactory,   // Target: None.
    TechLabStarport,   // Target: None.
    TemplarArchive,   // Target: Point.
    TwilightCouncil,   // Target: Point.
    UltraliskCavern,  // Target: Point.
}

pub enum BurrowDown {
    BurrowDown,  // Target: None.
    Baneling,  // Target: None.
    Drone,  // Target: None.
    Hydralisk,  // Target: None.
    Infestor,  // Target: None.
    Lurker,  // Target: None.
    Queen,  // Target: None.
    Ravager,  // Target: None.
    Roach,  // Target: None.
    SwarmHost,  // Target: None.
    WidowMine,  // Target: None.
    Zergling,  // Target: None.
}

pub enum BurrowUp {
    BurrowUp,  // Target: None.
    Baneling,  // Target: None.
    Drone,  // Target: None.
    Hydralisk,  // Target: None.
    Infestor,  // Target: None.
    Lurker,  // Target: None.
    Queen,  // Target: None.
    Ravager,  // Target: None.
    Roach,  // Target: None.
    SwarmHost,  // Target: None.
    WidowMine,  // Target: None.
    Zergling,  // Target: None.
}

pub enum Cancel {
    Cancel,  // Target: None.
    SlotAddOn,   // Target: None.
    SlotQueue1,   // Target: None.
    SlotQueue5,   // Target: None.
    SlotQueueToSelection,   // Target: None.
    SlotQueuePassive,  // Target: None.
    AdeptPhaseShift,  // Target: None.
    AdeptShadePhaseShift,  // Target: None.
    BarracksAddOn,   // Target: None.
    BuildInProgress,   // Target: None.
    CreepTumor,  // Target: None.
    FactoryAddOn,   // Target: None.
    GravitonBeam,   // Target: None.
    Last,  // Target: None.
    MorphBroodLord,  // Target: None.
    MorphLair,  // Target: None.
    MorphLurker,  // Target: None.
    MorphLurkerDen,  // Target: None.
    MorphMothership,  // Target: None.
    MorphOrbital,  // Target: None.
    MorphOverlordTransport,  // Target: None.
    MorphOverseer,  // Target: None.
    MorphPlanetaryFortress,  // Target: None.
    MorphRavager,  // Target: None.
    Queue1,   // Target: None.
    Queue5,   // Target: None.
    QueueAddOn,   // Target: None.
    QueueToSelection,   // Target: None.
    QueuePassive,  // Target: None.
    QueuePassiveToSelection,  // Target: None.
    SpineCrawlerRoot,  // Target: None.
    StarportAddOn,   // Target: None.
}

pub enum Effect {
    Abduct,  // Target: Unit.
    AdeptPhaseShift,  // Target: Point.
    AutoTurret,  // Target: Point.
    BlindingCloud,  // Target: Point.
    Blink,  // Target: Point.
    BlinkStalker,  // Target: Point.
    CallDownMule,   // Target: Unit, Point.
    CausticSpray,  // Target: Unit.
    Charge,  // Target: Unit.
    ChronoBoost,   // Target: Unit.
    Contaminate,  // Target: Unit.
    CorrosiveBile,  // Target: Point.
    Emp,  // Target: Point.
    Explode,    // Target: None.
    Feedback,   // Target: Unit.
    ForceField,  // Target: Point.
    FungalGrowth,    // Target: Point.
    GhostSnipe,  // Target: Unit.
    GravitonBeam,   // Target: Unit.
    GuardianShield,    // Target: None.
    Heal,   // Target: Unit.
    HunterSeekerMissile,   // Target: Unit.
    ImmortalBarrier,  // Target: None.
    InfestedTerrans,   // Target: Point.
    InjectLarva,   // Target: Unit.
    Kd8Charge,  // Target: Unit, Point.
    LockOn,  // Target: Unit.
    LocustSwoop,  // Target: Point.
    MassRecall,  // Target: Unit.
    MassRecallMothership,  // Target: Unit.
    MassRecallMothershipCore,  // Target: Unit.
    MedivacIgniteAfterBurners,  // Target: None.
    NeuralParasite,   // Target: Unit.
    NukeCallDown,  // Target: Point.
    OracleRevelation,  // Target: Point.
    ParasiticBomb,  // Target: Unit.
    PhotonOvercharge,  // Target: Unit.
    PointDefenseDrone,   // Target: Point.
    PsiStorm,  // Target: Point.
    PurificationNova,  // Target: Point.
    Repair,  // Target: Unit.
    RepairMule,    // Target: Unit.
    RepairScv,   // Target: Unit.
    Salvage,    // Target: None.
    Scan,   // Target: Point.
    ShadowStride,  // Target: Point.
    SpawnChangeling,   // Target: None.
    SpawnLocusts,  // Target: Point.
    Spray,  // Target: Point.
    SprayProtoss,    // Target: Point.
    SprayTerran,    // Target: Point.
    SprayZerg,    // Target: Point.
    Stim,  // Target: None.
    StimMarauder,   // Target: None.
    StimMarine,   // Target: None.
    StimMarineRedirect,  // Target: None.
    SupplyDrop,   // Target: Unit.
    TacticalJump,  // Target: Point.
    TempestDisruptionBlast,  // Target: Point.
    TimeWarp,  // Target: Point.
    Transfusion,  // Target: Unit.
    ViperConsume,  // Target: Unit.
    VoidRayPrismaticalAlignment,  // Target: None.
    WidowMineAttack,  // Target: Unit.
    YamatoGun,   // Target: Unit.
}

pub enum Hallucination {
    Adept,  // Target: None.
    Archon,   // Target: None.
    Colossus,   // Target: None.
    Disruptor,  // Target: None.
    HighTemplar,   // Target: None.
    Immortal,   // Target: None.
    Oracle,  // Target: None.
    Phoenix,   // Target: None.
    Probe,   // Target: None.
    Stalker,   // Target: None.
    VoidRay,   // Target: None.
    WarpPrism,   // Target: None.
    Zealot,   // Target: None.
}

pub enum Halt {
    Halt,  // Target: None.
    Building,   // Target: None.
    TerranBuild,   // Target: None.
}

pub enum Harvest {
    Gather,  // Target: Unit.
    GatherDrone,  // Target: Unit.
    GatherProbe,   // Target: Unit.
    GatherScv,   // Target: Unit.
    Return,  // Target: None.
    ReturnDrone,  // Target: None.
    ReturnMule,   // Target: None.
    ReturnProbe,   // Target: None.
    ReturnScv,   // Target: None.
}

pub enum Land {
    Land,  // Target: Point.
    Barracks,   // Target: Point.
    CommandCenter,   // Target: Point.
    Factory,   // Target: Point.
    OrbitalCommand,  // Target: Point.
    Starport,   // Target: Point.
}

pub enum Lift {
    Lift,  // Target: None.
    Barracks,   // Target: None.
    CommandCenter,   // Target: None.
    Factory,   // Target: None.
    OrbitalCommand,  // Target: None.
    Starport,   // Target: None.
}

pub enum Load {
    Load,  // Target: Unit.
    LoadAll,  // Target: None.
    LoadAllCommandCenter,   // Target: None.
    Bunker,   // Target: Unit.
    Medivac,   // Target: Unit.
}

pub enum Morph {
    Archon,  // Target: None.
    BroodLord,  // Target: None.
    Gateway,  // Target: None.
    GreaterSpire,  // Target: None.
    HellBat,  // Target: None.
    Hellion,  // Target: None.
    Hive,  // Target: None.
    Lair,  // Target: None.
    LiberatorAaMode,  // Target: None.
    LiberatorAgMode,  // Target: Point.
    Lurker,  // Target: None.
    LurkerDen,  // Target: None.
    Mothership,  // Target: None.
    OrbitalCommand,  // Target: None.
    OverlordTransport,  // Target: None.
    Overseer,  // Target: None.
    PlanetaryFortress,  // Target: None.
    Ravager,  // Target: None.
    Root,  // Target: Point.
    SiegeMode,   // Target: None.
    SpineCrawlerRoot,  // Target: Point.
    SpineCrawlerUproot,  // Target: None.
    SporeCrawlerRoot,  // Target: Point.
    SporeCrawlerUproot,  // Target: None.
    SupplyDepotLower,   // Target: None.
    SupplyDepotRaise,   // Target: None.
    ThorExplosiveMode,  // Target: None.
    ThorHighImpactMode,  // Target: None.
    Unsiege,   // Target: None.
    Uproot,  // Target: None.
    VikingAssaultMode,   // Target: None.
    VikingFighterMode,   // Target: None.
    WarpGate,  // Target: None.
    WarpPrismPhasingMode,  // Target: None.
    WarpPrismTransportMode,  // Target: None.
}

pub enum Move {
    Move,    // Target: Unit, Point.
    Patrol,    // Target: Unit, Point.
    RallyBuilding,   // Target: Unit, Point.
    RallyCommandCenter,   // Target: Unit, Point.
    RallyHatcheryUnits,   // Target: Unit, Point.
    RallyHatcheryWorkers,   // Target: Unit, Point.
    RallyMorphingUnit,   // Target: Unit, Point.
    RallyNexus,   // Target: Unit, Point.
    RallyUnits,  // Target: Unit, Point.
    RallyWorkers,  // Target: Unit, Point.
}

pub enum Research {
    AdeptResonatingGlaives,  // Target: None.
    AdvancedBallistics,   // Target: None.
    BansheeCloakingField,   // Target: None.
    BansheeHyperFlightRotors,   // Target: None.
    BattleCruiserWeaponReFit,  // Target: None.
    Blink,  // Target: None.
    Burrow,  // Target: None.
    CentrifugalHooks,  // Target: None.
    Charge,  // Target: None.
    ChitinousPlating,   // Target: None.
    CombatShield,   // Target: None.
    ConcussiveShells,   // Target: None.
    DrillingClaws,   // Target: None.
    ExtendedThermalLance,  // Target: None.
    GlialRegeneration,   // Target: None.
    GraviticBooster,  // Target: None.
    GraviticDrive,  // Target: None.
    GroovedSpines,  // Target: None.
    HighCapacityFuelTanks,   // Target: None.
    HisecAutoTracking,   // Target: None.
    InfernalPreIgniter,   // Target: None.
    InterceptorGravitonCatapult,    // Target: None.
    MagFieldLaunchers,   // Target: None.
    MuscularAugments,  // Target: None.
    NeoSteelFrame,   // Target: None.
    NeuralParasite,  // Target: None.
    PathogenGlands,  // Target: None.
    PersonalCloaking,   // Target: None.
    PhoenixAnionPulseCrystals,    // Target: None.
    PneumatizedCarapace,  // Target: None.
    ProtossAirArmor,  // Target: None.
    ProtossAirArmorLevel1,  // Target: None.
    ProtossAirArmorLevel2,  // Target: None.
    ProtossAirArmorLevel3,  // Target: None.
    ProtossAirWeapons,  // Target: None.
    ProtossAirWeaponsLevel1,  // Target: None.
    ProtossAirWeaponsLevel2,  // Target: None.
    ProtossAirWeaponsLevel3,  // Target: None.
    ProtossGroundArmor,  // Target: None.
    ProtossGroundArmorLevel1,  // Target: None.
    ProtossGroundArmorLevel2,  // Target: None.
    ProtossGroundArmorLevel3,  // Target: None.
    ProtossGroundWeapons,  // Target: None.
    ProtossGroundWeaponsLevel1,  // Target: None.
    ProtossGroundWeaponsLevel2,  // Target: None.
    ProtossGroundWeaponsLevel3,  // Target: None.
    ProtossShields,  // Target: None.
    ProtossShieldsLevel1,  // Target: None.
    ProtossShieldsLevel2,  // Target: None.
    ProtossShieldsLevel3,  // Target: None.
    PsiStorm,  // Target: None.
    RavenCorvidReactor,   // Target: None.
    RavenRecalibratedExplosives,   // Target: None.
    ShadowStrike,  // Target: None.
    Stimpack,   // Target: None.
    TerranInfantryArmor,  // Target: None.
    TerranInfantryArmorLevel1,   // Target: None.
    TerranInfantryArmorLevel2,   // Target: None.
    TerranInfantryArmorLevel3,   // Target: None.
    TerranInfantryWeapons,  // Target: None.
    TerranInfantryWeaponsLevel1,   // Target: None.
    TerranInfantryWeaponsLevel2,   // Target: None.
    TerranInfantryWeaponsLevel3,   // Target: None.
    TerranShipWeapons,  // Target: None.
    TerranShipWeaponsLevel1,   // Target: None.
    TerranShipWeaponsLevel2,   // Target: None.
    TerranShipWeaponsLevel3,   // Target: None.
    TerranStructureArmorUpgrade,   // Target: None.
    TerranVehicleAndShipPlating,  // Target: None.
    TerranVehicleAndShipPlatingLevel1,   // Target: None.
    TerranVehicleAndShipPlatingLevel2,   // Target: None.
    TerranVehicleAndShipPlatingLevel3,   // Target: None.
    TerranVehicleWeapons,  // Target: None.
    TerranVehicleWeaponsLevel1,   // Target: None.
    TerranVehicleWeaponsLevel2,   // Target: None.
    TerranVehicleWeaponsLevel3,   // Target: None.
    TunnelingClaws,   // Target: None.
    WarpGate,  // Target: None.
    ZergFlyerArmor,  // Target: None.
    ZergFlyerArmorLevel1,  // Target: None.
    ZergFlyerArmorLevel2,  // Target: None.
    ZergFlyerArmorLevel3,  // Target: None.
    ZergFlyerAttack,  // Target: None.
    ZergFlyerAttackLevel1,  // Target: None.
    ZergFlyerAttackLevel2,  // Target: None.
    ZergFlyerAttackLevel3,  // Target: None.
    ZergGroundArmor,  // Target: None.
    ZergGroundArmorLevel1,  // Target: None.
    ZergGroundArmorLevel2,  // Target: None.
    ZergGroundArmorLevel3,  // Target: None.
    ZerglingAdrenalGlands,  // Target: None.
    ZerglingMetabolicBoost,  // Target: None.
    ZergMeleeWeapons,  // Target: None.
    ZergMeleeWeaponsLevel1,  // Target: None.
    ZergMeleeWeaponsLevel2,  // Target: None.
    ZergMeleeWeaponsLevel3,  // Target: None.
    ZergMissileWeapons,  // Target: None.
    ZergMissileWeaponsLevel1,  // Target: None.
    ZergMissileWeaponsLevel2,  // Target: None.
    ZergMissileWeaponsLevel3,  // Target: None.
}

pub enum Stop {
    Stop,  // Target: None.
    Building,  // Target: None.
    Cheer,     // Target: None.
    Dance,     // Target: None.
    Redirect,  // Target: None.
}

pub enum TrainWarp {
    Adept,  // Target: Point.
    DarkTemplar,  // Target: Point.
    HighTemplar,  // Target: Point.
    Sentry,  // Target: Point.
    Stalker,  // Target: Point.
    Zealot,  // Target: Point.
}

pub enum Train {
    Adept,   // Target: None.
    Baneling,    // Target: None.
    Banshee,   // Target: None.
    BattleCruiser,   // Target: None.
    Carrier,   // Target: None.
    Colossus,   // Target: None.
    Corruptor,  // Target: None.
    Cyclone,   // Target: None.
    DarkTemplar,   // Target: None.
    Disruptor,   // Target: None.
    Drone,  // Target: None.
    Ghost,   // Target: None.
    HellBat,   // Target: None.
    Hellion,   // Target: None.
    HighTemplar,   // Target: None.
    Hydralisk,  // Target: None.
    Immortal,   // Target: None.
    Infestor,  // Target: None.
    Liberator,   // Target: None.
    Marauder,   // Target: None.
    Marine,   // Target: None.
    Medivac,   // Target: None.
    MothershipCore,  // Target: None.
    Mutalisk,  // Target: None.
    Observer,   // Target: None.
    Oracle,   // Target: None.
    Overlord,  // Target: None.
    Phoenix,   // Target: None.
    Probe,  // Target: None.
    Queen,  // Target: None.
    Raven,   // Target: None.
    Reaper,   // Target: None.
    Roach,  // Target: None.
    Scv,   // Target: None.
    Sentry,   // Target: None.
    SiegeTank,   // Target: None.
    Stalker,   // Target: None.
    SwarmHost,  // Target: None.
    Tempest,   // Target: None.
    Thor,   // Target: None.
    Ultralisk,  // Target: None.
    VikingFighter,   // Target: None.
    Viper,  // Target: None.
    VoidRay,   // Target: None.
    WarpPrism,   // Target: None.
    WidowMine,   // Target: None.
    Zealot,   // Target: None.
    Zergling,  // Target: None.
}

pub enum Unload {
    UnloadAll,  // Target: None.
    UnloadAllAt,  // Target: Unit, Point.
    UnloadAllAtMedivac,   // Target: Unit, Point.
    UnloadAllAtOverlord,  // Target: Unit, Point.
    UnloadAllAtWarpPrism,   // Target: Unit, Point.
    UnloadAllBunker,   // Target: None.
    UnloadAllCommandCenter,   // Target: None.
    UnloadAllNydusNetwork,  // Target: None.
    UnloadAllNydusWorm,  // Target: None.
    UnloadUnitBunker,   // Target: None.
    UnloadUnitCommandCenter,   // Target: None.
    UnloadUnitMedivac,   // Target: None.
    UnloadUnitNydusNetwork,  // Target: None.
    UnloadUnitOverlord,  // Target: None.
    UnloadUnitWarpPrism,   // Target: None.
}

pub enum Ability {
    Invalid,
    Smart,     // Target: Unit, Point.

    Attack(Attack),
    Behavior(Behavior),
    Build(Build),
    BurrowDown(BurrowDown),
    BurrowUp(BurrowUp),
    Cancel(Cancel),
    Effect(Effect),
    Hallucination(Hallucination),
    Halt(Halt),
    Harvest(Harvest)

    HoldPosition,    // Target: None.

    Land(Land),
    Lift(Lift),
    Load(Load),
    Morph(Morph),
    Move(Move),
    Research(Research),

    ScanMove,    // Target: Unit, Point.

    Stop(Stop),
    TrainWarp(TrainWarp),
    Train(Train),
    Unload(Unload),
}
