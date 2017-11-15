
use super::super::{ Result, ErrorKind, FromProto, IntoProto };

/// a list of known StarCraft II upgrades
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Upgrade {
    CarrierLaunchSpeedUpgrade           = 1,
    GlialReconstitution                 = 2,
    TunnelingClaws                      = 3,
    ChitinousPlating                    = 4,
    HiSecAutoTracking                   = 5,
    TerranBuildingArmor                 = 6,
    TerranInfantryWeaponsLevel1         = 7,
    TerranInfantryWeaponsLevel2         = 8,
    TerranInfantryWeaponsLevel3         = 9,
    NeoSteelFrame                       = 10,
    TerranInfantryArmorsLevel1          = 11,
    TerranInfantryArmorsLevel2          = 12,
    TerranInfantryArmorsLevel3          = 13,
    Stimpack                            = 15,
    ShieldWall                          = 16,
    PunisherGrenades                    = 17,
    HighCapacityBarrels                 = 19,
    BansheeCloak                        = 20,
    RavenCorvidReactor                  = 22,
    PersonalCloaking                    = 25,
    TerranVehicleWeaponsLevel1          = 30,
    TerranVehicleWeaponsLevel2          = 31,
    TerranVehicleWeaponsLevel3          = 32,
    TerranShipWeaponsLevel1             = 36,
    TerranShipWeaponsLevel2             = 37,
    TerranShipWeaponsLevel3             = 38,
    ProtossGroundWeaponsLevel1          = 39,
    ProtossGroundWeaponsLevel2          = 40,
    ProtossGroundWeaponsLevel3          = 41,
    ProtossGroundArmorsLevel1           = 42,
    ProtossGroundArmorsLevel2           = 43,
    ProtossGroundArmorsLevel3           = 44,
    ProtossShieldsLevel1                = 45,
    ProtossShieldsLevel2                = 46,
    ProtossShieldsLevel3                = 47,
    ObserverGraviticBooster             = 48,
    GraviticDrive                       = 49,
    ExtendedThermalLance                = 50,
    PsiStormTech                        = 52,
    ZergMeleeWeaponsLevel1              = 53,
    ZergMeleeWeaponsLevel2              = 54,
    ZergMeleeWeaponsLevel3              = 55,
    ZergGroundArmorsLevel1              = 56,
    ZergGroundArmorsLevel2              = 57,
    ZergGroundArmorsLevel3              = 58,
    ZergMissileWeaponsLevel1            = 59,
    ZergMissileWeaponsLevel2            = 60,
    ZergMissileWeaponsLevel3            = 61,
    OverlordSpeed                       = 62,
    Burrow                              = 64,
    ZerglingAttackSpeed                 = 65,
    ZerglingMovementSpeed               = 66,
    ZergFlyerWeaponsLevel1              = 68,
    ZergFlyerWeaponsLevel2              = 69,
    ZergFlyerWeaponsLevel3              = 70,
    ZergFlyerArmorsLevel1               = 71,
    ZergFlyerArmorsLevel2               = 72,
    ZergFlyerArmorsLevel3               = 73,
    InfestorEnergyUpgrade               = 74,
    CentrificalHooks                    = 75,
    BattleCruiserEnableSpecializations  = 76,
    ProtossAirWeaponsLevel1             = 78,
    ProtossAirWeaponsLevel2             = 79,
    ProtossAirWeaponsLevel3             = 80,
    ProtossAirArmorsLevel1              = 81,
    ProtossAirArmorsLevel2              = 82,
    ProtossAirArmorsLevel3              = 83,
    WarpGateResearch                    = 84,
    Charge                              = 86,
    BlinkTech                           = 87,
    PhoenixRangeUpgrade                 = 99,
    NeuralParasite                      = 101,
    TerranVehicleAndShipArmorsLevel1    = 116,
    TerranVehicleAndShipArmorsLevel2    = 117,
    TerranVehicleAndShipArmorsLevel3    = 118,
    DrillClaws                          = 122,
    AdeptPiercingAttack                 = 130,
    MagFieldLaunchers                   = 133,
    EvolveGroovedSpines                 = 134,
    EvolveMuscularAugments              = 135,
    BansheeSpeed                        = 136,
    RavenRecalibratedExplosives         = 138,
    MedivacIncreaseSpeedBoost           = 139,
    LiberatorAgRangeUpgrade             = 140,
    DarkTemplarBlinkUpgrade             = 141,
}

