
use super::super::{ Result, ErrorKind, FromProto, IntoProto };

/// list of known StarCraft II abilities
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Ability {
    Invalid,
    /// target: unit, point
    Smart,

    /// target: unit, point
    Attack,
    /// target: unit, point
    AttackAttack,
    /// target: unit, point
    AttackAttackBuilding,
    /// target: unit, point
    AttackRedirect,

    /// target: none
    BehaviorBuildingAttackOff,
    /// target: none
    BehaviorBuildingAttackOn,
    /// target: none
    BehaviorCloakOff,
    /// target: none
    BehaviorCloakOffBanshee,
    /// target: none
    BehaviorCloakOffGhost,
    /// target: none
    BehaviorCloakOn,
    /// target: none
    BehaviorCloakOnBanshee,
    /// target: none
    BehaviorCloakOnGhost,
    /// target: none
    BehaviorGenerateCreepOff,
    /// target: none
    BehaviorGenerateCreepOn,
    /// target: none
    BehaviorHoldFireOff,
    /// target: none
    BehaviorHoldFireOffLurker,
    /// target: none
    BehaviorHoldFireOn,
    /// target: none
    BehaviorHoldFireOnGhost,
    /// target: none
    BehaviorHoldFireOnLurker,
    /// target: none
    BehaviorPulsarBeamOff,
    /// target: none
    BehaviorPulsarBeamOn,

    /// target: point
    BuildArmory,
    /// target: unit
    BuildAssimilator,
    /// target: point
    BuildBanelingNest,
    /// target: point
    BuildBarracks,
    /// target: point
    BuildBunker,
    /// target: point
    BuildCommandCenter,
    /// target: point
    BuildCreepTumor,
    /// target: point
    BuildCreepTumorQueen,
    /// target: point
    BuildCreepTumorTumor,
    /// target: point
    BuildCyberneticsCore,
    /// target: point
    BuildDarkShrine,
    /// target: point
    BuildEngineeringBay,
    /// target: point
    BuildEvolutionChamber,
    /// target: unit
    BuildExtractor,
    /// target: point
    BuildFactory,
    /// target: point
    BuildFleetBeacon,
    /// target: point
    BuildForge,
    /// target: point
    BuildFusionCore,
    /// target: point
    BuildGateway,
    /// target: point
    BuildGhostAcademy,
    /// target: point
    BuildHatchery,
    /// target: point
    BuildHydraliskDen,
    /// target: point
    BuildInfestationPit,
    /// target: none
    BuildInterceptors,
    /// target: point
    BuildMissileTurret,
    /// target: point
    BuildNexus,
    /// target: none
    BuildNuke,
    /// target: point
    BuildNydusNetwork,
    /// target: point
    BuildNydusWorm,
    /// target: point
    BuildPhotonCannon,
    /// target: point
    BuildPylon,
    /// target: none
    BuildReactor,
    /// target: none
    BuildReactorBarracks,
    /// target: none
    BuildReactorFactory,
    /// target: none
    BuildReactorStarport,
    /// target: unit
    BuildRefinery,
    /// target: point
    BuildRoachWarren,
    /// target: point
    BuildRoboticsBay,
    /// target: point
    BuildRoboticsFacility,
    /// target: point
    BuildSensorTower,
    /// target: point
    BuildSpawningPool,
    /// target: point
    BuildSpineCrawler,
    /// target: point
    BuildSpire,
    /// target: point
    BuildSporeCrawler,
    /// target: point
    BuildStarGate,
    /// target: point
    BuildStarport,
    /// target: point
    BuildStasisTrap,
    /// target: point
    BuildSupplyDepot,
    /// target: none
    BuildTechLab,
    /// target: none
    BuildTechLabBarracks,
    /// target: none
    BuildTechLabFactory,
    /// target: none
    BuildTechLabStarport,
    /// target: point
    BuildTemplarArchive,
    /// target: point
    BuildTwilightCouncil,
    /// target: point
    BuildUltraliskCavern,

    /// target: none
    BurrowDown,
    /// target: none
    BurrowDownBaneling,
    /// target: none
    BurrowDownDrone,
    /// target: none
    BurrowDownHydralisk,
    /// target: none
    BurrowDownInfestor,
    /// target: none
    BurrowDownLurker,
    /// target: none
    BurrowDownQueen,
    /// target: none
    BurrowDownRavager,
    /// target: none
    BurrowDownRoach,
    /// target: none
    BurrowDownSwarmHost,
    /// target: none
    BurrowDownWidowMine,
    /// target: none
    BurrowDownZergling,

    /// target: none
    BurrowUp,
    /// target: none
    BurrowUpBaneling,
    /// target: none
    BurrowUpDrone,
    /// target: none
    BurrowUpHydralisk,
    /// target: none
    BurrowUpInfestor,
    /// target: none
    BurrowUpLurker,
    /// target: none
    BurrowUpQueen,
    /// target: none
    BurrowUpRavager,
    /// target: none
    BurrowUpRoach,
    /// target: none
    BurrowUpSwarmHost,
    /// target: none
    BurrowUpWidowMine,
    /// target: none
    BurrowUpZergling,

    /// target: none
    Cancel,
    /// target: none
    CancelSlotAddOn,
    /// target: none
    CancelSlotQueue1,
    /// target: none
    CancelSlotQueue5,
    /// target: none
    CancelSlotQueueCancelToSelection,
    /// target: none
    CancelSlotQueuePassive,
    /// target: none
    CancelAdeptPhaseShift,
    /// target: none
    CancelAdeptShadePhaseShift,
    /// target: none
    CancelBarracksAddOn,
    /// target: none
    CancelBuildInProgress,
    /// target: none
    CancelCreepTumor,
    /// target: none
    CancelFactoryAddOn,
    /// target: none
    CancelGravitonBeam,
    /// target: none
    CancelLast,
    /// target: none
    CancelMorphBroodLord,
    /// target: none
    CancelMorphLair,
    /// target: none
    CancelMorphLurker,
    /// target: none
    CancelMorphLurkerDen,
    /// target: none
    CancelMorphMothership,
    /// target: none
    CancelMorphOrbital,
    /// target: none
    CancelMorphOverlordTransport,
    /// target: none
    CancelMorphOverseer,
    /// target: none
    CancelMorphPlanetaryFortress,
    /// target: none
    CancelMorphRavager,
    /// target: none
    CancelQueue1,
    /// target: none
    CancelQueue5,
    /// target: none
    CancelQueueAddOn,
    /// target: none
    CancelQueueCancelToSelection,
    /// target: none
    CancelQueuePassive,
    /// target: none
    CancelQueuePassiveCancelTOSelection,
    /// target: none
    CancelSpineCrawlerRoot,
    /// target: none
    CancelStarportAddOn,

    /// target: unit
    EffectAbduct,
    /// target: point
    EffectAdeptPhaseShift,
    /// target: point
    EffectAutoTurret,
    /// target: point
    EffectBlindingCloud,
    /// target: point
    EffectBlink,
    /// target: point
    EffectBlinkStalker,
    /// target: unit, point
    EffectCallDownMule,
    /// target: unit
    EffectCausticSpray,
    /// target: unit
    EffectCharge,
    /// target: unit
    EffectChronoBoost,
    /// target: unit
    EffectContaminate,
    /// target: point
    EffectCorrosiveBile,
    /// target: point
    EffectEmp,
    /// target: none
    EffectExplode,
    /// target: unit
    EffectFeedback,
    /// target: point
    EffectForceField,
    /// target: point
    EffectFungalGrowth,
    /// target: unit
    EffectGhostSnipe,
    /// target: unit
    EffectGravitonBeam,
    /// target: none
    EffectGuardianShield,
    /// target: unit
    EffectHeal,
    /// target: unit
    EffectHunterSeekerMissile,
    /// target: none
    EffectImmortalBarrier,
    /// target: point
    EffectInfestedTerrans,
    /// target: unit
    EffectInjectLarva,
    /// target: unit, point
    EffectKd8Charge,
    /// target: unit
    EffectLockOn,
    /// target: point
    EffectLocustSwoop,
    /// target: unit
    EffectMassRecall,
    /// target: unit
    EffectMassRecallMothership,
    /// target: unit
    EffectMassRecallMothershipCore,
    /// target: none
    EffectMedivacIgniteAfterBurners,
    /// target: unit
    EffectNeuralParasite,
    /// target: point
    EffectNukeCallDown,
    /// target: point
    EffectOracleRevelation,
    /// target: unit
    EffectParasiticBomb,
    /// target: unit
    EffectPhotonOvercharge,
    /// target: point
    EffectPointDefenseDrone,
    /// target: point
    EffectPsiStorm,
    /// target: point
    EffectPurificationNova,
    /// target: unit
    EffectRepair,
    /// target: unit
    EffectRepairMule,
    /// target: unit
    EffectRepairScv,
    /// target: none
    EffectSalvage,
    /// target: point
    EffectScan,
    /// target: point
    EffectShadowStride,
    /// target: none
    EffectSpawnChangeling,
    /// target: point
    EffectSpawnLocusts,
    /// target: point
    EffectSpray,
    /// target: point
    EffectSprayProtoss,
    /// target: point
    EffectSprayTerran,
    /// target: point
    EffectSprayZerg,
    /// target: none
    EffectStim,
    /// target: none
    EffectStimMarauder,
    /// target: none
    EffectStimMarine,
    /// target: none
    EffectStimMarineRedirect,
    /// target: unit
    EffectSupplyDrop,
    /// target: point
    EffectTacticalJump,
    /// target: point
    EffectTempestDisruptionBlast,
    /// target: point
    EffectTimeWarp,
    /// target: unit
    EffectTransfusion,
    /// target: unit
    EffectViperConsume,
    /// target: none
    EffectVoidRayPrismaticAlignment,
    /// target: unit
    EffectWidowMineAttack,
    /// target: unit
    EffectYamatoGun,

    /// target: none
    HallucinationAdept,
    /// target: none
    HallucinationArchon,
    /// target: none
    HallucinationColossus,
    /// target: none
    HallucinationDisruptor,
    /// target: none
    HallucinationHighTemplar,
    /// target: none
    HallucinationImmortal,
    /// target: none
    HallucinationOracle,
    /// target: none
    HallucinationPhoenix,
    /// target: none
    HallucinationProbe,
    /// target: none
    HallucinationStalker,
    /// target: none
    HallucinationVoidRay,
    /// target: none
    HallucinationWarpPrism,
    /// target: none
    HallucinationZealot,

    /// target: none
    Halt,
    /// target: none
    HaltBuilding,
    /// target: none
    HaltTerranBuild,

    /// target: unit
    HarvestGather,
    /// target: unit
    HarvestGatherDrone,
    /// target: unit
    HarvestGatherProbe,
    /// target: unit
    HarvestGatherScv,
    /// target: none
    HarvestReturn,
    /// target: none
    HarvestReturnDrone,
    /// target: none
    HarvestReturnMule,
    /// target: none
    HarvestReturnProbe,
    /// target: none
    HarvestReturnScv,

    /// target: none
    HoldPosition,

    /// target: point
    Land,
    /// target: point
    LandBarracks,
    /// target: point
    LandCommandCenter,
    /// target: point
    LandFactory,
    /// target: point
    LandOrbitalCommand,
    /// target: point
    LandStarport,

    /// target: none
    Lift,
    /// target: none
    LiftBarracks,
    /// target: none
    LiftCommandCenter,
    /// target: none
    LiftFactory,
    /// target: none
    LiftOrbitalCommand,
    /// target: none
    LiftStarport,

    /// target: unit
    Load,
    /// target: none
    LoadAll,
    /// target: none
    LoadAllCommandCenter,
    /// target: unit
    LoadBunker,
    /// target: unit
    LoadMedivac,

    /// target: none
    MorphArchon,
    /// target: none
    MorphBroodLord,
    /// target: none
    MorphGateway,
    /// target: none
    MorphGreaterSpire,
    /// target: none
    MorphHellbat,
    /// target: none
    MorphHellion,
    /// target: none
    MorphHive,
    /// target: none
    MorphLair,
    /// target: none
    MorphLiberatorAaMode,
    /// target: point
    MorphLiberatorAgMode,
    /// target: none
    MorphLurker,
    /// target: none
    MorphLurkerDen,
    /// target: none
    MorphMothership,
    /// target: none
    MorphOrbitalCommand,
    /// target: none
    MorphOverlordTransport,
    /// target: none
    MorphOverseer,
    /// target: none
    MorphPlanetaryFortress,
    /// target: none
    MorphRavager,
    /// target: point
    MorphRoot,
    /// target: none
    MorphSiegeMode,
    /// target: point
    MorphSpineCrawlerRoot,
    /// target: none
    MorphSpineCrawlerUproot,
    /// target: point
    MorphSporeCrawlerRoot,
    /// target: none
    MorphSporeCrawlerUproot,
    /// target: none
    MorphSupplyDepotLower,
    /// target: none
    MorphSupplyDepotRaise,
    /// target: none
    MorphThorExplosiveMode,
    /// target: none
    MorphThorHighImpactMode,
    /// target: none
    MorphUnsiege,
    /// target: none
    MorphUproot,
    /// target: none
    MorphVikingAssaultMode,
    /// target: none
    MorphVikingFighterMode,
    /// target: none
    MorphWarpGate,
    /// target: none
    MorphWarpPrismPhasingMode,
    /// target: none
    MorphWarpPrismTransportMode,

    /// target: unit, point
    Move,
    /// target: unit, point
    Patrol,
    /// target: unit, point
    RallyBuilding,
    /// target: unit, point
    RallyCommandCenter,
    /// target: unit, point
    RallyHatcheryUnits,
    /// target: unit, point
    RallyHatcheryWorkers,
    /// target: unit, point
    RallyMorphingUnit,
    /// target: unit, point
    RallyNexus,
    /// target: unit, point
    RallyUnits,
    /// target: unit, point
    RallyWorkers,
    /// target: none
    ResearchAdeptResonatingGlaives,
    /// target: none
    ResearchAdvancedBallistics,
    /// target: none
    ResearchBansheeCloakingField,
    /// target: none
    ResearchBansheeHyperFlightRotors,
    /// target: none
    ResearchBattleCruiserWeaponRefit,
    /// target: none
    ResearchBlink,
    /// target: none
    ResearchBurrow,
    /// target: none
    ResearchCentrifugalHooks,
    /// target: none
    ResearchCharge,
    /// target: none
    ResearchChitinousPlating,
    /// target: none
    ResearchCombatShield,
    /// target: none
    ResearchConcussiveShells,
    /// target: none
    ResearchDrillingClaws,
    /// target: none
    ResearchExtendedThermalLance,
    /// target: none
    ResearchGlialRegeneration,
    /// target: none
    ResearchGraviticBooster,
    /// target: none
    ResearchGraviticDrive,
    /// target: none
    ResearchGroovedSpines,
    /// target: none
    ResearchHighCapacityFuelTanks,
    /// target: none
    ResearchHisecAutoTracking,
    /// target: none
    ResearchInfernalPreIgniter,
    /// target: none
    ResearchInterceptorGravitonCatapult,
    /// target: none
    ResearchMagFieldLaunchers,
    /// target: none
    ResearchMuscularAugments,
    /// target: none
    ResearchNeoSteelFrame,
    /// target: none
    ResearchNeuralParasite,
    /// target: none
    ResearchPathogenGlands,
    /// target: none
    ResearchPersonalCloaking,
    /// target: none
    ResearchPhoenixAnionPulseCrystals,
    /// target: none
    ResearchPneumatizedCarapace,
    /// target: none
    ResearchProtossAirArmor,
    /// target: none
    ResearchProtossAirArmorLevel1,
    /// target: none
    ResearchProtossAirArmorLevel2,
    /// target: none
    ResearchProtossAirArmorLevel3,
    /// target: none
    ResearchProtossAirWeapons,
    /// target: none
    ResearchProtossAirWeaponsLevel1,
    /// target: none
    ResearchProtossAirWeaponsLevel2,
    /// target: none
    ResearchProtossAirWeaponsLevel3,
    /// target: none
    ResearchProtossGroundArmor,
    /// target: none
    ResearchProtossGroundArmorLevel1,
    /// target: none
    ResearchProtossGroundArmorLevel2,
    /// target: none
    ResearchProtossGroundArmorLevel3,
    /// target: none
    ResearchProtossGroundWeapons,
    /// target: none
    ResearchProtossGroundWeaponsLevel1,
    /// target: none
    ResearchProtossGroundWeaponsLevel2,
    /// target: none
    ResearchProtossGroundWeaponsLevel3,
    /// target: none
    ResearchProtossShields,
    /// target: none
    ResearchProtossShieldsLevel1,
    /// target: none
    ResearchProtossShieldsLevel2,
    /// target: none
    ResearchProtossShieldsLevel3,
    /// target: none
    ResearchPsiStorm,
    /// target: none
    ResearchRavenCorvidReactor,
    /// target: none
    ResearchRavenRecalibratedExplosives,
    /// target: none
    ResearchShadowStrike,
    /// target: none
    ResearchStimpack,
    /// target: none
    ResearchTerranInfantryArmor,
    /// target: none
    ResearchTerranInfantryArmorLevel1,
    /// target: none
    ResearchTerranInfantryArmorLevel2,
    /// target: none
    ResearchTerranInfantryArmorLevel3,
    /// target: none
    ResearchTerranInfantryWeapons,
    /// target: none
    ResearchTerranInfantryWeaponsLevel1,
    /// target: none
    ResearchTerranInfantryWeaponsLevel2,
    /// target: none
    ResearchTerranInfantryWeaponsLevel3,
    /// target: none
    ResearchTerranShipWeapons,
    /// target: none
    ResearchTerranShipWeaponsLevel1,
    /// target: none
    ResearchTerranShipWeaponsLevel2,
    /// target: none
    ResearchTerranShipWeaponsLevel3,
    /// target: none
    ResearchTerranStructureArmorUpgrade,
    /// target: none
    ResearchTerranVehicleAndShipPlating,
    /// target: none
    ResearchTerranVehicleAndShipPlatingLevel1,
    /// target: none
    ResearchTerranVehicleAndShipPlatingLevel2,
    /// target: none
    ResearchTerranVehicleAndShipPlatingLevel3,
    /// target: none
    ResearchTerranVehicleWeapons,
    /// target: none
    ResearchTerranVehicleWeaponsLevel1,
    /// target: none
    ResearchTerranVehicleWeaponsLevel2,
    /// target: none
    ResearchTerranVehicleWeaponsLevel3,
    /// target: none
    ResearchTunnelingClaws,
    /// target: none
    ResearchWarpGate,
    /// target: none
    ResearchZergFlyerArmor,
    /// target: none
    ResearchZergFlyerArmorLevel1,
    /// target: none
    ResearchZergFlyerArmorLevel2,
    /// target: none
    ResearchZergFlyerArmorLevel3,
    /// target: none
    ResearchZergFlyerAttack,
    /// target: none
    ResearchZergFlyerAttackLevel1,
    /// target: none
    ResearchZergFlyerAttackLevel2,
    /// target: none
    ResearchZergFlyerAttackLevel3,
    /// target: none
    ResearchZergGroundArmor,
    /// target: none
    ResearchZergGroundArmorLevel1,
    /// target: none
    ResearchZergGroundArmorLevel2,
    /// target: none
    ResearchZergGroundArmorLevel3,
    /// target: none
    ResearchZerglingAdrenalGlands,
    /// target: none
    ResearchZerglingMetabolicBoost,
    /// target: none
    ResearchZergMeleeWeapons,
    /// target: none
    ResearchZergMeleeWeaponsLevel1,
    /// target: none
    ResearchZergMeleeWeaponsLevel2,
    /// target: none
    ResearchZergMeleeWeaponsLevel3,
    /// target: none
    ResearchZergMissileWeapons,
    /// target: none
    ResearchZergMissileWeaponsLevel1,
    /// target: none
    ResearchZergMissileWeaponsLevel2,
    /// target: none
    ResearchZergMissileWeaponsLevel3,

    /// target: unit, point
    ScanMove,

    /// target: none
    Stop,
    /// target: none
    StopBuilding,
    /// target: none
    StopAndCheer,
    /// target: none
    StopAndDance,
    /// target: none
    StopRedirect,
    /// target: none
    StopStop,

    /// target: point
    TrainWarpAdept,
    /// target: point
    TrainWarpDarkTemplar,
    /// target: point
    TrainWarpHighTemplar,
    /// target: point
    TrainWarpSentry,
    /// target: point
    TrainWarpStalker,
    /// target: point
    TrainWarpZealot,

    /// target: none
    TrainAdept,
    /// target: none
    TrainBaneling,
    /// target: none
    TrainBanshee,
    /// target: none
    TrainBattleCruiser,
    /// target: none
    TrainCarrier,
    /// target: none
    TrainColossus,
    /// target: none
    TrainCorruptor,
    /// target: none
    TrainCyclone,
    /// target: none
    TrainDarkTemplar,
    /// target: none
    TrainDisruptor,
    /// target: none
    TrainDrone,
    /// target: none
    TrainGhost,
    /// target: none
    TrainHellbat,
    /// target: none
    TrainHellion,
    /// target: none
    TrainHighTemplar,
    /// target: none
    TrainHydralisk,
    /// target: none
    TrainImmortal,
    /// target: none
    TrainInfestor,
    /// target: none
    TrainLiberator,
    /// target: none
    TrainMarauder,
    /// target: none
    TrainMarine,
    /// target: none
    TrainMedivac,
    /// target: none
    TrainMothershipCore,
    /// target: none
    TrainMutalisk,
    /// target: none
    TrainObserver,
    /// target: none
    TrainOracle,
    /// target: none
    TrainOverlord,
    /// target: none
    TrainPhoenix,
    /// target: none
    TrainProbe,
    /// target: none
    TrainQueen,
    /// target: none
    TrainRaven,
    /// target: none
    TrainReaper,
    /// target: none
    TrainRoach,
    /// target: none
    TrainScv,
    /// target: none
    TrainSentry,
    /// target: none
    TrainSiegeTank,
    /// target: none
    TrainStalker,
    /// target: none
    TrainSwarmHost,
    /// target: none
    TrainTempest,
    /// target: none
    TrainThor,
    /// target: none
    TrainUltralisk,
    /// target: none
    TrainVikingFighter,
    /// target: none
    TrainViper,
    /// target: none
    TrainVoidRay,
    /// target: none
    TrainWarpPrism,
    /// target: none
    TrainWidowMine,
    /// target: none
    TrainZealot,
    /// target: none
    TrainZergling,

    /// target: none
    UnloadAll,
    /// target: unit, point
    UnloadAllAt,
    /// target: unit, point
    UnloadAllAtMedivac,
    /// target: unit, point
    UnloadAllAtOverlord,
    /// target: unit, point
    UnloadAllAtWarpPrism,
    /// target: none
    UnloadAllBunker,
    /// target: none
    UnloadAllCommandCenter,
    /// target: none
    UnloadAllNydusNetwork,
    /// target: none
    UnloadAllNydusWorm,
    /// target: none
    UnloadUnitBunker,
    /// target: none
    UnloadUnitCommandCenter,
    /// target: none
    UnloadUnitMedivac,
    /// target: none
    UnloadUnitNydusNetwork,
    /// target: none
    UnloadUnitOverlord,
    /// target: none
    UnloadUnitWarpPrism,
}

impl FromProto<u32> for Ability {
    fn from_proto(id: u32) -> Result<Self> {
        Ok(
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
                6 => Ability::StopAndCheer,
                7 => Ability::StopAndDance,
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

                _ => bail!(
                    ErrorKind::InvalidProtobuf(format!("Ability id({})", id))
                )
            }
        )
    }
}

impl IntoProto<u32> for Ability {
    fn into_proto(self) -> Result<u32> {
        Ok(
            match self {
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
                Ability::StopAndCheer => 6,
                Ability::StopAndDance => 7,
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
        )
    }
}
