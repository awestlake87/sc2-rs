use sc2_proto::common;
use sc2_proto::sc2api;

use super::super::{FromProto, IntoProto, Result};

/// Race of the player.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
    Random,
}

impl FromProto<common::Race> for Race {
    fn from_proto(race: common::Race) -> Result<Self> {
        Ok(match race {
            common::Race::Terran => Race::Terran,
            common::Race::Zerg => Race::Zerg,
            common::Race::Protoss => Race::Protoss,
            common::Race::Random => Race::Random,
            common::Race::NoRace => panic!(concat!(
                "NoRace value (Library Bug! please let us know that ",
                "this can in fact happen!)"
            )),
        })
    }
}

impl IntoProto<common::Race> for Race {
    fn into_proto(self) -> Result<common::Race> {
        Ok(match self {
            Race::Zerg => common::Race::Zerg,
            Race::Terran => common::Race::Terran,
            Race::Protoss => common::Race::Protoss,
            Race::Random => common::Race::Random,
        })
    }
}

/// Difficulty setting for built-in StarCraft II AI.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    MediumHard,
    Hard,
    Harder,
    VeryHard,
    CheatVision,
    CheatMoney,
    CheatInsane,
}

impl Difficulty {
    /// convert to protobuf data
    pub fn to_proto(&self) -> sc2api::Difficulty {
        match *self {
            Difficulty::VeryEasy => sc2api::Difficulty::VeryEasy,
            Difficulty::Easy => sc2api::Difficulty::Easy,
            Difficulty::Medium => sc2api::Difficulty::Medium,
            Difficulty::MediumHard => sc2api::Difficulty::MediumHard,
            Difficulty::Hard => sc2api::Difficulty::Hard,
            Difficulty::Harder => sc2api::Difficulty::Harder,
            Difficulty::VeryHard => sc2api::Difficulty::VeryHard,
            Difficulty::CheatVision => sc2api::Difficulty::CheatVision,
            Difficulty::CheatMoney => sc2api::Difficulty::CheatMoney,
            Difficulty::CheatInsane => sc2api::Difficulty::CheatInsane,
        }
    }
}

/// Settings for players.
#[derive(Debug, Copy, Clone)]
pub enum PlayerSetup {
    /// Add a built-in StarCraft II bot with the given race and difficulty.
    Computer(Race, Difficulty),
    /// Add a user-controlled player.
    Player(Race),
    //Observer,
}

impl PlayerSetup {
    /// Does the PlayerSetup represent a player?
    pub fn is_player(&self) -> bool {
        match self {
            &PlayerSetup::Player(_) => true,
            _ => false,
        }
    }

    /// Does the PlayerSetup represent a computer?
    pub fn is_computer(&self) -> bool {
        match self {
            &PlayerSetup::Computer(_, _) => true,
            _ => false,
        }
    }

    /*/// does the player setup represent an observer
    pub fn is_observer(&self) -> bool {
        match self {
            &PlayerSetup::Observer => true,
            _ => false,
        }
    }*/
}