impl FromProto<u32> for Upgrade {
    fn from_proto(id: u32) -> Result<Self> {
        Ok(
            match id {
                1   => Upgrade::CarrierLaunchSpeedUpgrade,
                2   => Upgrade::GlialReconstitution,
                3   => Upgrade::TunnelingClaws,
                4   => Upgrade::ChitinousPlating,
                5   => Upgrade::HiSecAutoTracking,
                6   => Upgrade::TerranBuildingArmor,
                7   => Upgrade::TerranInfantryWeaponsLevel1,
                8   => Upgrade::TerranInfantryWeaponsLevel2,
                9   => Upgrade::TerranInfantryWeaponsLevel3,
                10  => Upgrade::NeoSteelFrame,
                11  => Upgrade::TerranInfantryArmorsLevel1,
                12  => Upgrade::TerranInfantryArmorsLevel2,
                13  => Upgrade::TerranInfantryArmorsLevel3,
                15  => Upgrade::Stimpack,
                16  => Upgrade::ShieldWall,
                17  => Upgrade::PunisherGrenades,
                19  => Upgrade::HighCapacityBarrels,
                20  => Upgrade::BansheeCloak,
                22  => Upgrade::RavenCorvidReactor,
                25  => Upgrade::PersonalCloaking,
                30  => Upgrade::TerranVehicleWeaponsLevel1,
                31  => Upgrade::TerranVehicleWeaponsLevel2,
                32  => Upgrade::TerranVehicleWeaponsLevel3,
                36  => Upgrade::TerranShipWeaponsLevel1,
                37  => Upgrade::TerranShipWeaponsLevel2,
                38  => Upgrade::TerranShipWeaponsLevel3,
                39  => Upgrade::ProtossGroundWeaponsLevel1,
                40  => Upgrade::ProtossGroundWeaponsLevel2,
                41  => Upgrade::ProtossGroundWeaponsLevel3,
                42  => Upgrade::ProtossGroundArmorsLevel1,
                43  => Upgrade::ProtossGroundArmorsLevel2,
                44  => Upgrade::ProtossGroundArmorsLevel3,
                45  => Upgrade::ProtossShieldsLevel1,
                46  => Upgrade::ProtossShieldsLevel2,
                47  => Upgrade::ProtossShieldsLevel3,
                48  => Upgrade::ObserverGraviticBooster,
                49  => Upgrade::GraviticDrive,
                50  => Upgrade::ExtendedThermalLance,
                52  => Upgrade::PsiStormTech,
                53  => Upgrade::ZergMeleeWeaponsLevel1,
                54  => Upgrade::ZergMeleeWeaponsLevel2,
                55  => Upgrade::ZergMeleeWeaponsLevel3,
                56  => Upgrade::ZergGroundArmorsLevel1,
                57  => Upgrade::ZergGroundArmorsLevel2,
                58  => Upgrade::ZergGroundArmorsLevel3,
                59  => Upgrade::ZergMissileWeaponsLevel1,
                60  => Upgrade::ZergMissileWeaponsLevel2,
                61  => Upgrade::ZergMissileWeaponsLevel3,
                62  => Upgrade::OverlordSpeed,
                64  => Upgrade::Burrow,
                65  => Upgrade::ZerglingAttackSpeed,
                66  => Upgrade::ZerglingMovementSpeed,
                68  => Upgrade::ZergFlyerWeaponsLevel1,
                69  => Upgrade::ZergFlyerWeaponsLevel2,
                70  => Upgrade::ZergFlyerWeaponsLevel3,
                71  => Upgrade::ZergFlyerArmorsLevel1,
                72  => Upgrade::ZergFlyerArmorsLevel2,
                73  => Upgrade::ZergFlyerArmorsLevel3,
                74  => Upgrade::InfestorEnergyUpgrade,
                75  => Upgrade::CentrificalHooks,
                76  => Upgrade::BattleCruiserEnableSpecializations,
                78  => Upgrade::ProtossAirWeaponsLevel1,
                79  => Upgrade::ProtossAirWeaponsLevel2,
                80  => Upgrade::ProtossAirWeaponsLevel3,
                81  => Upgrade::ProtossAirArmorsLevel1,
                82  => Upgrade::ProtossAirArmorsLevel2,
                83  => Upgrade::ProtossAirArmorsLevel3,
                84  => Upgrade::WarpGateResearch,
                86  => Upgrade::Charge,
                87  => Upgrade::BlinkTech,
                99  => Upgrade::PhoenixRangeUpgrade,
                101 => Upgrade::NeuralParasite,
                116 => Upgrade::TerranVehicleAndShipArmorsLevel1,
                117 => Upgrade::TerranVehicleAndShipArmorsLevel2,
                118 => Upgrade::TerranVehicleAndShipArmorsLevel3,
                122 => Upgrade::DrillClaws,
                130 => Upgrade::AdeptPiercingAttack,
                133 => Upgrade::MagFieldLaunchers,
                134 => Upgrade::EvolveGroovedSpines,
                135 => Upgrade::EvolveMuscularAugments,
                136 => Upgrade::BansheeSpeed,
                138 => Upgrade::RavenRecalibratedExplosives,
                139 => Upgrade::MedivacIncreaseSpeedBoost,
                140 => Upgrade::LiberatorAgRangeUpgrade,
                141 => Upgrade::DarkTemplarBlinkUpgrade,

                _ => bail!(
                    ErrorKind::InvalidProtobuf(format!("Upgrade id({})", id))
                )
            }
        )
    }
}

