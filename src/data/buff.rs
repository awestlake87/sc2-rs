use sc2_proto::data;

use {ErrorKind, FromProto, IntoProto, Result};

/// A list of known StarCraft II buffs.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Buff {
    GravitonBeam = 5,
    GhostCloak = 6,
    BansheeCloak = 7,
    PowerUserWarpable = 8,
    QueenSpawnLarvaTimer = 11,
    GhostHoldFire = 12,
    GhostHoldFireB = 13,
    EmpDeCloak = 16,
    FungalGrowth = 17,
    GuardianShield = 18,
    TimeWarpProduction = 20,
    NeuralParasite = 22,
    StimpackMarauder = 24,
    SupplyDrop = 25,
    Stimpack = 27,
    PsiStorm = 28,
    CloakFieldEffect = 29,
    Charging = 30,
    Slow = 33,
    Contaminated = 36,
    BlindingCloudStructure = 38,
    OracleRevelation = 49,
    ViperConsumeStructure = 59,
    BlindingCloud = 83,
    MedivacSpeedBoost = 89,
    Purify = 97,
    OracleWeapon = 99,
    ImmortalOverload = 102,
    Lockon = 116,
    SeekerMissile = 120,
    TemporalField = 121,
    VoidRaySwarmDamageBoost = 122,
    OracleStasisTrapTarget = 129,
    ParasiticBomb = 132,
    ParasiticBombUnitKu = 133,
    ParasiticBombSecondaryUnitSearch = 134,
    LurkerHoldFireB = 137,
    ChannelSnipeCombat = 145,
    TempestDisruptionBlastStunBehavior = 146,
    CarryMineralFieldMinerals = 271,
    CarryHighYieldMineralFieldMinerals = 272,
    CarryHarvestableVespeneGeyserGas = 273,
    CarryHarvestableVespeneGeyserGasProtoss = 274,
    CarryHarvestableVespeneGeyserGasZerg = 275,
}

impl FromProto<u32> for Buff {
    /// convert from raw protobuf buff id
    fn from_proto(id: u32) -> Result<Self> {
        Ok(match id {
            5 => Buff::GravitonBeam,
            6 => Buff::GhostCloak,
            7 => Buff::BansheeCloak,
            8 => Buff::PowerUserWarpable,
            11 => Buff::QueenSpawnLarvaTimer,
            12 => Buff::GhostHoldFire,
            13 => Buff::GhostHoldFireB,
            16 => Buff::EmpDeCloak,
            17 => Buff::FungalGrowth,
            18 => Buff::GuardianShield,
            20 => Buff::TimeWarpProduction,
            22 => Buff::NeuralParasite,
            24 => Buff::StimpackMarauder,
            25 => Buff::SupplyDrop,
            27 => Buff::Stimpack,
            28 => Buff::PsiStorm,
            29 => Buff::CloakFieldEffect,
            30 => Buff::Charging,
            33 => Buff::Slow,
            36 => Buff::Contaminated,
            38 => Buff::BlindingCloudStructure,
            49 => Buff::OracleRevelation,
            59 => Buff::ViperConsumeStructure,
            83 => Buff::BlindingCloud,
            89 => Buff::MedivacSpeedBoost,
            97 => Buff::Purify,
            99 => Buff::OracleWeapon,
            102 => Buff::ImmortalOverload,
            116 => Buff::Lockon,
            120 => Buff::SeekerMissile,
            121 => Buff::TemporalField,
            122 => Buff::VoidRaySwarmDamageBoost,
            129 => Buff::OracleStasisTrapTarget,
            132 => Buff::ParasiticBomb,
            133 => Buff::ParasiticBombUnitKu,
            134 => Buff::ParasiticBombSecondaryUnitSearch,
            137 => Buff::LurkerHoldFireB,
            145 => Buff::ChannelSnipeCombat,
            146 => Buff::TempestDisruptionBlastStunBehavior,
            271 => Buff::CarryMineralFieldMinerals,
            272 => Buff::CarryHighYieldMineralFieldMinerals,
            273 => Buff::CarryHarvestableVespeneGeyserGas,
            274 => Buff::CarryHarvestableVespeneGeyserGasProtoss,
            275 => Buff::CarryHarvestableVespeneGeyserGasZerg,

            _ => bail!(ErrorKind::InvalidProtobuf(format!(
                "Buff id({})",
                id
            ))),
        })
    }
}

impl IntoProto<u32> for Buff {
    fn into_proto(self) -> Result<u32> {
        Ok(self as u32)
    }
}

/// Buff data.
#[derive(Debug, Clone)]
pub struct BuffData {
    buff: Buff,
    name: String,
}

impl BuffData {
    /// Stable buff ID.
    pub fn get_id(&self) -> Buff {
        self.buff
    }

    /// Buff name (corresponds to the game's catalog).
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl FromProto<data::BuffData> for BuffData {
    fn from_proto(mut data: data::BuffData) -> Result<Self> {
        Ok(BuffData {
            buff: Buff::from_proto(data.get_buff_id())?,
            name: data.take_name(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invertibility() {
        let test_element = |element: Buff| {
            assert_eq!(
                element,
                Buff::from_proto(element.into_proto().unwrap()).unwrap()
            )
        };

        test_element(Buff::GravitonBeam);
        test_element(Buff::GhostCloak);
        test_element(Buff::BansheeCloak);
        test_element(Buff::PowerUserWarpable);
        test_element(Buff::QueenSpawnLarvaTimer);
        test_element(Buff::GhostHoldFire);
        test_element(Buff::GhostHoldFireB);
        test_element(Buff::EmpDeCloak);
        test_element(Buff::FungalGrowth);
        test_element(Buff::GuardianShield);
        test_element(Buff::TimeWarpProduction);
        test_element(Buff::NeuralParasite);
        test_element(Buff::StimpackMarauder);
        test_element(Buff::SupplyDrop);
        test_element(Buff::Stimpack);
        test_element(Buff::PsiStorm);
        test_element(Buff::CloakFieldEffect);
        test_element(Buff::Charging);
        test_element(Buff::Slow);
        test_element(Buff::Contaminated);
        test_element(Buff::BlindingCloudStructure);
        test_element(Buff::OracleRevelation);
        test_element(Buff::ViperConsumeStructure);
        test_element(Buff::BlindingCloud);
        test_element(Buff::MedivacSpeedBoost);
        test_element(Buff::Purify);
        test_element(Buff::OracleWeapon);
        test_element(Buff::ImmortalOverload);
        test_element(Buff::Lockon);
        test_element(Buff::SeekerMissile);
        test_element(Buff::TemporalField);
        test_element(Buff::VoidRaySwarmDamageBoost);
        test_element(Buff::OracleStasisTrapTarget);
        test_element(Buff::ParasiticBomb);
        test_element(Buff::ParasiticBombUnitKu);
        test_element(Buff::ParasiticBombSecondaryUnitSearch);
        test_element(Buff::LurkerHoldFireB);
        test_element(Buff::ChannelSnipeCombat);
        test_element(Buff::TempestDisruptionBlastStunBehavior);
        test_element(Buff::CarryMineralFieldMinerals);
        test_element(Buff::CarryHighYieldMineralFieldMinerals);
        test_element(Buff::CarryHarvestableVespeneGeyserGas);
        test_element(Buff::CarryHarvestableVespeneGeyserGasProtoss);
        test_element(Buff::CarryHarvestableVespeneGeyserGasZerg);
    }
}
