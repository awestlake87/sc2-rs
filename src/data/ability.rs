
use super::super::{ Result, FromProto, IntoProto };

/// list of known StarCraft II abilities
#[allow(missing_docs)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Ability {
    Invalid = 0,
    Smart = 1,

    Attack = 3674,
    AttackAttack = 23,
    AttackAttackBuilding = 2048,
    AttackRedirect = 1682,

    BehaviorBuildingAttackOff = 2082,
    BehaviorBuildingAttackOn = 2081,
    BehaviorCloakOff = 3677,
    BehaviorCloakOffBanshee = 393,
    BehaviorCloakOffGhost = 383,
    BehaviorCloakOn = 3676,
    BehaviorCloakOnBanshee = 392,
    BehaviorCloakOnGhost = 382,
    BehaviorGenerateCreepOff = 1693,
    BehaviorGenerateCreepOn = 1692,
    BehaviorHoldFireOff = 3689,
    BehaviorHoldFireOffLurker = 2552,
    BehaviorHoldFireOn = 3688,
    BehaviorHoldFireOnGhost = 36,
    BehaviorHoldFireOnLurker = 2550,
    BehaviorPulsarBeamOff = 2376,
    BehaviorPulsarBeamOn = 2375,

    BuildArmory = 331,
    BuildAssimilator = 882,
    BuildBanelingNest = 1162,
    BuildBarracks = 321,
    BuildBunker = 324,
    BuildCommandCenter = 318,
    BuildCreepTumor = 3691,
    BuildCreepTumorQueen = 1694,
    BuildCreepTumorTumor = 1733,
    BuildCyberneticsCore = 894,
    BuildDarkShrine = 891,
    BuildEngineeringBay = 322,
    BuildEvolutionChamber = 1156,
    BuildExtractor = 1154,
    BuildFactory = 328,
    BuildFleetBeacon = 885,
    BuildForge = 884,
    BuildFusionCore = 333,
    BuildGateway = 883,
    BuildGhostAcademy = 327,
    BuildHatchery = 1152,
    BuildHydraliskDen = 1157,
    BuildInfestationPit = 1160,
    BuildInterceptors = 1042,
    BuildMissileTurret = 323,
    BuildNexus = 880,
    BuildNuke = 710,
    BuildNydusNetwork = 1161,
    BuildNydusWorm = 1768,
    BuildPhotonCannon = 887,
    BuildPylon = 881,
    BuildReactor = 3683,
    BuildReactorBarracks = 422,
    BuildReactorFactory = 455,
    BuildReactorStarport = 488,
    BuildRefinery = 320,
    BuildRoachWarren = 1165,
    BuildRoboticsBay = 892,
    BuildRoboticsFacility = 893,
    BuildSensorTower = 326,
    BuildSpawningPool = 1155,
    BuildSpineCrawler = 1166,
    BuildSpire = 1158,
    BuildSporeCrawler = 1167,
    BuildStarGate = 889,
    BuildStarport = 329,
    BuildStasisTrap = 2505,
    BuildSupplyDepot = 319,
    BuildTechLab = 3682,
    BuildTechLabBarracks = 421,
    BuildTechLabFactory = 454,
    BuildTechLabStarport = 487,
    BuildTemplarArchive = 890,
    BuildTwilightCouncil = 886,
    BuildUltraliskCavern = 1159,

    BurrowDown = 3661,
    BurrowDownBaneling = 1374,
    BurrowDownDrone = 1378,
    BurrowDownHydralisk = 1382,
    BurrowDownInfestor = 1444,
    BurrowDownLurker = 2108,
    BurrowDownQueen = 1433,
    BurrowDownRavager = 2340,
    BurrowDownRoach = 1386,
    BurrowDownSwarmHost = 2014,
    BurrowDownWidowMine = 2095,
    BurrowDownZergling = 1390,

    BurrowUp = 3662,
    BurrowUpBaneling = 1376,
    BurrowUpDrone = 1380,
    BurrowUpHydralisk = 1384,
    BurrowUpInfestor = 1446,
    BurrowUpLurker = 2110,
    BurrowUpQueen = 1435,
    BurrowUpRavager = 2342,
    BurrowUpRoach = 1388,
    BurrowUpSwarmHost = 2016,
    BurrowUpWidowMine = 2097,
    BurrowUpZergling = 1392,

    Cancel = 3659,
    CancelSlotAddOn = 313,
    CancelSlotQueue1 = 305,
    CancelSlotQueue5 = 307,
    CancelSlotQueueCancelToSelection = 309,
    CancelSlotQueuePassive = 1832,
    CancelAdeptPhaseShift = 2594,
    CancelAdeptShadePhaseShift = 2596,
    CancelBarracksAddOn = 451,
    CancelBuildInProgress = 314,
    CancelCreepTumor = 1763,
    CancelFactoryAddOn = 484,
    CancelGravitonBeam = 174,
    CancelLast = 3671,
    CancelMorphBroodLord = 1373,
    CancelMorphLair = 1217,
    CancelMorphLurker = 2333,
    CancelMorphLurkerDen = 2113,
    CancelMorphMothership = 1848,
    CancelMorphOrbital = 1517,
    CancelMorphOverlordTransport = 2709,
    CancelMorphOverseer = 1449,
    CancelMorphPlanetaryFortress = 1451,
    CancelMorphRavager = 2331,
    CancelQueue1 = 304,
    CancelQueue5 = 306,
    CancelQueueAddOn = 312,
    CancelQueueCancelToSelection = 308,
    CancelQueuePassive = 1831,
    CancelQueuePassiveCancelTOSelection = 1833,
    CancelSpineCrawlerRoot = 1730,
    CancelStarportAddOn = 517,

    EffectAbduct = 2067,
    EffectAdeptPhaseShift = 2544,
    EffectAutoTurret = 1764,
    EffectBlindingCloud = 2063,
    EffectBlink = 3687,
    EffectBlinkStalker = 1442,
    EffectCallDownMule = 171,
    EffectCausticSpray = 2324,
    EffectCharge = 1819,
    EffectChronoBoost = 261,
    EffectContaminate = 1825,
    EffectCorrosiveBile = 2338,
    EffectEmp = 1628,
    EffectExplode = 42,
    EffectFeedback = 140,
    EffectForceField = 1526,
    EffectFungalGrowth = 74,
    EffectGhostSnipe = 2714,
    EffectGravitonBeam = 173,
    EffectGuardianShield = 76,
    EffectHeal = 386,
    EffectHunterSeekerMissile = 169,
    EffectImmortalBarrier = 2328,
    EffectInfestedTerrans = 247,
    EffectInjectLarva = 251,
    EffectKd8Charge = 2588,
    EffectLockOn = 2350,
    EffectLocustSwoop = 2387,
    EffectMassRecall = 3686,
    EffectMassRecallMothership = 2368,
    EffectMassRecallMothershipCore = 1974,
    EffectMedivacIgniteAfterBurners = 2116,
    EffectNeuralParasite = 249,
    EffectNukeCallDown = 1622,
    EffectOracleRevelation = 2146,
    EffectParasiticBomb = 2542,
    EffectPhotonOvercharge = 2162,
    EffectPointDefenseDrone = 144,
    EffectPsiStorm = 1036,
    EffectPurificationNova = 2346,
    EffectRepair = 3685,
    EffectRepairMule = 78,
    EffectRepairScv = 316,
    EffectSalvage = 32,
    EffectScan = 399,
    EffectShadowStride = 2700,
    EffectSpawnChangeling = 181,
    EffectSpawnLocusts = 2704,
    EffectSpray = 3684,
    EffectSprayProtoss = 30,
    EffectSprayTerran = 26,
    EffectSprayZerg = 28,
    EffectStim = 3675,
    EffectStimMarauder = 253,
    EffectStimMarine = 380,
    EffectStimMarineRedirect = 1683,
    EffectSupplyDrop = 255,
    EffectTacticalJump = 2358,
    EffectTempestDisruptionBlast = 2698,
    EffectTimeWarp = 2244,
    EffectTransfusion = 1664,
    EffectViperConsume = 2073,
    EffectVoidRayPrismaticAlignment = 2393,
    EffectWidowMineAttack = 2099,
    EffectYamatoGun = 401,

    HallucinationAdept = 2391,
    HallucinationArchon = 146,
    HallucinationColossus = 148,
    HallucinationDisruptor = 2389,
    HallucinationHighTemplar = 150,
    HallucinationImmortal = 152,
    HallucinationOracle = 2114,
    HallucinationPhoenix = 154,
    HallucinationProbe = 156,
    HallucinationStalker = 158,
    HallucinationVoidRay = 160,
    HallucinationWarpPrism = 162,
    HallucinationZealot = 164,

    Halt = 3660,
    HaltBuilding = 315,
    HaltTerranBuild = 348,

    HarvestGather = 3666,
    HarvestGatherDrone = 1183,
    HarvestGatherProbe = 298,
    HarvestGatherScv = 295,
    HarvestReturn = 3667,
    HarvestReturnDrone = 1184,
    HarvestReturnMule = 167,
    HarvestReturnProbe = 299,
    HarvestReturnScv = 296,

    HoldPosition = 18,

    Land = 3678,
    LandBarracks = 554,
    LandCommandCenter = 419,
    LandFactory = 520,
    LandOrbitalCommand = 1524,
    LandStarport = 522,

    Lift = 3679,
    LiftBarracks = 452,
    LiftCommandCenter = 417,
    LiftFactory = 485,
    LiftOrbitalCommand = 1522,
    LiftStarport = 518,

    Load = 3668,
    LoadAll = 3663,
    LoadAllCommandCenter = 416,
    LoadBunker = 407,
    LoadMedivac = 394,

    MorphArchon = 1766,
    MorphBroodLord = 1372,
    MorphGateway = 1520,
    MorphGreaterSpire = 1220,
    MorphHellbat = 1998,
    MorphHellion = 1978,
    MorphHive = 1218,
    MorphLair = 1216,
    MorphLiberatorAaMode = 2560,
    MorphLiberatorAgMode = 2558,
    MorphLurker = 2332,
    MorphLurkerDen = 2112,
    MorphMothership = 1847,
    MorphOrbitalCommand = 1516,
    MorphOverlordTransport = 2708,
    MorphOverseer = 1448,
    MorphPlanetaryFortress = 1450,
    MorphRavager = 2330,
    MorphRoot = 3680,
    MorphSiegeMode = 388,
    MorphSpineCrawlerRoot = 1729,
    MorphSpineCrawlerUproot = 1725,
    MorphSporeCrawlerRoot = 1731,
    MorphSporeCrawlerUproot = 1727,
    MorphSupplyDepotLower = 556,
    MorphSupplyDepotRaise = 558,
    MorphThorExplosiveMode = 2364,
    MorphThorHighImpactMode = 2362,
    MorphUnsiege = 390,
    MorphUproot = 3681,
    MorphVikingAssaultMode = 403,
    MorphVikingFighterMode = 405,
    MorphWarpGate = 1518,
    MorphWarpPrismPhasingMode = 1528,
    MorphWarpPrismTransportMode = 1530,

    Move = 16,
    Patrol = 17,
    RallyBuilding = 195,
    RallyCommandCenter = 203,
    RallyHatcheryUnits = 212,
    RallyHatcheryWorkers = 211,
    RallyMorphingUnit = 199,
    RallyNexus = 207,
    RallyUnits = 3673,
    RallyWorkers = 3690,
    ResearchAdeptResonatingGlaives = 1594,
    ResearchAdvancedBallistics = 805,
    ResearchBansheeCloakingField = 790,
    ResearchBansheeHyperFlightRotors = 799,
    ResearchBattleCruiserWeaponRefit = 1532,
    ResearchBlink = 1593,
    ResearchBurrow = 1225,
    ResearchCentrifugalHooks = 1482,
    ResearchCharge = 1592,
    ResearchChitinousPlating = 265,
    ResearchCombatShield = 731,
    ResearchConcussiveShells = 732,
    ResearchDrillingClaws = 764,
    ResearchExtendedThermalLance = 1097,
    ResearchGlialRegeneration = 216,
    ResearchGraviticBooster = 1093,
    ResearchGraviticDrive = 1094,
    ResearchGroovedSpines = 1282,
    ResearchHighCapacityFuelTanks = 804,
    ResearchHisecAutoTracking = 650,
    ResearchInfernalPreIgniter = 761,
    ResearchInterceptorGravitonCatapult = 44,
    ResearchMagFieldLaunchers = 766,
    ResearchMuscularAugments = 1283,
    ResearchNeoSteelFrame = 655,
    ResearchNeuralParasite = 1455,
    ResearchPathogenGlands = 1454,
    ResearchPersonalCloaking = 820,
    ResearchPhoenixAnionPulseCrystals = 46,
    ResearchPneumatizedCarapace = 1223,
    ResearchProtossAirArmor = 3692,
    ResearchProtossAirArmorLevel1 = 1565,
    ResearchProtossAirArmorLevel2 = 1566,
    ResearchProtossAirArmorLevel3 = 1567,
    ResearchProtossAirWeapons = 3693,
    ResearchProtossAirWeaponsLevel1 = 1562,
    ResearchProtossAirWeaponsLevel2 = 1563,
    ResearchProtossAirWeaponsLevel3 = 1564,
    ResearchProtossGroundArmor = 3694,
    ResearchProtossGroundArmorLevel1 = 1065,
    ResearchProtossGroundArmorLevel2 = 1066,
    ResearchProtossGroundArmorLevel3 = 1067,
    ResearchProtossGroundWeapons = 3695,
    ResearchProtossGroundWeaponsLevel1 = 1062,
    ResearchProtossGroundWeaponsLevel2 = 1063,
    ResearchProtossGroundWeaponsLevel3 = 1064,
    ResearchProtossShields = 3696,
    ResearchProtossShieldsLevel1 = 1068,
    ResearchProtossShieldsLevel2 = 1069,
    ResearchProtossShieldsLevel3 = 1070,
    ResearchPsiStorm = 1126,
    ResearchRavenCorvidReactor = 793,
    ResearchRavenRecalibratedExplosives = 803,
    ResearchShadowStrike = 2720,
    ResearchStimpack = 730,
    ResearchTerranInfantryArmor = 3697,
    ResearchTerranInfantryArmorLevel1 = 656,
    ResearchTerranInfantryArmorLevel2 = 657,
    ResearchTerranInfantryArmorLevel3 = 658,
    ResearchTerranInfantryWeapons = 3698,
    ResearchTerranInfantryWeaponsLevel1 = 652,
    ResearchTerranInfantryWeaponsLevel2 = 653,
    ResearchTerranInfantryWeaponsLevel3 = 654,
    ResearchTerranShipWeapons = 3699,
    ResearchTerranShipWeaponsLevel1 = 861,
    ResearchTerranShipWeaponsLevel2 = 862,
    ResearchTerranShipWeaponsLevel3 = 863,
    ResearchTerranStructureArmorUpgrade = 651,
    ResearchTerranVehicleAndShipPlating = 3700,
    ResearchTerranVehicleAndShipPlatingLevel1 = 864,
    ResearchTerranVehicleAndShipPlatingLevel2 = 865,
    ResearchTerranVehicleAndShipPlatingLevel3 = 866,
    ResearchTerranVehicleWeapons = 3701,
    ResearchTerranVehicleWeaponsLevel1 = 855,
    ResearchTerranVehicleWeaponsLevel2 = 856,
    ResearchTerranVehicleWeaponsLevel3 = 857,
    ResearchTunnelingClaws = 217,
    ResearchWarpGate = 1568,
    ResearchZergFlyerArmor = 3702,
    ResearchZergFlyerArmorLevel1 = 1315,
    ResearchZergFlyerArmorLevel2 = 1316,
    ResearchZergFlyerArmorLevel3 = 1317,
    ResearchZergFlyerAttack = 3703,
    ResearchZergFlyerAttackLevel1 = 1312,
    ResearchZergFlyerAttackLevel2 = 1313,
    ResearchZergFlyerAttackLevel3 = 1314,
    ResearchZergGroundArmor = 3704,
    ResearchZergGroundArmorLevel1 = 1189,
    ResearchZergGroundArmorLevel2 = 1190,
    ResearchZergGroundArmorLevel3 = 1191,
    ResearchZerglingAdrenalGlands = 1252,
    ResearchZerglingMetabolicBoost = 1253,
    ResearchZergMeleeWeapons = 3705,
    ResearchZergMeleeWeaponsLevel1 = 1186,
    ResearchZergMeleeWeaponsLevel2 = 1187,
    ResearchZergMeleeWeaponsLevel3 = 1188,
    ResearchZergMissileWeapons = 3706,
    ResearchZergMissileWeaponsLevel1 = 1192,
    ResearchZergMissileWeaponsLevel2 = 1193,
    ResearchZergMissileWeaponsLevel3 = 1194,

    ScanMove = 19,

    Stop = 3665,
    StopBuilding = 2057,
    StopAndCheer = 6,
    StopAndDance = 7,
    StopRedirect = 1691,
    StopStop = 4,

    TrainWarpAdept = 1419,
    TrainWarpDarkTemplar = 1417,
    TrainWarpHighTemplar = 1416,
    TrainWarpSentry = 1418,
    TrainWarpStalker = 1414,
    TrainWarpZealot = 1413,

    TrainAdept = 922,
    TrainBaneling = 80,
    TrainBanshee = 621,
    TrainBattleCruiser = 623,
    TrainCarrier = 948,
    TrainColossus = 978,
    TrainCorruptor = 1353,
    TrainCyclone = 597,
    TrainDarkTemplar = 920,
    TrainDisruptor = 994,
    TrainDrone = 1342,
    TrainGhost = 562,
    TrainHellbat = 596,
    TrainHellion = 595,
    TrainHighTemplar = 919,
    TrainHydralisk = 1345,
    TrainImmortal = 979,
    TrainInfestor = 1352,
    TrainLiberator = 626,
    TrainMarauder = 563,
    TrainMarine = 560,
    TrainMedivac = 620,
    TrainMothershipCore = 1853,
    TrainMutalisk = 1346,
    TrainObserver = 977,
    TrainOracle = 954,
    TrainOverlord = 1344,
    TrainPhoenix = 946,
    TrainProbe = 1006,
    TrainQueen = 1632,
    TrainRaven = 622,
    TrainReaper = 561,
    TrainRoach = 1351,
    TrainScv = 524,
    TrainSentry = 921,
    TrainSiegeTank = 591,
    TrainStalker = 917,
    TrainSwarmHost = 1356,
    TrainTempest = 955,
    TrainThor = 594,
    TrainUltralisk = 1348,
    TrainVikingFighter = 624,
    TrainViper = 1354,
    TrainVoidRay = 950,
    TrainWarpPrism = 976,
    TrainWidowMine = 614,
    TrainZealot = 916,
    TrainZergling = 1343,

    UnloadAll = 3664,
    UnloadAllAt = 3669,
    UnloadAllAtMedivac = 396,
    UnloadAllAtOverlord = 1408,
    UnloadAllAtWarpPrism = 913,
    UnloadAllBunker = 408,
    UnloadAllCommandCenter = 413,
    UnloadAllNydusNetwork = 1438,
    UnloadAllNydusWorm = 2371,
    UnloadUnitBunker = 410,
    UnloadUnitCommandCenter = 415,
    UnloadUnitMedivac = 397,
    UnloadUnitNydusNetwork = 1440,
    UnloadUnitOverlord = 1409,
    UnloadUnitWarpPrism = 914,
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

                _ => Ability::Invalid
            }
        )
    }
}