impl IntoProto<u32> for Upgrade {
    fn into_proto(self) -> Result<u32> {
        Ok(self as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutativity() {
        let test_element = |element: Upgrade| assert_eq!(
            element,
            Upgrade::from_proto(element.into_proto().unwrap()).unwrap()
        );

        test_element(Upgrade::CarrierLaunchSpeedUpgrade);
        test_element(Upgrade::GlialReconstitution);
        test_element(Upgrade::TunnelingClaws);
        test_element(Upgrade::ChitinousPlating);
        test_element(Upgrade::HiSecAutoTracking);
        test_element(Upgrade::TerranBuildingArmor);
        test_element(Upgrade::TerranInfantryWeaponsLevel1);
        test_element(Upgrade::TerranInfantryWeaponsLevel2);
        test_element(Upgrade::TerranInfantryWeaponsLevel3);
        test_element(Upgrade::NeoSteelFrame);
        test_element(Upgrade::TerranInfantryArmorsLevel1);
        test_element(Upgrade::TerranInfantryArmorsLevel2);
        test_element(Upgrade::TerranInfantryArmorsLevel3);
        test_element(Upgrade::Stimpack);
        test_element(Upgrade::ShieldWall);
        test_element(Upgrade::PunisherGrenades);
        test_element(Upgrade::HighCapacityBarrels);
        test_element(Upgrade::BansheeCloak);
        test_element(Upgrade::RavenCorvidReactor);
        test_element(Upgrade::PersonalCloaking);
        test_element(Upgrade::TerranVehicleWeaponsLevel1);
        test_element(Upgrade::TerranVehicleWeaponsLevel2);
        test_element(Upgrade::TerranVehicleWeaponsLevel3);
        test_element(Upgrade::TerranShipWeaponsLevel1);
        test_element(Upgrade::TerranShipWeaponsLevel2);
        test_element(Upgrade::TerranShipWeaponsLevel3);
        test_element(Upgrade::ProtossGroundWeaponsLevel1);
        test_element(Upgrade::ProtossGroundWeaponsLevel2);
        test_element(Upgrade::ProtossGroundWeaponsLevel3);
        test_element(Upgrade::ProtossGroundArmorsLevel1);
        test_element(Upgrade::ProtossGroundArmorsLevel2);
        test_element(Upgrade::ProtossGroundArmorsLevel3);
        test_element(Upgrade::ProtossShieldsLevel1);
        test_element(Upgrade::ProtossShieldsLevel2);
        test_element(Upgrade::ProtossShieldsLevel3);
        test_element(Upgrade::ObserverGraviticBooster);
        test_element(Upgrade::GraviticDrive);
        test_element(Upgrade::ExtendedThermalLance);
        test_element(Upgrade::PsiStormTech);
        test_element(Upgrade::ZergMeleeWeaponsLevel1);
        test_element(Upgrade::ZergMeleeWeaponsLevel2);
        test_element(Upgrade::ZergMeleeWeaponsLevel3);
        test_element(Upgrade::ZergGroundArmorsLevel1);
        test_element(Upgrade::ZergGroundArmorsLevel2);
        test_element(Upgrade::ZergGroundArmorsLevel3);
        test_element(Upgrade::ZergMissileWeaponsLevel1);
        test_element(Upgrade::ZergMissileWeaponsLevel2);
        test_element(Upgrade::ZergMissileWeaponsLevel3);
        test_element(Upgrade::OverlordSpeed);
        test_element(Upgrade::Burrow);
        test_element(Upgrade::ZerglingAttackSpeed);
        test_element(Upgrade::ZerglingMovementSpeed);
        test_element(Upgrade::ZergFlyerWeaponsLevel1);
        test_element(Upgrade::ZergFlyerWeaponsLevel2);
        test_element(Upgrade::ZergFlyerWeaponsLevel3);
        test_element(Upgrade::ZergFlyerArmorsLevel1);
        test_element(Upgrade::ZergFlyerArmorsLevel2);
        test_element(Upgrade::ZergFlyerArmorsLevel3);
        test_element(Upgrade::InfestorEnergyUpgrade);
        test_element(Upgrade::CentrificalHooks);
        test_element(Upgrade::BattleCruiserEnableSpecializations);
        test_element(Upgrade::ProtossAirWeaponsLevel1);
        test_element(Upgrade::ProtossAirWeaponsLevel2);
        test_element(Upgrade::ProtossAirWeaponsLevel3);
        test_element(Upgrade::ProtossAirArmorsLevel1);
        test_element(Upgrade::ProtossAirArmorsLevel2);
        test_element(Upgrade::ProtossAirArmorsLevel3);
        test_element(Upgrade::WarpGateResearch);
        test_element(Upgrade::Charge);
        test_element(Upgrade::BlinkTech);
        test_element(Upgrade::PhoenixRangeUpgrade);
        test_element(Upgrade::NeuralParasite);
        test_element(Upgrade::TerranVehicleAndShipArmorsLevel1);
        test_element(Upgrade::TerranVehicleAndShipArmorsLevel2);
        test_element(Upgrade::TerranVehicleAndShipArmorsLevel3);
        test_element(Upgrade::DrillClaws);
        test_element(Upgrade::AdeptPiercingAttack);
        test_element(Upgrade::MagFieldLaunchers);
        test_element(Upgrade::EvolveGroovedSpines);
        test_element(Upgrade::EvolveMuscularAugments);
        test_element(Upgrade::BansheeSpeed);
        test_element(Upgrade::RavenRecalibratedExplosives);
        test_element(Upgrade::MedivacIncreaseSpeedBoost);
        test_element(Upgrade::LiberatorAgRangeUpgrade);
        test_element(Upgrade::DarkTemplarBlinkUpgrade);
    }
}
