
pub enum Buff {
    Invalid,
    GravitonBeam,
    GhostCloak,
    BansheeCloak,
    PowerUserWarpable,
    QueenSpawnLarvaTimer,
    GhostHoldFire,
    GhostHoldFireB,
    EmpDeCloak,
    FungalGrowth,
    GuardianShield,
    TimeWarpProduction,
    NeuralParasite,
    StimpackMarauder,
    SupplyDrop,
    Stimpack,
    PsiStorm,
    CloakFieldEffect,
    Charging,
    Slow,
    Contaminated,
    BlindingCloudStructure,
    OracleRevelation,
    ViperConsumeStructure,
    BlindingCloud,
    MedivacSpeedBoost,
    Purify,
    OracleWeapon,
    ImmortalOverload,
    Lockon,
    SeekerMissile,
    TemporalField,
    VoidRaysWarmDamageBoost,
    OracleStasisTrapTarget,
    ParasiticBomb,
    ParasiticBombUnitKu,
    ParasiticBombSecondaryUnitSearch,
    LurkerHoldFireB,
    ChannelSnipeCombat,
    TempestDisruptionBlastStunBehavior,
    CarryMineralFieldMinerals,
    CarryHighYieldMineralFieldMinerals,
    CarryHarvestableVespeneGeyserGas,
    CarryHarvestableVespeneGeyserGasProtoss,
    CarryHarvestableVespeneGeyserGasZerg,
}

impl Buff {
    pub fn from_id(id: u32) -> Self {
        match id {
            _ => Buff::Invalid
        }
    }
}