impl IntoProto<u32> for Ability {
    fn into_proto(self) -> Result<u32> {
        Ok(self as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutativity() {
        let test_element = |element: Ability| assert_eq!(
            element,
            Ability::from_proto(element.into_proto().unwrap()).unwrap()
        );

        test_element(Ability::Smart);

        test_element(Ability::Attack);
        test_element(Ability::AttackAttack);
        test_element(Ability::AttackAttackBuilding);
        test_element(Ability::AttackRedirect);

        test_element(Ability::BehaviorBuildingAttackOff);
        test_element(Ability::BehaviorBuildingAttackOn);
        test_element(Ability::BehaviorCloakOff);
        test_element(Ability::BehaviorCloakOffBanshee);
        test_element(Ability::BehaviorCloakOffGhost);
        test_element(Ability::BehaviorCloakOn);
        test_element(Ability::BehaviorCloakOnBanshee);
        test_element(Ability::BehaviorCloakOnGhost);
        test_element(Ability::BehaviorGenerateCreepOff);
        test_element(Ability::BehaviorGenerateCreepOn);
        test_element(Ability::BehaviorHoldFireOff);
        test_element(Ability::BehaviorHoldFireOffLurker);
        test_element(Ability::BehaviorHoldFireOn);
        test_element(Ability::BehaviorHoldFireOnGhost);
        test_element(Ability::BehaviorHoldFireOnLurker);
        test_element(Ability::BehaviorPulsarBeamOff);
        test_element(Ability::BehaviorPulsarBeamOn);

        test_element(Ability::BuildArmory);
        test_element(Ability::BuildAssimilator);
        test_element(Ability::BuildBanelingNest);
        test_element(Ability::BuildBarracks);
        test_element(Ability::BuildBunker);
        test_element(Ability::BuildCommandCenter);
        test_element(Ability::BuildCreepTumor);
        test_element(Ability::BuildCreepTumorQueen);
        test_element(Ability::BuildCreepTumorTumor);
        test_element(Ability::BuildCyberneticsCore);
        test_element(Ability::BuildDarkShrine);
        test_element(Ability::BuildEngineeringBay);
        test_element(Ability::BuildEvolutionChamber);
        test_element(Ability::BuildExtractor);
        test_element(Ability::BuildFactory);
        test_element(Ability::BuildFleetBeacon);
        test_element(Ability::BuildForge);
        test_element(Ability::BuildFusionCore);
        test_element(Ability::BuildGateway);
        test_element(Ability::BuildGhostAcademy);
        test_element(Ability::BuildHatchery);
        test_element(Ability::BuildHydraliskDen);
        test_element(Ability::BuildInfestationPit);
        test_element(Ability::BuildInterceptors);
        test_element(Ability::BuildMissileTurret);
        test_element(Ability::BuildNexus);
        test_element(Ability::BuildNuke);
        test_element(Ability::BuildNydusNetwork);
        test_element(Ability::BuildNydusWorm);
        test_element(Ability::BuildPhotonCannon);
        test_element(Ability::BuildPylon);
        test_element(Ability::BuildReactor);
        test_element(Ability::BuildReactorBarracks);
        test_element(Ability::BuildReactorFactory);
        test_element(Ability::BuildReactorStarport);
        test_element(Ability::BuildRefinery);
        test_element(Ability::BuildRoachWarren);
        test_element(Ability::BuildRoboticsBay);
        test_element(Ability::BuildRoboticsFacility);
        test_element(Ability::BuildSensorTower);
        test_element(Ability::BuildSpawningPool);
        test_element(Ability::BuildSpineCrawler);
        test_element(Ability::BuildSpire);
        test_element(Ability::BuildSporeCrawler);
        test_element(Ability::BuildStarGate);
        test_element(Ability::BuildStarport);
        test_element(Ability::BuildStasisTrap);
        test_element(Ability::BuildSupplyDepot);
        test_element(Ability::BuildTechLab);
        test_element(Ability::BuildTechLabBarracks);
        test_element(Ability::BuildTechLabFactory);
        test_element(Ability::BuildTechLabStarport);
        test_element(Ability::BuildTemplarArchive);
        test_element(Ability::BuildTwilightCouncil);
        test_element(Ability::BuildUltraliskCavern);

        test_element(Ability::BurrowDown);
        test_element(Ability::BurrowDownBaneling);
        test_element(Ability::BurrowDownDrone);
        test_element(Ability::BurrowDownHydralisk);
        test_element(Ability::BurrowDownInfestor);
        test_element(Ability::BurrowDownLurker);
        test_element(Ability::BurrowDownQueen);
        test_element(Ability::BurrowDownRavager);
        test_element(Ability::BurrowDownRoach);
        test_element(Ability::BurrowDownSwarmHost);
        test_element(Ability::BurrowDownWidowMine);
        test_element(Ability::BurrowDownZergling);

        test_element(Ability::BurrowUp);
        test_element(Ability::BurrowUpBaneling);
        test_element(Ability::BurrowUpDrone);
        test_element(Ability::BurrowUpHydralisk);
        test_element(Ability::BurrowUpInfestor);
        test_element(Ability::BurrowUpLurker);
        test_element(Ability::BurrowUpQueen);
        test_element(Ability::BurrowUpRavager);
        test_element(Ability::BurrowUpRoach);
        test_element(Ability::BurrowUpSwarmHost);
        test_element(Ability::BurrowUpWidowMine);
        test_element(Ability::BurrowUpZergling);

        test_element(Ability::Cancel);
        test_element(Ability::CancelSlotAddOn);
        test_element(Ability::CancelSlotQueue1);
        test_element(Ability::CancelSlotQueue5);
        test_element(Ability::CancelSlotQueueCancelToSelection);
        test_element(Ability::CancelSlotQueuePassive);
        test_element(Ability::CancelAdeptPhaseShift);
        test_element(Ability::CancelAdeptShadePhaseShift);
        test_element(Ability::CancelBarracksAddOn);
        test_element(Ability::CancelBuildInProgress);
        test_element(Ability::CancelCreepTumor);
        test_element(Ability::CancelFactoryAddOn);
        test_element(Ability::CancelGravitonBeam);
        test_element(Ability::CancelLast);
        test_element(Ability::CancelMorphBroodLord);
        test_element(Ability::CancelMorphLair);
        test_element(Ability::CancelMorphLurker);
        test_element(Ability::CancelMorphLurkerDen);
        test_element(Ability::CancelMorphMothership);
        test_element(Ability::CancelMorphOrbital);
        test_element(Ability::CancelMorphOverlordTransport);
        test_element(Ability::CancelMorphOverseer);
        test_element(Ability::CancelMorphPlanetaryFortress);
        test_element(Ability::CancelMorphRavager);
        test_element(Ability::CancelQueue1);
        test_element(Ability::CancelQueue5);
        test_element(Ability::CancelQueueAddOn);
        test_element(Ability::CancelQueueCancelToSelection);
        test_element(Ability::CancelQueuePassive);
        test_element(Ability::CancelQueuePassiveCancelTOSelection);
        test_element(Ability::CancelSpineCrawlerRoot);
        test_element(Ability::CancelStarportAddOn);

        test_element(Ability::EffectAbduct);
        test_element(Ability::EffectAdeptPhaseShift);
        test_element(Ability::EffectAutoTurret);
        test_element(Ability::EffectBlindingCloud);
        test_element(Ability::EffectBlink);
        test_element(Ability::EffectBlinkStalker);
        test_element(Ability::EffectCallDownMule);
        test_element(Ability::EffectCausticSpray);
        test_element(Ability::EffectCharge);
        test_element(Ability::EffectChronoBoost);
        test_element(Ability::EffectContaminate);
        test_element(Ability::EffectCorrosiveBile);
        test_element(Ability::EffectEmp);
        test_element(Ability::EffectExplode);
        test_element(Ability::EffectFeedback);
        test_element(Ability::EffectForceField);
        test_element(Ability::EffectFungalGrowth);
        test_element(Ability::EffectGhostSnipe);
        test_element(Ability::EffectGravitonBeam);
        test_element(Ability::EffectGuardianShield);
        test_element(Ability::EffectHeal);
        test_element(Ability::EffectHunterSeekerMissile);
        test_element(Ability::EffectImmortalBarrier);
        test_element(Ability::EffectInfestedTerrans);
        test_element(Ability::EffectInjectLarva);
        test_element(Ability::EffectKd8Charge);
        test_element(Ability::EffectLockOn);
        test_element(Ability::EffectLocustSwoop);
        test_element(Ability::EffectMassRecall);
        test_element(Ability::EffectMassRecallMothership);
        test_element(Ability::EffectMassRecallMothershipCore);
        test_element(Ability::EffectMedivacIgniteAfterBurners);
        test_element(Ability::EffectNeuralParasite);
        test_element(Ability::EffectNukeCallDown);
        test_element(Ability::EffectOracleRevelation);
        test_element(Ability::EffectParasiticBomb);
        test_element(Ability::EffectPhotonOvercharge);
        test_element(Ability::EffectPointDefenseDrone);
        test_element(Ability::EffectPsiStorm);
        test_element(Ability::EffectPurificationNova);
        test_element(Ability::EffectRepair);
        test_element(Ability::EffectRepairMule);
        test_element(Ability::EffectRepairScv);
        test_element(Ability::EffectSalvage);
        test_element(Ability::EffectScan);
        test_element(Ability::EffectShadowStride);
        test_element(Ability::EffectSpawnChangeling);
        test_element(Ability::EffectSpawnLocusts);
        test_element(Ability::EffectSpray);
        test_element(Ability::EffectSprayProtoss);
        test_element(Ability::EffectSprayTerran);
        test_element(Ability::EffectSprayZerg);
        test_element(Ability::EffectStim);
        test_element(Ability::EffectStimMarauder);
        test_element(Ability::EffectStimMarine);
        test_element(Ability::EffectStimMarineRedirect);
        test_element(Ability::EffectSupplyDrop);
        test_element(Ability::EffectTacticalJump);
        test_element(Ability::EffectTempestDisruptionBlast);
        test_element(Ability::EffectTimeWarp);
        test_element(Ability::EffectTransfusion);
        test_element(Ability::EffectViperConsume);
        test_element(Ability::EffectVoidRayPrismaticAlignment);
        test_element(Ability::EffectWidowMineAttack);
        test_element(Ability::EffectYamatoGun);

        test_element(Ability::HallucinationAdept);
        test_element(Ability::HallucinationArchon);
        test_element(Ability::HallucinationColossus);
        test_element(Ability::HallucinationDisruptor);
        test_element(Ability::HallucinationHighTemplar);
        test_element(Ability::HallucinationImmortal);
        test_element(Ability::HallucinationOracle);
        test_element(Ability::HallucinationPhoenix);
        test_element(Ability::HallucinationProbe);
        test_element(Ability::HallucinationStalker);
        test_element(Ability::HallucinationVoidRay);
        test_element(Ability::HallucinationWarpPrism);
        test_element(Ability::HallucinationZealot);

        test_element(Ability::Halt);
        test_element(Ability::HaltBuilding);
        test_element(Ability::HaltTerranBuild);

        test_element(Ability::HarvestGather);
        test_element(Ability::HarvestGatherDrone);
        test_element(Ability::HarvestGatherProbe);
        test_element(Ability::HarvestGatherScv);
        test_element(Ability::HarvestReturn);
        test_element(Ability::HarvestReturnDrone);
        test_element(Ability::HarvestReturnMule);
        test_element(Ability::HarvestReturnProbe);
        test_element(Ability::HarvestReturnScv);

        test_element(Ability::HoldPosition);

        test_element(Ability::Land);
        test_element(Ability::LandBarracks);
        test_element(Ability::LandCommandCenter);
        test_element(Ability::LandFactory);
        test_element(Ability::LandOrbitalCommand);
        test_element(Ability::LandStarport);

        test_element(Ability::Lift);
        test_element(Ability::LiftBarracks);
        test_element(Ability::LiftCommandCenter);
        test_element(Ability::LiftFactory);
        test_element(Ability::LiftOrbitalCommand);
        test_element(Ability::LiftStarport);

        test_element(Ability::Load);
        test_element(Ability::LoadAll);
        test_element(Ability::LoadAllCommandCenter);
        test_element(Ability::LoadBunker);
        test_element(Ability::LoadMedivac);

        test_element(Ability::MorphArchon);
        test_element(Ability::MorphBroodLord);
        test_element(Ability::MorphGateway);
        test_element(Ability::MorphGreaterSpire);
        test_element(Ability::MorphHellbat);
        test_element(Ability::MorphHellion);
        test_element(Ability::MorphHive);
        test_element(Ability::MorphLair);
        test_element(Ability::MorphLiberatorAaMode);
        test_element(Ability::MorphLiberatorAgMode);
        test_element(Ability::MorphLurker);
        test_element(Ability::MorphLurkerDen);
        test_element(Ability::MorphMothership);
        test_element(Ability::MorphOrbitalCommand);
        test_element(Ability::MorphOverlordTransport);
        test_element(Ability::MorphOverseer);
        test_element(Ability::MorphPlanetaryFortress);
        test_element(Ability::MorphRavager);
        test_element(Ability::MorphRoot);
        test_element(Ability::MorphSiegeMode);
        test_element(Ability::MorphSpineCrawlerRoot);
        test_element(Ability::MorphSpineCrawlerUproot);
        test_element(Ability::MorphSporeCrawlerRoot);
        test_element(Ability::MorphSporeCrawlerUproot);
        test_element(Ability::MorphSupplyDepotLower);
        test_element(Ability::MorphSupplyDepotRaise);
        test_element(Ability::MorphThorExplosiveMode);
        test_element(Ability::MorphThorHighImpactMode);
        test_element(Ability::MorphUnsiege);
        test_element(Ability::MorphUproot);
        test_element(Ability::MorphVikingAssaultMode);
        test_element(Ability::MorphVikingFighterMode);
        test_element(Ability::MorphWarpGate);
        test_element(Ability::MorphWarpPrismPhasingMode);
        test_element(Ability::MorphWarpPrismTransportMode);

        test_element(Ability::Move);
        test_element(Ability::Patrol);
        test_element(Ability::RallyBuilding);
        test_element(Ability::RallyCommandCenter);
        test_element(Ability::RallyHatcheryUnits);
        test_element(Ability::RallyHatcheryWorkers);
        test_element(Ability::RallyMorphingUnit);
        test_element(Ability::RallyNexus);
        test_element(Ability::RallyUnits);
        test_element(Ability::RallyWorkers);
        test_element(Ability::ResearchAdeptResonatingGlaives);
        test_element(Ability::ResearchAdvancedBallistics);
        test_element(Ability::ResearchBansheeCloakingField);
        test_element(Ability::ResearchBansheeHyperFlightRotors);
        test_element(Ability::ResearchBattleCruiserWeaponRefit);
        test_element(Ability::ResearchBlink);
        test_element(Ability::ResearchBurrow);
        test_element(Ability::ResearchCentrifugalHooks);
        test_element(Ability::ResearchCharge);
        test_element(Ability::ResearchChitinousPlating);
        test_element(Ability::ResearchCombatShield);
        test_element(Ability::ResearchConcussiveShells);
        test_element(Ability::ResearchDrillingClaws);
        test_element(Ability::ResearchExtendedThermalLance);
        test_element(Ability::ResearchGlialRegeneration);
        test_element(Ability::ResearchGraviticBooster);
        test_element(Ability::ResearchGraviticDrive);
        test_element(Ability::ResearchGroovedSpines);
        test_element(Ability::ResearchHighCapacityFuelTanks);
        test_element(Ability::ResearchHisecAutoTracking);
        test_element(Ability::ResearchInfernalPreIgniter);
        test_element(Ability::ResearchInterceptorGravitonCatapult);
        test_element(Ability::ResearchMagFieldLaunchers);
        test_element(Ability::ResearchMuscularAugments);
        test_element(Ability::ResearchNeoSteelFrame);
        test_element(Ability::ResearchNeuralParasite);
        test_element(Ability::ResearchPathogenGlands);
        test_element(Ability::ResearchPersonalCloaking);
        test_element(Ability::ResearchPhoenixAnionPulseCrystals);
        test_element(Ability::ResearchPneumatizedCarapace);
        test_element(Ability::ResearchProtossAirArmor);
        test_element(Ability::ResearchProtossAirArmorLevel1);
        test_element(Ability::ResearchProtossAirArmorLevel2);
        test_element(Ability::ResearchProtossAirArmorLevel3);
        test_element(Ability::ResearchProtossAirWeapons);
        test_element(Ability::ResearchProtossAirWeaponsLevel1);
        test_element(Ability::ResearchProtossAirWeaponsLevel2);
        test_element(Ability::ResearchProtossAirWeaponsLevel3);
        test_element(Ability::ResearchProtossGroundArmor);
        test_element(Ability::ResearchProtossGroundArmorLevel1);
        test_element(Ability::ResearchProtossGroundArmorLevel2);
        test_element(Ability::ResearchProtossGroundArmorLevel3);
        test_element(Ability::ResearchProtossGroundWeapons);
        test_element(Ability::ResearchProtossGroundWeaponsLevel1);
        test_element(Ability::ResearchProtossGroundWeaponsLevel2);
        test_element(Ability::ResearchProtossGroundWeaponsLevel3);
        test_element(Ability::ResearchProtossShields);
        test_element(Ability::ResearchProtossShieldsLevel1);
        test_element(Ability::ResearchProtossShieldsLevel2);
        test_element(Ability::ResearchProtossShieldsLevel3);
        test_element(Ability::ResearchPsiStorm);
        test_element(Ability::ResearchRavenCorvidReactor);
        test_element(Ability::ResearchRavenRecalibratedExplosives);
        test_element(Ability::ResearchShadowStrike);
        test_element(Ability::ResearchStimpack);
        test_element(Ability::ResearchTerranInfantryArmor);
        test_element(Ability::ResearchTerranInfantryArmorLevel1);
        test_element(Ability::ResearchTerranInfantryArmorLevel2);
        test_element(Ability::ResearchTerranInfantryArmorLevel3);
        test_element(Ability::ResearchTerranInfantryWeapons);
        test_element(Ability::ResearchTerranInfantryWeaponsLevel1);
        test_element(Ability::ResearchTerranInfantryWeaponsLevel2);
        test_element(Ability::ResearchTerranInfantryWeaponsLevel3);
        test_element(Ability::ResearchTerranShipWeapons);
        test_element(Ability::ResearchTerranShipWeaponsLevel1);
        test_element(Ability::ResearchTerranShipWeaponsLevel2);
        test_element(Ability::ResearchTerranShipWeaponsLevel3);
        test_element(Ability::ResearchTerranStructureArmorUpgrade);
        test_element(Ability::ResearchTerranVehicleAndShipPlating);
        test_element(Ability::ResearchTerranVehicleAndShipPlatingLevel1);
        test_element(Ability::ResearchTerranVehicleAndShipPlatingLevel2);
        test_element(Ability::ResearchTerranVehicleAndShipPlatingLevel3);
        test_element(Ability::ResearchTerranVehicleWeapons);
        test_element(Ability::ResearchTerranVehicleWeaponsLevel1);
        test_element(Ability::ResearchTerranVehicleWeaponsLevel2);
        test_element(Ability::ResearchTerranVehicleWeaponsLevel3);
        test_element(Ability::ResearchTunnelingClaws);
        test_element(Ability::ResearchWarpGate);
        test_element(Ability::ResearchZergFlyerArmor);
        test_element(Ability::ResearchZergFlyerArmorLevel1);
        test_element(Ability::ResearchZergFlyerArmorLevel2);
        test_element(Ability::ResearchZergFlyerArmorLevel3);
        test_element(Ability::ResearchZergFlyerAttack);
        test_element(Ability::ResearchZergFlyerAttackLevel1);
        test_element(Ability::ResearchZergFlyerAttackLevel2);
        test_element(Ability::ResearchZergFlyerAttackLevel3);
        test_element(Ability::ResearchZergGroundArmor);
        test_element(Ability::ResearchZergGroundArmorLevel1);
        test_element(Ability::ResearchZergGroundArmorLevel2);
        test_element(Ability::ResearchZergGroundArmorLevel3);
        test_element(Ability::ResearchZerglingAdrenalGlands);
        test_element(Ability::ResearchZerglingMetabolicBoost);
        test_element(Ability::ResearchZergMeleeWeapons);
        test_element(Ability::ResearchZergMeleeWeaponsLevel1);
        test_element(Ability::ResearchZergMeleeWeaponsLevel2);
        test_element(Ability::ResearchZergMeleeWeaponsLevel3);
        test_element(Ability::ResearchZergMissileWeapons);
        test_element(Ability::ResearchZergMissileWeaponsLevel1);
        test_element(Ability::ResearchZergMissileWeaponsLevel2);
        test_element(Ability::ResearchZergMissileWeaponsLevel3);

        test_element(Ability::ScanMove);

        test_element(Ability::Stop);
        test_element(Ability::StopBuilding);
        test_element(Ability::StopAndCheer);
        test_element(Ability::StopAndDance);
        test_element(Ability::StopRedirect);
        test_element(Ability::StopStop);

        test_element(Ability::TrainWarpAdept);
        test_element(Ability::TrainWarpDarkTemplar);
        test_element(Ability::TrainWarpHighTemplar);
        test_element(Ability::TrainWarpSentry);
        test_element(Ability::TrainWarpStalker);
        test_element(Ability::TrainWarpZealot);

        test_element(Ability::TrainAdept);
        test_element(Ability::TrainBaneling);
        test_element(Ability::TrainBanshee);
        test_element(Ability::TrainBattleCruiser);
        test_element(Ability::TrainCarrier);
        test_element(Ability::TrainColossus);
        test_element(Ability::TrainCorruptor);
        test_element(Ability::TrainCyclone);
        test_element(Ability::TrainDarkTemplar);
        test_element(Ability::TrainDisruptor);
        test_element(Ability::TrainDrone);
        test_element(Ability::TrainGhost);
        test_element(Ability::TrainHellbat);
        test_element(Ability::TrainHellion);
        test_element(Ability::TrainHighTemplar);
        test_element(Ability::TrainHydralisk);
        test_element(Ability::TrainImmortal);
        test_element(Ability::TrainInfestor);
        test_element(Ability::TrainLiberator);
        test_element(Ability::TrainMarauder);
        test_element(Ability::TrainMarine);
        test_element(Ability::TrainMedivac);
        test_element(Ability::TrainMothershipCore);
        test_element(Ability::TrainMutalisk);
        test_element(Ability::TrainObserver);
        test_element(Ability::TrainOracle);
        test_element(Ability::TrainOverlord);
        test_element(Ability::TrainPhoenix);
        test_element(Ability::TrainProbe);
        test_element(Ability::TrainQueen);
        test_element(Ability::TrainRaven);
        test_element(Ability::TrainReaper);
        test_element(Ability::TrainRoach);
        test_element(Ability::TrainScv);
        test_element(Ability::TrainSentry);
        test_element(Ability::TrainSiegeTank);
        test_element(Ability::TrainStalker);
        test_element(Ability::TrainSwarmHost);
        test_element(Ability::TrainTempest);
        test_element(Ability::TrainThor);
        test_element(Ability::TrainUltralisk);
        test_element(Ability::TrainVikingFighter);
        test_element(Ability::TrainViper);
        test_element(Ability::TrainVoidRay);
        test_element(Ability::TrainWarpPrism);
        test_element(Ability::TrainWidowMine);
        test_element(Ability::TrainZealot);
        test_element(Ability::TrainZergling);

        test_element(Ability::UnloadAll);
        test_element(Ability::UnloadAllAt);
        test_element(Ability::UnloadAllAtMedivac);
        test_element(Ability::UnloadAllAtOverlord);
        test_element(Ability::UnloadAllAtWarpPrism);
        test_element(Ability::UnloadAllBunker);
        test_element(Ability::UnloadAllCommandCenter);
        test_element(Ability::UnloadAllNydusNetwork);
        test_element(Ability::UnloadAllNydusWorm);
        test_element(Ability::UnloadUnitBunker);
        test_element(Ability::UnloadUnitCommandCenter);
        test_element(Ability::UnloadUnitMedivac);
        test_element(Ability::UnloadUnitNydusNetwork);
        test_element(Ability::UnloadUnitOverlord);
        test_element(Ability::UnloadUnitWarpPrism);
    }
}
