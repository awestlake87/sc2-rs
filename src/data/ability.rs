
#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum Ability {
    Invalid,
    Smart,                                      // Target: Unit, Point.

    Attack,                                     // Target: Unit, Point.
    AttackAttack,                               // Target: Unit, Point.
    AttackAttackBuilding,                       // Target: Unit, Point.
    AttackAttackRedirect,                       // Target: Unit, Point.

    BehaviorBuildingAttackOff,                  // Target: None.
    BehaviorBuildingAttackOn,                   // Target: None.
    BehaviorCloakOff,                           // Target: None.
    BehaviorCloakOffBanshee,                    // Target: None.
    BehaviorCloakOffGhost,                      // Target: None.
    BehaviorCloakOn,                            // Target: None.
    BehaviorCloakOnBanshee,                     // Target: None.
    BehaviorCloakOnGhost,                       // Target: None.
    BehaviorGenerateCreepOff,                   // Target: None.
    BehaviorGenerateCreepOn,                    // Target: None.
    BehaviorHoldFireOff,                        // Target: None.
    BehaviorHoldFireOffLurker,                  // Target: None.
    BehaviorHoldFireOn,                         // Target: None.
    BehaviorHoldFireOnGhost,                    // Target: None.
    BehaviorHoldFireOnLurker,                   // Target: None.
    BehaviorPulsarBeamOff,                      // Target: None.
    BehaviorPulsarBeamOn,                       // Target: None.

    BuildArmory,                                // Target: Point.
    BuildAssimilator,                           // Target: Unit.
    BuildBanelingNest,                          // Target: Point.
    BuildBarracks,                              // Target: Point.
    BuildBunker,                                // Target: Point.
    BuildCommandCenter,                         // Target: Point.
    BuildCreepTumor,                            // Target: Point.
    BuildCreepTumorQueen,                       // Target: Point.
    BuildCreepTumorTumor,                       // Target: Point.
    BuildCyberneticsCore,                       // Target: Point.
    BuildDarkShrine,                            // Target: Point.
    BuildEngineeringBay,                        // Target: Point.
    BuildEvolutionChamber,                      // Target: Point.
    BuildExtractor,                             // Target: Unit.
    BuildFactory,                               // Target: Point.
    BuildFleetBeacon,                           // Target: Point.
    BuildForge,                                 // Target: Point.
    BuildFusionCore,                            // Target: Point.
    BuildGateway,                               // Target: Point.
    BuildGhostAcademy,                          // Target: Point.
    BuildHatchery,                              // Target: Point.
    BuildHydraliskDen,                          // Target: Point.
    BuildInfestationPit,                        // Target: Point.
    BuildInterceptors,                          // Target: None.
    BuildMissileTurret,                         // Target: Point.
    BuildNexus,                                 // Target: Point.
    BuildNuke,                                  // Target: None.
    BuildNydusNetwork,                          // Target: Point.
    BuildNydusWorm,                             // Target: Point.
    BuildPhotonCannon,                          // Target: Point.
    BuildPylon,                                 // Target: Point.
    BuildReactor,                               // Target: None.
    BuildReactorBarracks,                       // Target: None.
    BuildReactorFactory,                        // Target: None.
    BuildReactorStarport,                       // Target: None.
    BuildRefinery,                              // Target: Unit.
    BuildRoachWarren,                           // Target: Point.
    BuildRoboticsBay,                           // Target: Point.
    BuildRoboticsFacility,                      // Target: Point.
    BuildSensorTower,                           // Target: Point.
    BuildSpawningPool,                          // Target: Point.
    BuildSpineCrawler,                          // Target: Point.
    BuildSpire,                                 // Target: Point.
    BuildSporeCrawler,                          // Target: Point.
    BuildStarGate,                              // Target: Point.
    BuildStarport,                              // Target: Point.
    BuildStasisTrap,                            // Target: Point.
    BuildSupplyDepot,                           // Target: Point.
    BuildTechLab,                               // Target: None.
    BuildTechLabBarracks,                       // Target: None.
    BuildTechLabFactory,                        // Target: None.
    BuildTechLabStarport,                       // Target: None.
    BuildTemplarArchive,                        // Target: Point.
    BuildTwilightCouncil,                       // Target: Point.
    BuildUltraliskCavern,                       // Target: Point.

    BurrowDown,                                 // Target: None.
    BurrowDownBaneling,                         // Target: None.
    BurrowDownDrone,                            // Target: None.
    BurrowDownHydralisk,                        // Target: None.
    BurrowDownInfestor,                         // Target: None.
    BurrowDownLurker,                           // Target: None.
    BurrowDownQueen,                            // Target: None.
    BurrowDownRavager,                          // Target: None.
    BurrowDownRoach,                            // Target: None.
    BurrowDownSwarmHost,                        // Target: None.
    BurrowDownWidowMine,                        // Target: None.
    BurrowDownZergling,                         // Target: None.

    BurrowUp,                                   // Target: None.
    BurrowUpBaneling,                           // Target: None.
    BurrowUpDrone,                              // Target: None.
    BurrowUpHydralisk,                          // Target: None.
    BurrowUpInfestor,                           // Target: None.
    BurrowUpLurker,                             // Target: None.
    BurrowUpQueen,                              // Target: None.
    BurrowUpRavager,                            // Target: None.
    BurrowUpRoach,                              // Target: None.
    BurrowUpSwarmHost,                          // Target: None.
    BurrowUpWidowMine,                          // Target: None.
    BurrowUpZergling,                           // Target: None.

    Cancel,                                     // Target: None.
    CancelSlotAddOn,                            // Target: None.
    CancelSlotQueue1,                           // Target: None.
    CancelSlotQueue5,                           // Target: None.
    CancelSlotQueueToSelection,                 // Target: None.
    CancelSlotQueuePassive,                     // Target: None.
    CancelAdeptPhaseShift,                      // Target: None.
    CancelAdeptShadePhaseShift,                 // Target: None.
    CancelBarracksAddOn,                        // Target: None.
    CancelBuildInProgress,                      // Target: None.
    CancelCreepTumor,                           // Target: None.
    CancelFactoryAddOn,                         // Target: None.
    CancelGravitonBeam,                         // Target: None.
    CancelLast,                                 // Target: None.
    CancelMorphBroodLord,                       // Target: None.
    CancelMorphLair,                            // Target: None.
    CancelMorphLurker,                          // Target: None.
    CancelMorphLurkerDen,                       // Target: None.
    CancelMorphMothership,                      // Target: None.
    CancelMorphOrbital,                         // Target: None.
    CancelMorphOverlordTransport,               // Target: None.
    CancelMorphOverseer,                        // Target: None.
    CancelMorphPlanetaryFortress,               // Target: None.
    CancelMorphRavager,                         // Target: None.
    CancelQueue1,                               // Target: None.
    CancelQueue5,                               // Target: None.
    CancelQueueAddOn,                           // Target: None.
    CancelQueueToSelection,                     // Target: None.
    CancelQueuePassive,                         // Target: None.
    CancelQueuePassiveToSelection,              // Target: None.
    CancelSpineCrawlerRoot,                     // Target: None.
    CancelStarportAddOn,                        // Target: None.

    EffectAbduct,                               // Target: Unit.
    EffectAdeptPhaseShift,                      // Target: Point.
    EffectAutoTurret,                           // Target: Point.
    EffectBlindingCloud,                        // Target: Point.
    EffectBlink,                                // Target: Point.
    EffectBlinkStalker,                         // Target: Point.
    EffectCallDownMule,                         // Target: Unit, Point.
    EffectCausticSpray,                         // Target: Unit.
    EffectCharge,                               // Target: Unit.
    EffectChronoBoost,                          // Target: Unit.
    EffectContaminate,                          // Target: Unit.
    EffectCorrosiveBile,                        // Target: Point.
    EffectEmp,                                  // Target: Point.
    EffectExplode,                              // Target: None.
    EffectFeedback,                             // Target: Unit.
    EffectForceField,                           // Target: Point.
    EffectFungalGrowth,                         // Target: Point.
    EffectGhostSnipe,                           // Target: Unit.
    EffectGravitonBeam,                         // Target: Unit.
    EffectGuardianShield,                       // Target: None.
    EffectHeal,                                 // Target: Unit.
    EffectHunterSeekerMissile,                  // Target: Unit.
    EffectImmortalBarrier,                      // Target: None.
    EffectInfestedTerrans,                      // Target: Point.
    EffectInjectLarva,                          // Target: Unit.
    EffectKd8Charge,                            // Target: Unit, Point.
    EffectLockOn,                               // Target: Unit.
    EffectLocustSwoop,                          // Target: Point.
    EffectMassRecall,                           // Target: Unit.
    EffectMassRecallMothership,                 // Target: Unit.
    EffectMassRecallMothershipCore,             // Target: Unit.
    EffectMedivacIgniteAfterBurners,            // Target: None.
    EffectNeuralParasite,                       // Target: Unit.
    EffectNukeCallDown,                         // Target: Point.
    EffectOracleRevelation,                     // Target: Point.
    EffectParasiticBomb,                        // Target: Unit.
    EffectPhotonOvercharge,                     // Target: Unit.
    EffectPointDefenseDrone,                    // Target: Point.
    EffectPsiStorm,                             // Target: Point.
    EffectPurificationNova,                     // Target: Point.
    EffectRepair,                               // Target: Unit.
    EffectRepairMule,                           // Target: Unit.
    EffectRepairScv,                            // Target: Unit.
    EffectSalvage,                              // Target: None.
    EffectScan,                                 // Target: Point.
    EffectShadowStride,                         // Target: Point.
    EffectSpawnChangeling,                      // Target: None.
    EffectSpawnLocusts,                         // Target: Point.
    EffectSpray,                                // Target: Point.
    EffectSprayProtoss,                         // Target: Point.
    EffectSprayTerran,                          // Target: Point.
    EffectSprayZerg,                            // Target: Point.
    EffectStim,                                 // Target: None.
    EffectStimMarauder,                         // Target: None.
    EffectStimMarine,                           // Target: None.
    EffectStimMarineRedirect,                   // Target: None.
    EffectSupplyDrop,                           // Target: Unit.
    EffectTacticalJump,                         // Target: Point.
    EffectTempestDisruptionBlast,               // Target: Point.
    EffectTimeWarp,                             // Target: Point.
    EffectTransfusion,                          // Target: Unit.
    EffectViperConsume,                         // Target: Unit.
    EffectVoidRayPrismaticalAlignment,          // Target: None.
    EffectWidowMineAttack,                      // Target: Unit.
    EffectYamatoGun,                            // Target: Unit.

    HallucinationAdept,                         // Target: None.
    HallucinationArchon,                        // Target: None.
    HallucinationColossus,                      // Target: None.
    HallucinationDisruptor,                     // Target: None.
    HallucinationHighTemplar,                   // Target: None.
    HallucinationImmortal,                      // Target: None.
    HallucinationOracle,                        // Target: None.
    HallucinationPhoenix,                       // Target: None.
    HallucinationProbe,                         // Target: None.
    HallucinationStalker,                       // Target: None.
    HallucinationVoidRay,                       // Target: None.
    HallucinationWarpPrism,                     // Target: None.
    HallucinationZealot,                        // Target: None.

    Halt,                                       // Target: None.
    HaltBuilding,                               // Target: None.
    HaltTerranBuild,                            // Target: None.

    HarvestGather,                              // Target: Unit.
    HarvestGatherDrone,                         // Target: Unit.
    HarvestGatherProbe,                         // Target: Unit.
    HarvestGatherScv,                           // Target: Unit.
    HarvestReturn,                              // Target: None.
    HarvestReturnDrone,                         // Target: None.
    HarvestReturnMule,                          // Target: None.
    HarvestReturnProbe,                         // Target: None.
    HarvestReturnScv,                           // Target: None.

    HoldPosition,                               // Target: None.

    Land,                                       // Target: Point.
    LandBarracks,                               // Target: Point.
    LandCommandCenter,                          // Target: Point.
    LandFactory,                                // Target: Point.
    LandOrbitalCommand,                         // Target: Point.
    LandStarport,                               // Target: Point.

    Lift,                                       // Target: None.
    LiftBarracks,                               // Target: None.
    LiftCommandCenter,                          // Target: None.
    LiftFactory,                                // Target: None.
    LiftOrbitalCommand,                         // Target: None.
    LiftStarport,                               // Target: None.

    Load,                                       // Target: Unit.
    LoadAll,                                    // Target: None.
    LoadAllCommandCenter,                       // Target: None.
    LoadBunker,                                 // Target: Unit.
    LoadMedivac,                                // Target: Unit.

    MorphArchon,                                // Target: None.
    MorphBroodLord,                             // Target: None.
    MorphGateway,                               // Target: None.
    MorphGreaterSpire,                          // Target: None.
    MorphHellBat,                               // Target: None.
    MorphHellion,                               // Target: None.
    MorphHive,                                  // Target: None.
    MorphLair,                                  // Target: None.
    MorphLiberatorAaMode,                       // Target: None.
    MorphLiberatorAgMode,                       // Target: Point.
    MorphLurker,                                // Target: None.
    MorphLurkerDen,                             // Target: None.
    MorphMothership,                            // Target: None.
    MorphOrbitalCommand,                        // Target: None.
    MorphOverlordTransport,                     // Target: None.
    MorphOverseer,                              // Target: None.
    MorphPlanetaryFortress,                     // Target: None.
    MorphRavager,                               // Target: None.
    MorphRoot,                                  // Target: Point.
    MorphSiegeMode,                             // Target: None.
    MorphSpineCrawlerRoot,                      // Target: Point.
    MorphSpineCrawlerUproot,                    // Target: None.
    MorphSporeCrawlerRoot,                      // Target: Point.
    MorphSporeCrawlerUproot,                    // Target: None.
    MorphSupplyDepotLower,                      // Target: None.
    MorphSupplyDepotRaise,                      // Target: None.
    MorphThorExplosiveMode,                     // Target: None.
    MorphThorHighImpactMode,                    // Target: None.
    MorphUnsiege,                               // Target: None.
    MorphUproot,                                // Target: None.
    MorphVikingAssaultMode,                     // Target: None.
    MorphVikingFighterMode,                     // Target: None.
    MorphWarpGate,                              // Target: None.
    MorphWarpPrismPhasingMode,                  // Target: None.
    MorphWarpPrismTransportMode,                // Target: None.

    Move,                                       // Target: Unit, Point.
    Patrol,                                     // Target: Unit, Point.
    RallyBuilding,                              // Target: Unit, Point.
    RallyCommandCenter,                         // Target: Unit, Point.
    RallyHatcheryUnits,                         // Target: Unit, Point.
    RallyHatcheryWorkers,                       // Target: Unit, Point.
    RallyMorphingUnit,                          // Target: Unit, Point.
    RallyNexus,                                 // Target: Unit, Point.
    RallyUnits,                                 // Target: Unit, Point.
    RallyWorkers,                               // Target: Unit, Point.

    ResearchAdeptResonatingGlaives,             // Target: None.
    ResearchAdvancedBallistics,                 // Target: None.
    ResearchBansheeCloakingField,               // Target: None.
    ResearchBansheeHyperFlightRotors,           // Target: None.
    ResearchBattleCruiserWeaponReFit,           // Target: None.
    ResearchBlink,                              // Target: None.
    ResearchBurrow,                             // Target: None.
    ResearchCentrifugalHooks,                   // Target: None.
    ResearchCharge,                             // Target: None.
    ResearchChitinousPlating,                   // Target: None.
    ResearchCombatShield,                       // Target: None.
    ResearchConcussiveShells,                   // Target: None.
    ResearchDrillingClaws,                      // Target: None.
    ResearchExtendedThermalLance,               // Target: None.
    ResearchGlialRegeneration,                  // Target: None.
    ResearchGraviticBooster,                    // Target: None.
    ResearchGraviticDrive,                      // Target: None.
    ResearchGroovedSpines,                      // Target: None.
    ResearchHighCapacityFuelTanks,              // Target: None.
    ResearchHisecAutoTracking,                  // Target: None.
    ResearchInfernalPreIgniter,                 // Target: None.
    ResearchInterceptorGravitonCatapult,        // Target: None.
    ResearchMagFieldLaunchers,                  // Target: None.
    ResearchMuscularAugments,                   // Target: None.
    ResearchNeoSteelFrame,                      // Target: None.
    ResearchNeuralParasite,                     // Target: None.
    ResearchPathogenGlands,                     // Target: None.
    ResearchPersonalCloaking,                   // Target: None.
    ResearchPhoenixAnionPulseCrystals,          // Target: None.
    ResearchPneumatizedCarapace,                // Target: None.
    ResearchProtossAirArmor,                    // Target: None.
    ResearchProtossAirArmorLevel1,              // Target: None.
    ResearchProtossAirArmorLevel2,              // Target: None.
    ResearchProtossAirArmorLevel3,              // Target: None.
    ResearchProtossAirWeapons,                  // Target: None.
    ResearchProtossAirWeaponsLevel1,            // Target: None.
    ResearchProtossAirWeaponsLevel2,            // Target: None.
    ResearchProtossAirWeaponsLevel3,            // Target: None.
    ResearchProtossGroundArmor,                 // Target: None.
    ResearchProtossGroundArmorLevel1,           // Target: None.
    ResearchProtossGroundArmorLevel2,           // Target: None.
    ResearchProtossGroundArmorLevel3,           // Target: None.
    ResearchProtossGroundWeapons,               // Target: None.
    ResearchProtossGroundWeaponsLevel1,         // Target: None.
    ResearchProtossGroundWeaponsLevel2,         // Target: None.
    ResearchProtossGroundWeaponsLevel3,         // Target: None.
    ResearchProtossShields,                     // Target: None.
    ResearchProtossShieldsLevel1,               // Target: None.
    ResearchProtossShieldsLevel2,               // Target: None.
    ResearchProtossShieldsLevel3,               // Target: None.
    ResearchPsiStorm,                           // Target: None.
    ResearchRavenCorvidReactor,                 // Target: None.
    ResearchRavenRecalibratedExplosives,        // Target: None.
    ResearchShadowStrike,                       // Target: None.
    ResearchStimpack,                           // Target: None.
    ResearchTerranInfantryArmor,                // Target: None.
    ResearchTerranInfantryArmorLevel1,          // Target: None.
    ResearchTerranInfantryArmorLevel2,          // Target: None.
    ResearchTerranInfantryArmorLevel3,          // Target: None.
    ResearchTerranInfantryWeapons,              // Target: None.
    ResearchTerranInfantryWeaponsLevel1,        // Target: None.
    ResearchTerranInfantryWeaponsLevel2,        // Target: None.
    ResearchTerranInfantryWeaponsLevel3,        // Target: None.
    ResearchTerranShipWeapons,                  // Target: None.
    ResearchTerranShipWeaponsLevel1,            // Target: None.
    ResearchTerranShipWeaponsLevel2,            // Target: None.
    ResearchTerranShipWeaponsLevel3,            // Target: None.
    ResearchTerranStructureArmorUpgrade,        // Target: None.
    ResearchTerranVehicleAndShipPlating,        // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel1,  // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel2,  // Target: None.
    ResearchTerranVehicleAndShipPlatingLevel3,  // Target: None.
    ResearchTerranVehicleWeapons,               // Target: None.
    ResearchTerranVehicleWeaponsLevel1,         // Target: None.
    ResearchTerranVehicleWeaponsLevel2,         // Target: None.
    ResearchTerranVehicleWeaponsLevel3,         // Target: None.
    ResearchTunnelingClaws,                     // Target: None.
    ResearchWarpGate,                           // Target: None.
    ResearchZergFlyerArmor,                     // Target: None.
    ResearchZergFlyerArmorLevel1,               // Target: None.
    ResearchZergFlyerArmorLevel2,               // Target: None.
    ResearchZergFlyerArmorLevel3,               // Target: None.
    ResearchZergFlyerAttack,                    // Target: None.
    ResearchZergFlyerAttackLevel1,              // Target: None.
    ResearchZergFlyerAttackLevel2,              // Target: None.
    ResearchZergFlyerAttackLevel3,              // Target: None.
    ResearchZergGroundArmor,                    // Target: None.
    ResearchZergGroundArmorLevel1,              // Target: None.
    ResearchZergGroundArmorLevel2,              // Target: None.
    ResearchZergGroundArmorLevel3,              // Target: None.
    ResearchZerglingAdrenalGlands,              // Target: None.
    ResearchZerglingMetabolicBoost,             // Target: None.
    ResearchZergMeleeWeapons,                   // Target: None.
    ResearchZergMeleeWeaponsLevel1,             // Target: None.
    ResearchZergMeleeWeaponsLevel2,             // Target: None.
    ResearchZergMeleeWeaponsLevel3,             // Target: None.
    ResearchZergMissileWeapons,                 // Target: None.
    ResearchZergMissileWeaponsLevel1,           // Target: None.
    ResearchZergMissileWeaponsLevel2,           // Target: None.
    ResearchZergMissileWeaponsLevel3,           // Target: None.

    ScanMove,                                   // Target: Unit, Point.

    Stop,                                       // Target: None.
    StopBuilding,                               // Target: None.
    StopCheer,                                  // Target: None.
    StopDance,                                  // Target: None.
    StopRedirect,                               // Target: None.

    TrainWarpAdept,                             // Target: Point.
    TrainWarpDarkTemplar,                       // Target: Point.
    TrainWarpHighTemplar,                       // Target: Point.
    TrainWarpSentry,                            // Target: Point.
    TrainWarpStalker,                           // Target: Point.
    TrainWarpZealot,                            // Target: Point.

    TrainAdept,                                 // Target: None.
    TrainBaneling,                              // Target: None.
    TrainBanshee,                               // Target: None.
    TrainBattleCruiser,                         // Target: None.
    TrainCarrier,                               // Target: None.
    TrainColossus,                              // Target: None.
    TrainCorruptor,                             // Target: None.
    TrainCyclone,                               // Target: None.
    TrainDarkTemplar,                           // Target: None.
    TrainDisruptor,                             // Target: None.
    TrainDrone,                                 // Target: None.
    TrainGhost,                                 // Target: None.
    TrainHellBat,                               // Target: None.
    TrainHellion,                               // Target: None.
    TrainHighTemplar,                           // Target: None.
    TrainHydralisk,                             // Target: None.
    TrainImmortal,                              // Target: None.
    TrainInfestor,                              // Target: None.
    TrainLiberator,                             // Target: None.
    TrainMarauder,                              // Target: None.
    TrainMarine,                                // Target: None.
    TrainMedivac,                               // Target: None.
    TrainMothershipCore,                        // Target: None.
    TrainMutalisk,                              // Target: None.
    TrainObserver,                              // Target: None.
    TrainOracle,                                // Target: None.
    TrainOverlord,                              // Target: None.
    TrainPhoenix,                               // Target: None.
    TrainProbe,                                 // Target: None.
    TrainQueen,                                 // Target: None.
    TrainRaven,                                 // Target: None.
    TrainReaper,                                // Target: None.
    TrainRoach,                                 // Target: None.
    TrainScv,                                   // Target: None.
    TrainSentry,                                // Target: None.
    TrainSiegeTank,                             // Target: None.
    TrainStalker,                               // Target: None.
    TrainSwarmHost,                             // Target: None.
    TrainTempest,                               // Target: None.
    TrainThor,                                  // Target: None.
    TrainUltralisk,                             // Target: None.
    TrainVikingFighter,                         // Target: None.
    TrainViper,                                 // Target: None.
    TrainVoidRay,                               // Target: None.
    TrainWarpPrism,                             // Target: None.
    TrainWidowMine,                             // Target: None.
    TrainZealot,                                // Target: None.
    TrainZergling,                              // Target: None.

    UnloadAll,                                  // Target: None.
    UnloadAllAt,                                // Target: Unit, Point.
    UnloadAllAtMedivac,                         // Target: Unit, Point.
    UnloadAllAtOverlord,                        // Target: Unit, Point.
    UnloadAllAtWarpPrism,                       // Target: Unit, Point.
    UnloadAllBunker,                            // Target: None.
    UnloadAllCommandCenter,                     // Target: None.
    UnloadAllNydusNetwork,                      // Target: None.
    UnloadAllNydusWorm,                         // Target: None.
    UnloadUnitBunker,                           // Target: None.
    UnloadUnitCommandCenter,                    // Target: None.
    UnloadUnitMedivac,                          // Target: None.
    UnloadUnitNydusNetwork,                     // Target: None.
    UnloadUnitOverlord,                         // Target: None.
    UnloadUnitWarpPrism,                        // Target: None.
}

impl Ability {
    pub fn from_id(id: u32) -> Self {
        match id {
            _ => Ability::Invalid
        }
    }
}
