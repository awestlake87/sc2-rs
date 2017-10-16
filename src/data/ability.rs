
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum Ability {
    Invalid,
    Smart,     // Target: Unit, Point.

    Attack,  // Target: Unit, Point.
    AttackAttack,    // Target: Unit, Point.
    AttackAttackBuilding,  // Target: Unit, Point.
    AttackRedirect,  // Target: Unit, Point.

    BehaviorBuildingAttackOff,  // Target: None.
    BehaviorBuildingAttackOn,  // Target: None.
    BehaviorCloakOff,  // Target: None.
    BehaviorCloakOffBanshee,   // Target: None.
    BehaviorCloakOffGhost,   // Target: None.
    BehaviorCloakOn,  // Target: None.
    BehaviorCloakOnBanshee,   // Target: None.
    BehaviorCloakOnGhost,   // Target: None.
    BehaviorGenerateCreepOff,  // Target: None.
    BehaviorGenerateCreepOn,  // Target: None.
    BehaviorHoldFireOff,  // Target: None.
    BehaviorHoldFireOffLurker,  // Target: None.
    BehaviorHoldFireOn,  // Target: None.
    BehaviorHoldFireOnGhost,    // Target: None.
    BehaviorHoldFireOnLurker,  // Target: None.
    BehaviorPulsarBeamOff,  // Target: None.
    BehaviorPulsarBeamOn,  // Target: None.

    BuildArmory,   // Target: Point.
    BuildAssimilator,   // Target: Unit.
    BuildBanelingNest,  // Target: Point.
    BuildBarracks,   // Target: Point.
    BuildBunker,   // Target: Point.
    BuildCommandCenter,   // Target: Point.
    BuildCreepTumor,  // Target: Point.
    BuildCreepTumorQueen,  // Target: Point.
    BuildCreepTumorTumor,  // Target: Point.
    BuildCyberneticsCore,   // Target: Point.
    BuildDarkShrine,   // Target: Point.
    BuildEngineeringBay,   // Target: Point.
    BuildEvolutionChamber,  // Target: Point.
    BuildExtractor,  // Target: Unit.
    BuildFactory,   // Target: Point.
    BuildFleetBeacon,   // Target: Point.
    BuildForge,   // Target: Point.
    BuildFusionCore,   // Target: Point.
    BuildGateway,   // Target: Point.
    BuildGhostAcademy,   // Target: Point.
    BuildHatchery,  // Target: Point.
    BuildHydraliskDen,  // Target: Point.
    BuildInfestationPit,  // Target: Point.
    BuildInterceptors,  // Target: None.
    BuildMissileTurret,   // Target: Point.
    BuildNexus,   // Target: Point.
    BuildNuke,   // Target: None.
    BuildNydusNetwork,  // Target: Point.
    BuildNydusWorm,  // Target: Point.
    BuildPhotonCannon,   // Target: Point.
    BuildPylon,   // Target: Point.
    BuildReactor,  // Target: None.
    BuildReactorBarracks,   // Target: None.
    BuildReactorFactory,   // Target: None.
    BuildReactorStarport,   // Target: None.
    BuildRefinery,   // Target: Unit.
    BuildRoachWarren,  // Target: Point.
    BuildRoboticsBay,   // Target: Point.
    BuildRoboticsFacility,   // Target: Point.
    BuildSensorTower,   // Target: Point.
    BuildSpawningPool,  // Target: Point.
    BuildSpineCrawler,  // Target: Point.
    BuildSpire,  // Target: Point.
    BuildSporeCrawler,  // Target: Point.
    BuildStarGate,   // Target: Point.
    BuildStarport,   // Target: Point.
    BuildStasisTrap,  // Target: Point.
    BuildSupplyDepot,   // Target: Point.
    BuildTechLab,  // Target: None.
    BuildTechLabBarracks,   // Target: None.
    BuildTechLabFactory,   // Target: None.
    BuildTechLabStarport,   // Target: None.
    BuildTemplarArchive,   // Target: Point.
    BuildTwilightCouncil,   // Target: Point.
    BuildUltraliskCavern,  // Target: Point.

    BurrowDown,  // Target: None.
    BurrowDownBaneling,  // Target: None.
    BurrowDownDrone,  // Target: None.
    BurrowDownHydralisk,  // Target: None.
    BurrowDownInfestor,  // Target: None.
    BurrowDownLurker,  // Target: None.
    BurrowDownQueen,  // Target: None.
    BurrowDownRavager,  // Target: None.
    BurrowDownRoach,  // Target: None.
    BurrowDownSwarmHost,  // Target: None.
    BurrowDownWidowMine,  // Target: None.
    BurrowDownZergling,  // Target: None.

    BurrowUp,  // Target: None.
    BurrowUpBaneling,  // Target: None.
    BurrowUpDrone,  // Target: None.
    BurrowUpHydralisk,  // Target: None.
    BurrowUpInfestor,  // Target: None.
    BurrowUpLurker,  // Target: None.
    BurrowUpQueen,  // Target: None.
    BurrowUpRavager,  // Target: None.
    BurrowUpRoach,  // Target: None.
    BurrowUpSwarmHost,  // Target: None.
    BurrowUpWidowMine,  // Target: None.
    BurrowUpZergling,  // Target: None.

    Cancel,  // Target: None.
    CancelSlotAddOn,   // Target: None.
    CancelSlotQueue1,   // Target: None.
    CancelSlotQueue5,   // Target: None.
    CancelSlotQueueCancelToSelection,   // Target: None.
    CancelSlotQueuePassive,  // Target: None.
    CancelAdeptPhaseShift,  // Target: None.
    CancelAdeptShadePhaseShift,  // Target: None.
    CancelBarracksAddOn,   // Target: None.
    CancelBuildInProgress,   // Target: None.
    CancelCreepTumor,  // Target: None.
    CancelFactoryAddOn,   // Target: None.
    CancelGravitonBeam,   // Target: None.
    CancelLast,  // Target: None.
    CancelMorphBroodLord,  // Target: None.
    CancelMorphLair,  // Target: None.
    CancelMorphLurker,  // Target: None.
    CancelMorphLurkerDen,  // Target: None.
    CancelMorphMothership,  // Target: None.
    CancelMorphOrbital,  // Target: None.
    CancelMorphOverlordTransport,  // Target: None.
    CancelMorphOverseer,  // Target: None.
    CancelMorphPlanetaryFortress,  // Target: None.
    CancelMorphRavager,  // Target: None.
    CancelQueue1,   // Target: None.
    CancelQueue5,   // Target: None.
    CancelQueueAddOn,   // Target: None.
    CancelQueueCancelToSelection,   // Target: None.
    CancelQueuePassive,  // Target: None.
    CancelQueuePassiveCancelTOSelection,  // Target: None.
    CancelSpineCrawlerRoot,  // Target: None.
    CancelStarportAddOn,   // Target: None.

    EffectAbduct,  // Target: Unit.
    EffectAdeptPhaseShift,  // Target: Point.
    EffectAutoTurret,  // Target: Point.
    EffectBlindingCloud,  // Target: Point.
    EffectBlink,  // Target: Point.
    EffectBlinkStalker,  // Target: Point.
    EffectCallDownMule,   // Target: Unit, Point.
    EffectCausticSpray,  // Target: Unit.
    EffectCharge,  // Target: Unit.
    EffectChronoBoost,   // Target: Unit.
    EffectContaminate,  // Target: Unit.
    EffectCorrosiveBile,  // Target: Point.
    EffectEmp,  // Target: Point.
    EffectExplode,    // Target: None.
    EffectFeedback,   // Target: Unit.
    EffectForceField,  // Target: Point.
    EffectFungalGrowth,    // Target: Point.
    EffectGhostSnipe,  // Target: Unit.
    EffectGravitonBeam,   // Target: Unit.
    EffectGuardianShield,    // Target: None.
    EffectHeal,   // Target: Unit.
    EffectHunterSeekerMissile,   // Target: Unit.
    EffectImmortalBarrier,  // Target: None.
    EffectInfestedTerrans,   // Target: Point.
    EffectInjectLarva,   // Target: Unit.
    EffectKd8Charge,  // Target: Unit, Point.
    EffectLockOn,  // Target: Unit.
    EffectLocustSwoop,  // Target: Point.
    EffectMassRecall,  // Target: Unit.
    EffectMassRecallMothership,  // Target: Unit.
    EffectMassRecallMothershipCore,  // Target: Unit.
    EffectMedivacIgniteAfterBurners,  // Target: None.
    EffectNeuralParasite,   // Target: Unit.
    EffectNukeCallDown,  // Target: Point.
    EffectOracleRevelation,  // Target: Point.
    EffectParasiticBomb,  // Target: Unit.
    EffectPhotonOvercharge,  // Target: Unit.
    EffectPointDefenseDrone,   // Target: Point.
    EffectPsiStorm,  // Target: Point.
    EffectPurificationNova,  // Target: Point.
    EffectRepair,  // Target: Unit.
    EffectRepairMule,    // Target: Unit.
    EffectRepairScv,   // Target: Unit.
    EffectSalvage,    // Target: None.
    EffectScan,   // Target: Point.
    EffectShadowStride,  // Target: Point.
    EffectSpawnChangeling,   // Target: None.
    EffectSpawnLocusts,  // Target: Point.
    EffectSpray,  // Target: Point.
    EffectSprayProtoss,    // Target: Point.
    EffectSprayTerran,    // Target: Point.
    EffectSprayZerg,    // Target: Point.
    EffectStim,  // Target: None.
    EffectStimMarauder,   // Target: None.
    EffectStimMarine,   // Target: None.
    EffectStimMarineRedirect,  // Target: None.
    EffectSupplyDrop,   // Target: Unit.
    EffectTacticalJump,  // Target: Point.
    EffectTempestDisruptionBlast,  // Target: Point.
    EffectTimeWarp,  // Target: Point.
    EffectTransfusion,  // Target: Unit.
    EffectViperConsume,  // Target: Unit.
    EffectVoidRayPrismaticAlignment,  // Target: None.
    EffectWidowMineAttack,  // Target: Unit.
    EffectYamatoGun,   // Target: Unit.

    HallucinationAdept,  // Target: None.
    HallucinationArchon,   // Target: None.
    HallucinationColossus,   // Target: None.
    HallucinationDisruptor,  // Target: None.
    HallucinationHighTemplar,   // Target: None.
    HallucinationImmortal,   // Target: None.
    HallucinationOracle,  // Target: None.
    HallucinationPhoenix,   // Target: None.
    HallucinationProbe,   // Target: None.
    HallucinationStalker,   // Target: None.
    HallucinationVoidRay,   // Target: None.
    HallucinationWarpPrism,   // Target: None.
    HallucinationZealot,   // Target: None.

    Halt,  // Target: None.
    HaltBuilding,   // Target: None.
    HaltTerranBuild,   // Target: None.

    HarvestGather,  // Target: Unit.
    HarvestGatherDrone,  // Target: Unit.
    HarvestGatherProbe,   // Target: Unit.
    HarvestGatherScv,   // Target: Unit.
    HarvestReturn,  // Target: None.
    HarvestReturnDrone,  // Target: None.
    HarvestReturnMule,   // Target: None.
    HarvestReturnProbe,   // Target: None.
    HarvestReturnScv,   // Target: None.

    HoldPosition,    // Target: None.

    Land,  // Target: Point.
    LandBarracks,   // Target: Point.
    LandCommandCenter,   // Target: Point.
    LandFactory,   // Target: Point.
    LandOrbitalCommand,  // Target: Point.
    LandStarport,   // Target: Point.

    Lift,  // Target: None.
    LiftBarracks,   // Target: None.
    LiftCommandCenter,   // Target: None.
    LiftFactory,   // Target: None.
    LiftOrbitalCommand,  // Target: None.
    LiftStarport,   // Target: None.

    Load,  // Target: Unit.
    LoadAll,  // Target: None.
    LoadAllCommandCenter,   // Target: None.
    LoadBunker,   // Target: Unit.
    LoadMedivac,   // Target: Unit.

    MorphArchon,  // Target: None.
    MorphBroodLord,  // Target: None.
    MorphGateway,  // Target: None.
    MorphGreaterSpire,  // Target: None.
    MorphHellbat,  // Target: None.
    MorphHellion,  // Target: None.
    MorphHive,  // Target: None.
    MorphLair,  // Target: None.
    MorphLiberatorAaMode,  // Target: None.
    MorphLiberatorAgMode,  // Target: Point.
    MorphLurker,  // Target: None.
    MorphLurkerDen,  // Target: None.
    MorphMothership,  // Target: None.
    MorphOrbitalCommand,  // Target: None.
    MorphOverlordTransport,  // Target: None.
    MorphOverseer,  // Target: None.
    MorphPlanetaryFortress,  // Target: None.
    MorphRavager,  // Target: None.
    MorphRoot,  // Target: Point.
    MorphSiegeMode,   // Target: None.
    MorphSpineCrawlerRoot,  // Target: Point.
    MorphSpineCrawlerUproot,  // Target: None.
    MorphSporeCrawlerRoot,  // Target: Point.
    MorphSporeCrawlerUproot,  // Target: None.
    MorphSupplyDepotLower,   // Target: None.
    MorphSupplyDepotRaise,   // Target: None.
    MorphThorExplosiveMode,  // Target: None.
    MorphThorHighImpactMode,  // Target: None.
    MorphUnsiege,   // Target: None.
    MorphUproot,  // Target: None.
    MorphVikingAssaultMode,   // Target: None.
    MorphVikingFighterMode,   // Target: None.
    MorphWarpGate,  // Target: None.
    MorphWarpPrismPhasingMode,  // Target: None.
    MorphWarpPrismTransportMode,  // Target: None.

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
    ResearchAdeptResonatingGlaives,  // Target: None.
    ResearchAdvancedBallistics,   // Target: None.
    ResearchBansheeCloakingField,   // Target: None.
    ResearchBansheeHyperFlightRotors,   // Target: None.
    ResearchBattleCruiserWeaponRefit,  // Target: None.
    ResearchBlink,  // Target: None.
    ResearchBurrow,  // Target: None.
    ResearchCentrifugalHooks,  // Target: None.
    ResearchCharge,  // Target: None.
    ResearchChitinousPlating,   // Target: None.
    ResearchCombatShield,   // Target: None.
    ResearchConcussiveShells,   // Target: None.
    ResearchDrillingClaws,   // Target: None.
    ResearchExtendedThermalLance,  // Target: None.
    ResearchGlialRegeneration,   // Target: None.
    ResearchGraviticBooster,  // Target: None.
    ResearchGraviticDrive,  // Target: None.
    ResearchGroovedSpines,  // Target: None.
    ResearchHighCapacityFuelTanks,   // Target: None.
    ResearchHisecAutoTracking,   // Target: None.
    ResearchInfernalPreIgniter,   // Target: None.
    ResearchInterceptorGravitonCatapult,    // Target: None.
    ResearchMagFieldLaunchers,   // Target: None.
    ResearchMuscularAugments,  // Target: None.
    ResearchNeoSteelFrame,   // Target: None.
    ResearchNeuralParasite,  // Target: None.
    ResearchPathogenGlands,  // Target: None.
    ResearchPersonalCloaking,   // Target: None.
    ResearchPhoenixAnionPulseCrystals,    // Target: None.
    ResearchPneumatizedCarapace,  // Target: None.
    ResearchProtossAirArmor,  // Target: None.
    ResearchProtossAirArmorLevel1,  // Target: None.
    ResearchProtossAirArmorLevel2,  // Target: None.
    ResearchProtossAirArmorLevel3,  // Target: None.
    ResearchProtossAirWeapons,  // Target: None.
    ResearchProtossAirWeaponsLevel1,  // Target: None.
    ResearchProtossAirWeaponsLevel2,  // Target: None.
    ResearchProtossAirWeaponsLevel3,  // Target: None.
    ResearchProtossGroundArmor,  // Target: None.
    ResearchProtossGroundArmorLevel1,  // Target: None.
    ResearchProtossGroundArmorLevel2,  // Target: None.
    ResearchProtossGroundArmorLevel3,  // Target: None.
    ResearchProtossGroundWeapons,  // Target: None.
    ResearchProtossGroundWeaponsLevel1,  // Target: None.
    ResearchProtossGroundWeaponsLevel2,  // Target: None.
    ResearchProtossGroundWeaponsLevel3,  // Target: None.
    ResearchProtossShields,  // Target: None.
    ResearchProtossShieldsLevel1,  // Target: None.
    ResearchProtossShieldsLevel2,  // Target: None.
    ResearchProtossShieldsLevel3,  // Target: None.
    ResearchPsiStorm,  // Target: None.
    ResearchRavenCorvidReactor,   // Target: None.
    ResearchRavenRecalibratedExplosives,   // Target: None.
    ResearchShadowStrike,  // Target: None.
    ResearchStimpack,   // Target: None.
    ResearchTerranInfantryArmor,  // Target: None.
    ResearchTerranInfantryArmorLevel1,   // Target: None.
    ResearchTerranInfantryArmorLevel2,   // Target: None.
    ResearchTerranInfantryArmorLevel3,   // Target: None.
    ResearchTerranInfantryWeapons,  // Target: None.
    ResearchTerranInfantryWeaponsLevel1,   // Target: None.
    ResearchTerranInfantryWeaponsLevel2,   // Target: None.
    ResearchTerranInfantryWeaponsLevel3,   // Target: None.
    ResearchTerranShipWeapons,  // Target: None.
    ResearchTerranShipWeaponsLevel1,   // Target: None.
    ResearchTerranShipWeaponsLevel2,   // Target: None.
    ResearchTerranShipWeaponsLevel3,   // Target: None.
    ResearchTerranStructureArmorUpgrade,   // Target: None.
    ResearchTerranVehicleAndShipPlating,  // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel1,   // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel2,   // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel3,   // Target: None.
    ResearchTerranVehicleWeapons,  // Target: None.
    ResearchTerranVehicleWeaponsLevel1,   // Target: None.
    ResearchTerranVehicleWeaponsLevel2,   // Target: None.
    ResearchTerranVehicleWeaponsLevel3,   // Target: None.
    ResearchTunnelingClaws,   // Target: None.
    ResearchWarpGate,  // Target: None.
    ResearchZergFlyerArmor,  // Target: None.
    ResearchZergFlyerArmorLevel1,  // Target: None.
    ResearchZergFlyerArmorLevel2,  // Target: None.
    ResearchZergFlyerArmorLevel3,  // Target: None.
    ResearchZergFlyerAttack,  // Target: None.
    ResearchZergFlyerAttackLevel1,  // Target: None.
    ResearchZergFlyerAttackLevel2,  // Target: None.
    ResearchZergFlyerAttackLevel3,  // Target: None.
    ResearchZergGroundArmor,  // Target: None.
    ResearchZergGroundArmorLevel1,  // Target: None.
    ResearchZergGroundArmorLevel2,  // Target: None.
    ResearchZergGroundArmorLevel3,  // Target: None.
    ResearchZerglingAdrenalGlands,  // Target: None.
    ResearchZerglingMetabolicBoost,  // Target: None.
    ResearchZergMeleeWeapons,  // Target: None.
    ResearchZergMeleeWeaponsLevel1,  // Target: None.
    ResearchZergMeleeWeaponsLevel2,  // Target: None.
    ResearchZergMeleeWeaponsLevel3,  // Target: None.
    ResearchZergMissileWeapons,  // Target: None.
    ResearchZergMissileWeaponsLevel1,  // Target: None.
    ResearchZergMissileWeaponsLevel2,  // Target: None.
    ResearchZergMissileWeaponsLevel3,  // Target: None.

    ScanMove,    // Target: Unit, Point.

    Stop,  // Target: None.
    StopBuilding,  // Target: None.
    StopCheer,     // Target: None.
    StopDance,     // Target: None.
    StopRedirect,  // Target: None.
    StopStop,     // Target: None.

    TrainWarpAdept,  // Target: Point.
    TrainWarpDarkTemplar,  // Target: Point.
    TrainWarpHighTemplar,  // Target: Point.
    TrainWarpSentry,  // Target: Point.
    TrainWarpStalker,  // Target: Point.
    TrainWarpZealot,  // Target: Point.

    TrainAdept,   // Target: None.
    TrainBaneling,    // Target: None.
    TrainBanshee,   // Target: None.
    TrainBattleCruiser,   // Target: None.
    TrainCarrier,   // Target: None.
    TrainColossus,   // Target: None.
    TrainCorruptor,  // Target: None.
    TrainCyclone,   // Target: None.
    TrainDarkTemplar,   // Target: None.
    TrainDisruptor,   // Target: None.
    TrainDrone,  // Target: None.
    TrainGhost,   // Target: None.
    TrainHellbat,   // Target: None.
    TrainHellion,   // Target: None.
    TrainHighTemplar,   // Target: None.
    TrainHydralisk,  // Target: None.
    TrainImmortal,   // Target: None.
    TrainInfestor,  // Target: None.
    TrainLiberator,   // Target: None.
    TrainMarauder,   // Target: None.
    TrainMarine,   // Target: None.
    TrainMedivac,   // Target: None.
    TrainMothershipCore,  // Target: None.
    TrainMutalisk,  // Target: None.
    TrainObserver,   // Target: None.
    TrainOracle,   // Target: None.
    TrainOverlord,  // Target: None.
    TrainPhoenix,   // Target: None.
    TrainProbe,  // Target: None.
    TrainQueen,  // Target: None.
    TrainRaven,   // Target: None.
    TrainReaper,   // Target: None.
    TrainRoach,  // Target: None.
    TrainScv,   // Target: None.
    TrainSentry,   // Target: None.
    TrainSiegeTank,   // Target: None.
    TrainStalker,   // Target: None.
    TrainSwarmHost,  // Target: None.
    TrainTempest,   // Target: None.
    TrainThor,   // Target: None.
    TrainUltralisk,  // Target: None.
    TrainVikingFighter,   // Target: None.
    TrainViper,  // Target: None.
    TrainVoidRay,   // Target: None.
    TrainWarpPrism,   // Target: None.
    TrainWidowMine,   // Target: None.
    TrainZealot,   // Target: None.
    TrainZergling,  // Target: None.

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

impl Ability {
    pub fn from_id(id: u32) -> Self {
        match id {
            1 => Ability::Smart,

            3674 => Ability::Attack,
            23 => Ability::AttackAttack,
            2048 => Ability::AttackAttackBuilding,
            1682 => Ability::AttackRedirect,

            2082 => Ability::BehaviorBuildingAttackOff,
            2081 => Ability::BehaviorBuildingAttackOn,
            3677 => Ability::BehaviorCloakOff,
            393 => Ability::BehaviorCloakOffBanshee,
            383 => Ability::BehaviorCloakOffGhost,
            3676 => Ability::BehaviorCloakOn,
            392 => Ability::BehaviorCloakOnBanshee,
            382 => Ability::BehaviorCloakOnGhost,
            1693 => Ability::BehaviorGenerateCreepOff,
            1692 => Ability::BehaviorGenerateCreepOn,
            3689 => Ability::BehaviorHoldFireOff,
            2552 => Ability::BehaviorHoldFireOffLurker,
            3688 => Ability::BehaviorHoldFireOn,
            36 => Ability::BehaviorHoldFireOnGhost,
            2550 => Ability::BehaviorHoldFireOnLurker,
            2376 => Ability::BehaviorPulsarBeamOff,
            2375 => Ability::BehaviorPulsarBeamOn,

            331 => Ability::BuildArmory,
            882 => Ability::BuildAssimilator,
            1162 => Ability::BuildBanelingNest,
            321 => Ability::BuildBarracks,
            324 => Ability::BuildBunker,
            318 => Ability::BuildCommandCenter,
            3691 => Ability::BuildCreepTumor,
            1694 => Ability::BuildCreepTumorQueen,
            1733 => Ability::BuildCreepTumorTumor,
            894 => Ability::BuildCyberneticsCore,
            891 => Ability::BuildDarkShrine,
            322 => Ability::BuildEngineeringBay,
            1156 => Ability::BuildEvolutionChamber,
            1154 => Ability::BuildExtractor,
            328 => Ability::BuildFactory,
            885 => Ability::BuildFleetBeacon,
            884 => Ability::BuildForge,
            333 => Ability::BuildFusionCore,
            883 => Ability::BuildGateway,
            327 => Ability::BuildGhostAcademy,
            1152 => Ability::BuildHatchery,
            1157 => Ability::BuildHydraliskDen,
            1160 => Ability::BuildInfestationPit,
            1042 => Ability::BuildInterceptors,
            323 => Ability::BuildMissileTurret,
            880 => Ability::BuildNexus,
            710 => Ability::BuildNuke,
            1161 => Ability::BuildNydusNetwork,
            1768 => Ability::BuildNydusWorm,
            887 => Ability::BuildPhotonCannon,
            881 => Ability::BuildPylon,
            3683 => Ability::BuildReactor,
            422 => Ability::BuildReactorBarracks,
            455 => Ability::BuildReactorFactory,
            488 => Ability::BuildReactorStarport,
            320 => Ability::BuildRefinery,
            1165 => Ability::BuildRoachWarren,
            892 => Ability::BuildRoboticsBay,
            893 => Ability::BuildRoboticsFacility,
            326 => Ability::BuildSensorTower,
            1155 => Ability::BuildSpawningPool,
            1166 => Ability::BuildSpineCrawler,
            1158 => Ability::BuildSpire,
            1167 => Ability::BuildSporeCrawler,
            889 => Ability::BuildStarGate,
            329 => Ability::BuildStarport,
            2505 => Ability::BuildStasisTrap,
            319 => Ability::BuildSupplyDepot,
            3682 => Ability::BuildTechLab,
            421 => Ability::BuildTechLabBarracks,
            454 => Ability::BuildTechLabFactory,
            487 => Ability::BuildTechLabStarport,
            890 => Ability::BuildTemplarArchive,
            886 => Ability::BuildTwilightCouncil,
            1159 => Ability::BuildUltraliskCavern,

            3661 => Ability::BurrowDown,
            1374 => Ability::BurrowDownBaneling,
            1378 => Ability::BurrowDownDrone,
            1382 => Ability::BurrowDownHydralisk,
            1444 => Ability::BurrowDownInfestor,
            2108 => Ability::BurrowDownLurker,
            1433 => Ability::BurrowDownQueen,
            2340 => Ability::BurrowDownRavager,
            1386 => Ability::BurrowDownRoach,
            2014 => Ability::BurrowDownSwarmHost,
            2095 => Ability::BurrowDownWidowMine,
            1390 => Ability::BurrowDownZergling,

            3662 => Ability::BurrowUp,
            1376 => Ability::BurrowUpBaneling,
            1380 => Ability::BurrowUpDrone,
            1384 => Ability::BurrowUpHydralisk,
            1446 => Ability::BurrowUpInfestor,
            2110 => Ability::BurrowUpLurker,
            1435 => Ability::BurrowUpQueen,
            2342 => Ability::BurrowUpRavager,
            1388 => Ability::BurrowUpRoach,
            2016 => Ability::BurrowUpSwarmHost,
            2097 => Ability::BurrowUpWidowMine,
            1392 => Ability::BurrowUpZergling,

            3659 => Ability::Cancel,
            313 => Ability::CancelSlotAddOn,
            305 => Ability::CancelSlotQueue1,
            307 => Ability::CancelSlotQueue5,
            309 => Ability::CancelSlotQueueCancelToSelection,
            1832 => Ability::CancelSlotQueuePassive,
            2594 => Ability::CancelAdeptPhaseShift,
            2596 => Ability::CancelAdeptShadePhaseShift,
            451 => Ability::CancelBarracksAddOn,
            314 => Ability::CancelBuildInProgress,
            1763 => Ability::CancelCreepTumor,
            484 => Ability::CancelFactoryAddOn,
            174 => Ability::CancelGravitonBeam,
            3671 => Ability::CancelLast,
            1373 => Ability::CancelMorphBroodLord,
            1217 => Ability::CancelMorphLair,
            2333 => Ability::CancelMorphLurker,
            2113 => Ability::CancelMorphLurkerDen,
            1848 => Ability::CancelMorphMothership,
            1517 => Ability::CancelMorphOrbital,
            2709 => Ability::CancelMorphOverlordTransport,
            1449 => Ability::CancelMorphOverseer,
            1451 => Ability::CancelMorphPlanetaryFortress,
            2331 => Ability::CancelMorphRavager,
            304 => Ability::CancelQueue1,
            306 => Ability::CancelQueue5,
            312 => Ability::CancelQueueAddOn,
            308 => Ability::CancelQueueCancelToSelection,
            1831 => Ability::CancelQueuePassive,
            1833 => Ability::CancelQueuePassiveCancelTOSelection,
            1730 => Ability::CancelSpineCrawlerRoot,
            517 => Ability::CancelStarportAddOn,

            2067 => Ability::EffectAbduct,
            2544 => Ability::EffectAdeptPhaseShift,
            1764 => Ability::EffectAutoTurret,
            2063 => Ability::EffectBlindingCloud,
            3687 => Ability::EffectBlink,
            1442 => Ability::EffectBlinkStalker,
            171 => Ability::EffectCallDownMule,
            2324 => Ability::EffectCausticSpray,
            1819 => Ability::EffectCharge,
            261 => Ability::EffectChronoBoost,
            1825 => Ability::EffectContaminate,
            2338 => Ability::EffectCorrosiveBile,
            1628 => Ability::EffectEmp,
            42 => Ability::EffectExplode,
            140 => Ability::EffectFeedback,
            1526 => Ability::EffectForceField,
            74 => Ability::EffectFungalGrowth,
            2714 => Ability::EffectGhostSnipe,
            173 => Ability::EffectGravitonBeam,
            76 => Ability::EffectGuardianShield,
            386 => Ability::EffectHeal,
            169 => Ability::EffectHunterSeekerMissile,
            2328 => Ability::EffectImmortalBarrier,
            247 => Ability::EffectInfestedTerrans,
            251 => Ability::EffectInjectLarva,
            2588 => Ability::EffectKd8Charge,
            2350 => Ability::EffectLockOn,
            2387 => Ability::EffectLocustSwoop,
            3686 => Ability::EffectMassRecall,
            2368 => Ability::EffectMassRecallMothership,
            1974 => Ability::EffectMassRecallMothershipCore,
            2116 => Ability::EffectMedivacIgniteAfterBurners,
            249 => Ability::EffectNeuralParasite,
            1622 => Ability::EffectNukeCallDown,
            2146 => Ability::EffectOracleRevelation,
            2542 => Ability::EffectParasiticBomb,
            2162 => Ability::EffectPhotonOvercharge,
            144 => Ability::EffectPointDefenseDrone,
            1036 => Ability::EffectPsiStorm,
            2346 => Ability::EffectPurificationNova,
            3685 => Ability::EffectRepair,
            78 => Ability::EffectRepairMule,
            316 => Ability::EffectRepairScv,
            32 => Ability::EffectSalvage,
            399 => Ability::EffectScan,
            2700 => Ability::EffectShadowStride,
            181 => Ability::EffectSpawnChangeling,
            2704 => Ability::EffectSpawnLocusts,
            3684 => Ability::EffectSpray,
            30 => Ability::EffectSprayProtoss,
            26 => Ability::EffectSprayTerran,
            28 => Ability::EffectSprayZerg,
            3675 => Ability::EffectStim,
            253 => Ability::EffectStimMarauder,
            380 => Ability::EffectStimMarine,
            1683 => Ability::EffectStimMarineRedirect,
            255 => Ability::EffectSupplyDrop,
            2358 => Ability::EffectTacticalJump,
            2698 => Ability::EffectTempestDisruptionBlast,
            2244 => Ability::EffectTimeWarp,
            1664 => Ability::EffectTransfusion,
            2073 => Ability::EffectViperConsume,
            2393 => Ability::EffectVoidRayPrismaticAlignment,
            2099 => Ability::EffectWidowMineAttack,
            401 => Ability::EffectYamatoGun,

            2391 => Ability::HallucinationAdept,
            146 => Ability::HallucinationArchon,
            148 => Ability::HallucinationColossus,
            2389 => Ability::HallucinationDisruptor,
            150 => Ability::HallucinationHighTemplar,
            152 => Ability::HallucinationImmortal,
            2114 => Ability::HallucinationOracle,
            154 => Ability::HallucinationPhoenix,
            156 => Ability::HallucinationProbe,
            158 => Ability::HallucinationStalker,
            160 => Ability::HallucinationVoidRay,
            162 => Ability::HallucinationWarpPrism,
            164 => Ability::HallucinationZealot,

            3660 => Ability::Halt,
            315 => Ability::HaltBuilding,
            348 => Ability::HaltTerranBuild,

            3666 => Ability::HarvestGather,
            1183 => Ability::HarvestGatherDrone,
            298 => Ability::HarvestGatherProbe,
            295 => Ability::HarvestGatherScv,
            3667 => Ability::HarvestReturn,
            1184 => Ability::HarvestReturnDrone,
            167 => Ability::HarvestReturnMule,
            299 => Ability::HarvestReturnProbe,
            296 => Ability::HarvestReturnScv,

            18 => Ability::HoldPosition,

            3678 => Ability::Land,
            554 => Ability::LandBarracks,
            419 => Ability::LandCommandCenter,
            520 => Ability::LandFactory,
            1524 => Ability::LandOrbitalCommand,
            522 => Ability::LandStarport,

            3679 => Ability::Lift,
            452 => Ability::LiftBarracks,
            417 => Ability::LiftCommandCenter,
            485 => Ability::LiftFactory,
            1522 => Ability::LiftOrbitalCommand,
            518 => Ability::LiftStarport,

            3668 => Ability::Load,
            3663 => Ability::LoadAll,
            416 => Ability::LoadAllCommandCenter,
            407 => Ability::LoadBunker,
            394 => Ability::LoadMedivac,

            1766 => Ability::MorphArchon,
            1372 => Ability::MorphBroodLord,
            1520 => Ability::MorphGateway,
            1220 => Ability::MorphGreaterSpire,
            1998 => Ability::MorphHellbat,
            1978 => Ability::MorphHellion,
            1218 => Ability::MorphHive,
            1216 => Ability::MorphLair,
            2560 => Ability::MorphLiberatorAaMode,
            2558 => Ability::MorphLiberatorAgMode,
            2332 => Ability::MorphLurker,
            2112 => Ability::MorphLurkerDen,
            1847 => Ability::MorphMothership,
            1516 => Ability::MorphOrbitalCommand,
            2708 => Ability::MorphOverlordTransport,
            1448 => Ability::MorphOverseer,
            1450 => Ability::MorphPlanetaryFortress,
            2330 => Ability::MorphRavager,
            3680 => Ability::MorphRoot,
            388 => Ability::MorphSiegeMode,
            1729 => Ability::MorphSpineCrawlerRoot,
            1725 => Ability::MorphSpineCrawlerUproot,
            1731 => Ability::MorphSporeCrawlerRoot,
            1727 => Ability::MorphSporeCrawlerUproot,
            556 => Ability::MorphSupplyDepotLower,
            558 => Ability::MorphSupplyDepotRaise,
            2364 => Ability::MorphThorExplosiveMode,
            2362 => Ability::MorphThorHighImpactMode,
            390 => Ability::MorphUnsiege,
            3681 => Ability::MorphUproot,
            403 => Ability::MorphVikingAssaultMode,
            405 => Ability::MorphVikingFighterMode,
            1518 => Ability::MorphWarpGate,
            1528 => Ability::MorphWarpPrismPhasingMode,
            1530 => Ability::MorphWarpPrismTransportMode,

            16 => Ability::Move,
            17 => Ability::Patrol,
            195 => Ability::RallyBuilding,
            203 => Ability::RallyCommandCenter,
            212 => Ability::RallyHatcheryUnits,
            211 => Ability::RallyHatcheryWorkers,
            199 => Ability::RallyMorphingUnit,
            207 => Ability::RallyNexus,
            3673 => Ability::RallyUnits,
            3690 => Ability::RallyWorkers,
            1594 => Ability::ResearchAdeptResonatingGlaives,
            805 => Ability::ResearchAdvancedBallistics,
            790 => Ability::ResearchBansheeCloakingField,
            799 => Ability::ResearchBansheeHyperFlightRotors,
            1532 => Ability::ResearchBattleCruiserWeaponRefit,
            1593 => Ability::ResearchBlink,
            1225 => Ability::ResearchBurrow,
            1482 => Ability::ResearchCentrifugalHooks,
            1592 => Ability::ResearchCharge,
            265 => Ability::ResearchChitinousPlating,
            731 => Ability::ResearchCombatShield,
            732 => Ability::ResearchConcussiveShells,
            764 => Ability::ResearchDrillingClaws,
            1097 => Ability::ResearchExtendedThermalLance,
            216 => Ability::ResearchGlialRegeneration,
            1093 => Ability::ResearchGraviticBooster,
            1094 => Ability::ResearchGraviticDrive,
            1282 => Ability::ResearchGroovedSpines,
            804 => Ability::ResearchHighCapacityFuelTanks,
            650 => Ability::ResearchHisecAutoTracking,
            761 => Ability::ResearchInfernalPreIgniter,
            44 => Ability::ResearchInterceptorGravitonCatapult,
            766 => Ability::ResearchMagFieldLaunchers,
            1283 => Ability::ResearchMuscularAugments,
            655 => Ability::ResearchNeoSteelFrame,
            1455 => Ability::ResearchNeuralParasite,
            1454 => Ability::ResearchPathogenGlands,
            820 => Ability::ResearchPersonalCloaking,
            46 => Ability::ResearchPhoenixAnionPulseCrystals,
            1223 => Ability::ResearchPneumatizedCarapace,
            3692 => Ability::ResearchProtossAirArmor,
            1565 => Ability::ResearchProtossAirArmorLevel1,
            1566 => Ability::ResearchProtossAirArmorLevel2,
            1567 => Ability::ResearchProtossAirArmorLevel3,
            3693 => Ability::ResearchProtossAirWeapons,
            1562 => Ability::ResearchProtossAirWeaponsLevel1,
            1563 => Ability::ResearchProtossAirWeaponsLevel2,
            1564 => Ability::ResearchProtossAirWeaponsLevel3,
            3694 => Ability::ResearchProtossGroundArmor,
            1065 => Ability::ResearchProtossGroundArmorLevel1,
            1066 => Ability::ResearchProtossGroundArmorLevel2,
            1067 => Ability::ResearchProtossGroundArmorLevel3,
            3695 => Ability::ResearchProtossGroundWeapons,
            1062 => Ability::ResearchProtossGroundWeaponsLevel1,
            1063 => Ability::ResearchProtossGroundWeaponsLevel2,
            1064 => Ability::ResearchProtossGroundWeaponsLevel3,
            3696 => Ability::ResearchProtossShields,
            1068 => Ability::ResearchProtossShieldsLevel1,
            1069 => Ability::ResearchProtossShieldsLevel2,
            1070 => Ability::ResearchProtossShieldsLevel3,
            1126 => Ability::ResearchPsiStorm,
            793 => Ability::ResearchRavenCorvidReactor,
            803 => Ability::ResearchRavenRecalibratedExplosives,
            2720 => Ability::ResearchShadowStrike,
            730 => Ability::ResearchStimpack,
            3697 => Ability::ResearchTerranInfantryArmor,
            656 => Ability::ResearchTerranInfantryArmorLevel1,
            657 => Ability::ResearchTerranInfantryArmorLevel2,
            658 => Ability::ResearchTerranInfantryArmorLevel3,
            3698 => Ability::ResearchTerranInfantryWeapons,
            652 => Ability::ResearchTerranInfantryWeaponsLevel1,
            653 => Ability::ResearchTerranInfantryWeaponsLevel2,
            654 => Ability::ResearchTerranInfantryWeaponsLevel3,
            3699 => Ability::ResearchTerranShipWeapons,
            861 => Ability::ResearchTerranShipWeaponsLevel1,
            862 => Ability::ResearchTerranShipWeaponsLevel2,
            863 => Ability::ResearchTerranShipWeaponsLevel3,
            651 => Ability::ResearchTerranStructureArmorUpgrade,
            3700 => Ability::ResearchTerranVehicleAndShipPlating,
            864 => Ability::ResearchTerranVehicleAndShipPlatingLevel1,
            865 => Ability::ResearchTerranVehicleAndShipPlatingLevel2,
            866 => Ability::ResearchTerranVehicleAndShipPlatingLevel3,
            3701 => Ability::ResearchTerranVehicleWeapons,
            855 => Ability::ResearchTerranVehicleWeaponsLevel1,
            856 => Ability::ResearchTerranVehicleWeaponsLevel2,
            857 => Ability::ResearchTerranVehicleWeaponsLevel3,
            217 => Ability::ResearchTunnelingClaws,
            1568 => Ability::ResearchWarpGate,
            3702 => Ability::ResearchZergFlyerArmor,
            1315 => Ability::ResearchZergFlyerArmorLevel1,
            1316 => Ability::ResearchZergFlyerArmorLevel2,
            1317 => Ability::ResearchZergFlyerArmorLevel3,
            3703 => Ability::ResearchZergFlyerAttack,
            1312 => Ability::ResearchZergFlyerAttackLevel1,
            1313 => Ability::ResearchZergFlyerAttackLevel2,
            1314 => Ability::ResearchZergFlyerAttackLevel3,
            3704 => Ability::ResearchZergGroundArmor,
            1189 => Ability::ResearchZergGroundArmorLevel1,
            1190 => Ability::ResearchZergGroundArmorLevel2,
            1191 => Ability::ResearchZergGroundArmorLevel3,
            1252 => Ability::ResearchZerglingAdrenalGlands,
            1253 => Ability::ResearchZerglingMetabolicBoost,
            3705 => Ability::ResearchZergMeleeWeapons,
            1186 => Ability::ResearchZergMeleeWeaponsLevel1,
            1187 => Ability::ResearchZergMeleeWeaponsLevel2,
            1188 => Ability::ResearchZergMeleeWeaponsLevel3,
            3706 => Ability::ResearchZergMissileWeapons,
            1192 => Ability::ResearchZergMissileWeaponsLevel1,
            1193 => Ability::ResearchZergMissileWeaponsLevel2,
            1194 => Ability::ResearchZergMissileWeaponsLevel3,

            19 => Ability::ScanMove,

            3665 => Ability::Stop,
            2057 => Ability::StopBuilding,
            6 => Ability::StopCheer,
            7 => Ability::StopDance,
            1691 => Ability::StopRedirect,
            4 => Ability::StopStop,

            1419 => Ability::TrainWarpAdept,
            1417 => Ability::TrainWarpDarkTemplar,
            1416 => Ability::TrainWarpHighTemplar,
            1418 => Ability::TrainWarpSentry,
            1414 => Ability::TrainWarpStalker,
            1413 => Ability::TrainWarpZealot,

            922 => Ability::TrainAdept,
            80 => Ability::TrainBaneling,
            621 => Ability::TrainBanshee,
            623 => Ability::TrainBattleCruiser,
            948 => Ability::TrainCarrier,
            978 => Ability::TrainColossus,
            1353 => Ability::TrainCorruptor,
            597 => Ability::TrainCyclone,
            920 => Ability::TrainDarkTemplar,
            994 => Ability::TrainDisruptor,
            1342 => Ability::TrainDrone,
            562 => Ability::TrainGhost,
            596 => Ability::TrainHellbat,
            595 => Ability::TrainHellion,
            919 => Ability::TrainHighTemplar,
            1345 => Ability::TrainHydralisk,
            979 => Ability::TrainImmortal,
            1352 => Ability::TrainInfestor,
            626 => Ability::TrainLiberator,
            563 => Ability::TrainMarauder,
            560 => Ability::TrainMarine,
            620 => Ability::TrainMedivac,
            1853 => Ability::TrainMothershipCore,
            1346 => Ability::TrainMutalisk,
            977 => Ability::TrainObserver,
            954 => Ability::TrainOracle,
            1344 => Ability::TrainOverlord,
            946 => Ability::TrainPhoenix,
            1006 => Ability::TrainProbe,
            1632 => Ability::TrainQueen,
            622 => Ability::TrainRaven,
            561 => Ability::TrainReaper,
            1351 => Ability::TrainRoach,
            524 => Ability::TrainScv,
            921 => Ability::TrainSentry,
            591 => Ability::TrainSiegeTank,
            917 => Ability::TrainStalker,
            1356 => Ability::TrainSwarmHost,
            955 => Ability::TrainTempest,
            594 => Ability::TrainThor,
            1348 => Ability::TrainUltralisk,
            624 => Ability::TrainVikingFighter,
            1354 => Ability::TrainViper,
            950 => Ability::TrainVoidRay,
            976 => Ability::TrainWarpPrism,
            614 => Ability::TrainWidowMine,
            916 => Ability::TrainZealot,
            1343 => Ability::TrainZergling,

            3664 => Ability::UnloadAll,
            3669 => Ability::UnloadAllAt,
            396 => Ability::UnloadAllAtMedivac,
            1408 => Ability::UnloadAllAtOverlord,
            913 => Ability::UnloadAllAtWarpPrism,
            408 => Ability::UnloadAllBunker,
            413 => Ability::UnloadAllCommandCenter,
            1438 => Ability::UnloadAllNydusNetwork,
            2371 => Ability::UnloadAllNydusWorm,
            410 => Ability::UnloadUnitBunker,
            415 => Ability::UnloadUnitCommandCenter,
            397 => Ability::UnloadUnitMedivac,
            1440 => Ability::UnloadUnitNydusNetwork,
            1409 => Ability::UnloadUnitOverlord,
            914 => Ability::UnloadUnitWarpPrism,

            _ => Ability::Invalid,
        }
    }

    pub fn as_id(&self) -> u32 {
        match *self {
            Ability::Invalid => 0,
            Ability::Smart => 1,

            Ability::Attack => 3674,
            Ability::AttackAttack => 23,
            Ability::AttackAttackBuilding => 2048,
            Ability::AttackRedirect => 1682,

            Ability::BehaviorBuildingAttackOff => 2082,
            Ability::BehaviorBuildingAttackOn => 2081,
            Ability::BehaviorCloakOff => 3677,
            Ability::BehaviorCloakOffBanshee => 393,
            Ability::BehaviorCloakOffGhost => 383,
            Ability::BehaviorCloakOn => 3676,
            Ability::BehaviorCloakOnBanshee => 392,
            Ability::BehaviorCloakOnGhost => 382,
            Ability::BehaviorGenerateCreepOff => 1693,
            Ability::BehaviorGenerateCreepOn => 1692,
            Ability::BehaviorHoldFireOff => 3689,
            Ability::BehaviorHoldFireOffLurker => 2552,
            Ability::BehaviorHoldFireOn => 3688,
            Ability::BehaviorHoldFireOnGhost => 36,
            Ability::BehaviorHoldFireOnLurker => 2550,
            Ability::BehaviorPulsarBeamOff => 2376,
            Ability::BehaviorPulsarBeamOn => 2375,

            Ability::BuildArmory => 331,
            Ability::BuildAssimilator => 882,
            Ability::BuildBanelingNest => 1162,
            Ability::BuildBarracks => 321,
            Ability::BuildBunker => 324,
            Ability::BuildCommandCenter => 318,
            Ability::BuildCreepTumor => 3691,
            Ability::BuildCreepTumorQueen => 1694,
            Ability::BuildCreepTumorTumor => 1733,
            Ability::BuildCyberneticsCore => 894,
            Ability::BuildDarkShrine => 891,
            Ability::BuildEngineeringBay => 322,
            Ability::BuildEvolutionChamber => 1156,
            Ability::BuildExtractor => 1154,
            Ability::BuildFactory => 328,
            Ability::BuildFleetBeacon => 885,
            Ability::BuildForge => 884,
            Ability::BuildFusionCore => 333,
            Ability::BuildGateway => 883,
            Ability::BuildGhostAcademy => 327,
            Ability::BuildHatchery => 1152,
            Ability::BuildHydraliskDen => 1157,
            Ability::BuildInfestationPit => 1160,
            Ability::BuildInterceptors => 1042,
            Ability::BuildMissileTurret => 323,
            Ability::BuildNexus => 880,
            Ability::BuildNuke => 710,
            Ability::BuildNydusNetwork => 1161,
            Ability::BuildNydusWorm => 1768,
            Ability::BuildPhotonCannon => 887,
            Ability::BuildPylon => 881,
            Ability::BuildReactor => 3683,
            Ability::BuildReactorBarracks => 422,
            Ability::BuildReactorFactory => 455,
            Ability::BuildReactorStarport => 488,
            Ability::BuildRefinery => 320,
            Ability::BuildRoachWarren => 1165,
            Ability::BuildRoboticsBay => 892,
            Ability::BuildRoboticsFacility => 893,
            Ability::BuildSensorTower => 326,
            Ability::BuildSpawningPool => 1155,
            Ability::BuildSpineCrawler => 1166,
            Ability::BuildSpire => 1158,
            Ability::BuildSporeCrawler => 1167,
            Ability::BuildStarGate => 889,
            Ability::BuildStarport => 329,
            Ability::BuildStasisTrap => 2505,
            Ability::BuildSupplyDepot => 319,
            Ability::BuildTechLab => 3682,
            Ability::BuildTechLabBarracks => 421,
            Ability::BuildTechLabFactory => 454,
            Ability::BuildTechLabStarport => 487,
            Ability::BuildTemplarArchive => 890,
            Ability::BuildTwilightCouncil => 886,
            Ability::BuildUltraliskCavern => 1159,

            Ability::BurrowDown => 3661,
            Ability::BurrowDownBaneling => 1374,
            Ability::BurrowDownDrone => 1378,
            Ability::BurrowDownHydralisk => 1382,
            Ability::BurrowDownInfestor => 1444,
            Ability::BurrowDownLurker => 2108,
            Ability::BurrowDownQueen => 1433,
            Ability::BurrowDownRavager => 2340,
            Ability::BurrowDownRoach => 1386,
            Ability::BurrowDownSwarmHost => 2014,
            Ability::BurrowDownWidowMine => 2095,
            Ability::BurrowDownZergling => 1390,

            Ability::BurrowUp => 3662,
            Ability::BurrowUpBaneling => 1376,
            Ability::BurrowUpDrone => 1380,
            Ability::BurrowUpHydralisk => 1384,
            Ability::BurrowUpInfestor => 1446,
            Ability::BurrowUpLurker => 2110,
            Ability::BurrowUpQueen => 1435,
            Ability::BurrowUpRavager => 2342,
            Ability::BurrowUpRoach => 1388,
            Ability::BurrowUpSwarmHost => 2016,
            Ability::BurrowUpWidowMine => 2097,
            Ability::BurrowUpZergling => 1392,

            Ability::Cancel => 3659,
            Ability::CancelSlotAddOn => 313,
            Ability::CancelSlotQueue1 => 305,
            Ability::CancelSlotQueue5 => 307,
            Ability::CancelSlotQueueCancelToSelection => 309,
            Ability::CancelSlotQueuePassive => 1832,
            Ability::CancelAdeptPhaseShift => 2594,
            Ability::CancelAdeptShadePhaseShift => 2596,
            Ability::CancelBarracksAddOn => 451,
            Ability::CancelBuildInProgress => 314,
            Ability::CancelCreepTumor => 1763,
            Ability::CancelFactoryAddOn => 484,
            Ability::CancelGravitonBeam => 174,
            Ability::CancelLast => 3671,
            Ability::CancelMorphBroodLord => 1373,
            Ability::CancelMorphLair => 1217,
            Ability::CancelMorphLurker => 2333,
            Ability::CancelMorphLurkerDen => 2113,
            Ability::CancelMorphMothership => 1848,
            Ability::CancelMorphOrbital => 1517,
            Ability::CancelMorphOverlordTransport => 2709,
            Ability::CancelMorphOverseer => 1449,
            Ability::CancelMorphPlanetaryFortress => 1451,
            Ability::CancelMorphRavager => 2331,
            Ability::CancelQueue1 => 304,
            Ability::CancelQueue5 => 306,
            Ability::CancelQueueAddOn => 312,
            Ability::CancelQueueCancelToSelection => 308,
            Ability::CancelQueuePassive => 1831,
            Ability::CancelQueuePassiveCancelTOSelection => 1833,
            Ability::CancelSpineCrawlerRoot => 1730,
            Ability::CancelStarportAddOn => 517,

            Ability::EffectAbduct => 2067,
            Ability::EffectAdeptPhaseShift => 2544,
            Ability::EffectAutoTurret => 1764,
            Ability::EffectBlindingCloud => 2063,
            Ability::EffectBlink => 3687,
            Ability::EffectBlinkStalker => 1442,
            Ability::EffectCallDownMule => 171,
            Ability::EffectCausticSpray => 2324,
            Ability::EffectCharge => 1819,
            Ability::EffectChronoBoost => 261,
            Ability::EffectContaminate => 1825,
            Ability::EffectCorrosiveBile => 2338,
            Ability::EffectEmp => 1628,
            Ability::EffectExplode => 42,
            Ability::EffectFeedback => 140,
            Ability::EffectForceField => 1526,
            Ability::EffectFungalGrowth => 74,
            Ability::EffectGhostSnipe => 2714,
            Ability::EffectGravitonBeam => 173,
            Ability::EffectGuardianShield => 76,
            Ability::EffectHeal => 386,
            Ability::EffectHunterSeekerMissile => 169,
            Ability::EffectImmortalBarrier => 2328,
            Ability::EffectInfestedTerrans => 247,
            Ability::EffectInjectLarva => 251,
            Ability::EffectKd8Charge => 2588,
            Ability::EffectLockOn => 2350,
            Ability::EffectLocustSwoop => 2387,
            Ability::EffectMassRecall => 3686,
            Ability::EffectMassRecallMothership => 2368,
            Ability::EffectMassRecallMothershipCore => 1974,
            Ability::EffectMedivacIgniteAfterBurners => 2116,
            Ability::EffectNeuralParasite => 249,
            Ability::EffectNukeCallDown => 1622,
            Ability::EffectOracleRevelation => 2146,
            Ability::EffectParasiticBomb => 2542,
            Ability::EffectPhotonOvercharge => 2162,
            Ability::EffectPointDefenseDrone => 144,
            Ability::EffectPsiStorm => 1036,
            Ability::EffectPurificationNova => 2346,
            Ability::EffectRepair => 3685,
            Ability::EffectRepairMule => 78,
            Ability::EffectRepairScv => 316,
            Ability::EffectSalvage => 32,
            Ability::EffectScan => 399,
            Ability::EffectShadowStride => 2700,
            Ability::EffectSpawnChangeling => 181,
            Ability::EffectSpawnLocusts => 2704,
            Ability::EffectSpray => 3684,
            Ability::EffectSprayProtoss => 30,
            Ability::EffectSprayTerran => 26,
            Ability::EffectSprayZerg => 28,
            Ability::EffectStim => 3675,
            Ability::EffectStimMarauder => 253,
            Ability::EffectStimMarine => 380,
            Ability::EffectStimMarineRedirect => 1683,
            Ability::EffectSupplyDrop => 255,
            Ability::EffectTacticalJump => 2358,
            Ability::EffectTempestDisruptionBlast => 2698,
            Ability::EffectTimeWarp => 2244,
            Ability::EffectTransfusion => 1664,
            Ability::EffectViperConsume => 2073,
            Ability::EffectVoidRayPrismaticAlignment => 2393,
            Ability::EffectWidowMineAttack => 2099,
            Ability::EffectYamatoGun => 401,

            Ability::HallucinationAdept => 2391,
            Ability::HallucinationArchon => 146,
            Ability::HallucinationColossus => 148,
            Ability::HallucinationDisruptor => 2389,
            Ability::HallucinationHighTemplar => 150,
            Ability::HallucinationImmortal => 152,
            Ability::HallucinationOracle => 2114,
            Ability::HallucinationPhoenix => 154,
            Ability::HallucinationProbe => 156,
            Ability::HallucinationStalker => 158,
            Ability::HallucinationVoidRay => 160,
            Ability::HallucinationWarpPrism => 162,
            Ability::HallucinationZealot => 164,

            Ability::Halt => 3660,
            Ability::HaltBuilding => 315,
            Ability::HaltTerranBuild => 348,

            Ability::HarvestGather => 3666,
            Ability::HarvestGatherDrone => 1183,
            Ability::HarvestGatherProbe => 298,
            Ability::HarvestGatherScv => 295,
            Ability::HarvestReturn => 3667,
            Ability::HarvestReturnDrone => 1184,
            Ability::HarvestReturnMule => 167,
            Ability::HarvestReturnProbe => 299,
            Ability::HarvestReturnScv => 296,

            Ability::HoldPosition => 18,

            Ability::Land => 3678,
            Ability::LandBarracks => 554,
            Ability::LandCommandCenter => 419,
            Ability::LandFactory => 520,
            Ability::LandOrbitalCommand => 1524,
            Ability::LandStarport => 522,

            Ability::Lift => 3679,
            Ability::LiftBarracks => 452,
            Ability::LiftCommandCenter => 417,
            Ability::LiftFactory => 485,
            Ability::LiftOrbitalCommand => 1522,
            Ability::LiftStarport => 518,

            Ability::Load => 3668,
            Ability::LoadAll => 3663,
            Ability::LoadAllCommandCenter => 416,
            Ability::LoadBunker => 407,
            Ability::LoadMedivac => 394,

            Ability::MorphArchon => 1766,
            Ability::MorphBroodLord => 1372,
            Ability::MorphGateway => 1520,
            Ability::MorphGreaterSpire => 1220,
            Ability::MorphHellbat => 1998,
            Ability::MorphHellion => 1978,
            Ability::MorphHive => 1218,
            Ability::MorphLair => 1216,
            Ability::MorphLiberatorAaMode => 2560,
            Ability::MorphLiberatorAgMode => 2558,
            Ability::MorphLurker => 2332,
            Ability::MorphLurkerDen => 2112,
            Ability::MorphMothership => 1847,
            Ability::MorphOrbitalCommand => 1516,
            Ability::MorphOverlordTransport => 2708,
            Ability::MorphOverseer => 1448,
            Ability::MorphPlanetaryFortress => 1450,
            Ability::MorphRavager => 2330,
            Ability::MorphRoot => 3680,
            Ability::MorphSiegeMode => 388,
            Ability::MorphSpineCrawlerRoot => 1729,
            Ability::MorphSpineCrawlerUproot => 1725,
            Ability::MorphSporeCrawlerRoot => 1731,
            Ability::MorphSporeCrawlerUproot => 1727,
            Ability::MorphSupplyDepotLower => 556,
            Ability::MorphSupplyDepotRaise => 558,
            Ability::MorphThorExplosiveMode => 2364,
            Ability::MorphThorHighImpactMode => 2362,
            Ability::MorphUnsiege => 390,
            Ability::MorphUproot => 3681,
            Ability::MorphVikingAssaultMode => 403,
            Ability::MorphVikingFighterMode => 405,
            Ability::MorphWarpGate => 1518,
            Ability::MorphWarpPrismPhasingMode => 1528,
            Ability::MorphWarpPrismTransportMode => 1530,

            Ability::Move => 16,
            Ability::Patrol => 17,
            Ability::RallyBuilding => 195,
            Ability::RallyCommandCenter => 203,
            Ability::RallyHatcheryUnits => 212,
            Ability::RallyHatcheryWorkers => 211,
            Ability::RallyMorphingUnit => 199,
            Ability::RallyNexus => 207,
            Ability::RallyUnits => 3673,
            Ability::RallyWorkers => 3690,
            Ability::ResearchAdeptResonatingGlaives => 1594,
            Ability::ResearchAdvancedBallistics => 805,
            Ability::ResearchBansheeCloakingField => 790,
            Ability::ResearchBansheeHyperFlightRotors => 799,
            Ability::ResearchBattleCruiserWeaponRefit => 1532,
            Ability::ResearchBlink => 1593,
            Ability::ResearchBurrow => 1225,
            Ability::ResearchCentrifugalHooks => 1482,
            Ability::ResearchCharge => 1592,
            Ability::ResearchChitinousPlating => 265,
            Ability::ResearchCombatShield => 731,
            Ability::ResearchConcussiveShells => 732,
            Ability::ResearchDrillingClaws => 764,
            Ability::ResearchExtendedThermalLance => 1097,
            Ability::ResearchGlialRegeneration => 216,
            Ability::ResearchGraviticBooster => 1093,
            Ability::ResearchGraviticDrive => 1094,
            Ability::ResearchGroovedSpines => 1282,
            Ability::ResearchHighCapacityFuelTanks => 804,
            Ability::ResearchHisecAutoTracking => 650,
            Ability::ResearchInfernalPreIgniter => 761,
            Ability::ResearchInterceptorGravitonCatapult => 44,
            Ability::ResearchMagFieldLaunchers => 766,
            Ability::ResearchMuscularAugments => 1283,
            Ability::ResearchNeoSteelFrame => 655,
            Ability::ResearchNeuralParasite => 1455,
            Ability::ResearchPathogenGlands => 1454,
            Ability::ResearchPersonalCloaking => 820,
            Ability::ResearchPhoenixAnionPulseCrystals => 46,
            Ability::ResearchPneumatizedCarapace => 1223,
            Ability::ResearchProtossAirArmor => 3692,
            Ability::ResearchProtossAirArmorLevel1 => 1565,
            Ability::ResearchProtossAirArmorLevel2 => 1566,
            Ability::ResearchProtossAirArmorLevel3 => 1567,
            Ability::ResearchProtossAirWeapons => 3693,
            Ability::ResearchProtossAirWeaponsLevel1 => 1562,
            Ability::ResearchProtossAirWeaponsLevel2 => 1563,
            Ability::ResearchProtossAirWeaponsLevel3 => 1564,
            Ability::ResearchProtossGroundArmor => 3694,
            Ability::ResearchProtossGroundArmorLevel1 => 1065,
            Ability::ResearchProtossGroundArmorLevel2 => 1066,
            Ability::ResearchProtossGroundArmorLevel3 => 1067,
            Ability::ResearchProtossGroundWeapons => 3695,
            Ability::ResearchProtossGroundWeaponsLevel1 => 1062,
            Ability::ResearchProtossGroundWeaponsLevel2 => 1063,
            Ability::ResearchProtossGroundWeaponsLevel3 => 1064,
            Ability::ResearchProtossShields => 3696,
            Ability::ResearchProtossShieldsLevel1 => 1068,
            Ability::ResearchProtossShieldsLevel2 => 1069,
            Ability::ResearchProtossShieldsLevel3 => 1070,
            Ability::ResearchPsiStorm => 1126,
            Ability::ResearchRavenCorvidReactor => 793,
            Ability::ResearchRavenRecalibratedExplosives => 803,
            Ability::ResearchShadowStrike => 2720,
            Ability::ResearchStimpack => 730,
            Ability::ResearchTerranInfantryArmor => 3697,
            Ability::ResearchTerranInfantryArmorLevel1 => 656,
            Ability::ResearchTerranInfantryArmorLevel2 => 657,
            Ability::ResearchTerranInfantryArmorLevel3 => 658,
            Ability::ResearchTerranInfantryWeapons => 3698,
            Ability::ResearchTerranInfantryWeaponsLevel1 => 652,
            Ability::ResearchTerranInfantryWeaponsLevel2 => 653,
            Ability::ResearchTerranInfantryWeaponsLevel3 => 654,
            Ability::ResearchTerranShipWeapons => 3699,
            Ability::ResearchTerranShipWeaponsLevel1 => 861,
            Ability::ResearchTerranShipWeaponsLevel2 => 862,
            Ability::ResearchTerranShipWeaponsLevel3 => 863,
            Ability::ResearchTerranStructureArmorUpgrade => 651,
            Ability::ResearchTerranVehicleAndShipPlating => 3700,
            Ability::ResearchTerranVehicleAndShipPlatingLevel1 => 864,
            Ability::ResearchTerranVehicleAndShipPlatingLevel2 => 865,
            Ability::ResearchTerranVehicleAndShipPlatingLevel3 => 866,
            Ability::ResearchTerranVehicleWeapons => 3701,
            Ability::ResearchTerranVehicleWeaponsLevel1 => 855,
            Ability::ResearchTerranVehicleWeaponsLevel2 => 856,
            Ability::ResearchTerranVehicleWeaponsLevel3 => 857,
            Ability::ResearchTunnelingClaws => 217,
            Ability::ResearchWarpGate => 1568,
            Ability::ResearchZergFlyerArmor => 3702,
            Ability::ResearchZergFlyerArmorLevel1 => 1315,
            Ability::ResearchZergFlyerArmorLevel2 => 1316,
            Ability::ResearchZergFlyerArmorLevel3 => 1317,
            Ability::ResearchZergFlyerAttack => 3703,
            Ability::ResearchZergFlyerAttackLevel1 => 1312,
            Ability::ResearchZergFlyerAttackLevel2 => 1313,
            Ability::ResearchZergFlyerAttackLevel3 => 1314,
            Ability::ResearchZergGroundArmor => 3704,
            Ability::ResearchZergGroundArmorLevel1 => 1189,
            Ability::ResearchZergGroundArmorLevel2 => 1190,
            Ability::ResearchZergGroundArmorLevel3 => 1191,
            Ability::ResearchZerglingAdrenalGlands => 1252,
            Ability::ResearchZerglingMetabolicBoost => 1253,
            Ability::ResearchZergMeleeWeapons => 3705,
            Ability::ResearchZergMeleeWeaponsLevel1 => 1186,
            Ability::ResearchZergMeleeWeaponsLevel2 => 1187,
            Ability::ResearchZergMeleeWeaponsLevel3 => 1188,
            Ability::ResearchZergMissileWeapons => 3706,
            Ability::ResearchZergMissileWeaponsLevel1 => 1192,
            Ability::ResearchZergMissileWeaponsLevel2 => 1193,
            Ability::ResearchZergMissileWeaponsLevel3 => 1194,

            Ability::ScanMove => 19,

            Ability::Stop => 3665,
            Ability::StopBuilding => 2057,
            Ability::StopCheer => 6,
            Ability::StopDance => 7,
            Ability::StopRedirect => 1691,
            Ability::StopStop => 4,

            Ability::TrainWarpAdept => 1419,
            Ability::TrainWarpDarkTemplar => 1417,
            Ability::TrainWarpHighTemplar => 1416,
            Ability::TrainWarpSentry => 1418,
            Ability::TrainWarpStalker => 1414,
            Ability::TrainWarpZealot => 1413,

            Ability::TrainAdept => 922,
            Ability::TrainBaneling => 80,
            Ability::TrainBanshee => 621,
            Ability::TrainBattleCruiser => 623,
            Ability::TrainCarrier => 948,
            Ability::TrainColossus => 978,
            Ability::TrainCorruptor => 1353,
            Ability::TrainCyclone => 597,
            Ability::TrainDarkTemplar => 920,
            Ability::TrainDisruptor => 994,
            Ability::TrainDrone => 1342,
            Ability::TrainGhost => 562,
            Ability::TrainHellbat => 596,
            Ability::TrainHellion => 595,
            Ability::TrainHighTemplar => 919,
            Ability::TrainHydralisk => 1345,
            Ability::TrainImmortal => 979,
            Ability::TrainInfestor => 1352,
            Ability::TrainLiberator => 626,
            Ability::TrainMarauder => 563,
            Ability::TrainMarine => 560,
            Ability::TrainMedivac => 620,
            Ability::TrainMothershipCore => 1853,
            Ability::TrainMutalisk => 1346,
            Ability::TrainObserver => 977,
            Ability::TrainOracle => 954,
            Ability::TrainOverlord => 1344,
            Ability::TrainPhoenix => 946,
            Ability::TrainProbe => 1006,
            Ability::TrainQueen => 1632,
            Ability::TrainRaven => 622,
            Ability::TrainReaper => 561,
            Ability::TrainRoach => 1351,
            Ability::TrainScv => 524,
            Ability::TrainSentry => 921,
            Ability::TrainSiegeTank => 591,
            Ability::TrainStalker => 917,
            Ability::TrainSwarmHost => 1356,
            Ability::TrainTempest => 955,
            Ability::TrainThor => 594,
            Ability::TrainUltralisk => 1348,
            Ability::TrainVikingFighter => 624,
            Ability::TrainViper => 1354,
            Ability::TrainVoidRay => 950,
            Ability::TrainWarpPrism => 976,
            Ability::TrainWidowMine => 614,
            Ability::TrainZealot => 916,
            Ability::TrainZergling => 1343,

            Ability::UnloadAll => 3664,
            Ability::UnloadAllAt => 3669,
            Ability::UnloadAllAtMedivac => 396,
            Ability::UnloadAllAtOverlord => 1408,
            Ability::UnloadAllAtWarpPrism => 913,
            Ability::UnloadAllBunker => 408,
            Ability::UnloadAllCommandCenter => 413,
            Ability::UnloadAllNydusNetwork => 1438,
            Ability::UnloadAllNydusWorm => 2371,
            Ability::UnloadUnitBunker => 410,
            Ability::UnloadUnitCommandCenter => 415,
            Ability::UnloadUnitMedivac => 397,
            Ability::UnloadUnitNydusNetwork => 1440,
            Ability::UnloadUnitOverlord => 1409,
            Ability::UnloadUnitWarpPrism => 914,
        }
    }
}
