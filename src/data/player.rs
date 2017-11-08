
use sc2_proto::sc2api;
use sc2_proto::common;

/// race of the player
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
    Random,
}

impl Race {
    /// convert from protobuf data
    pub fn from_proto(race: common::Race) -> Option<Self> {
        match race {
            common::Race::Terran => Some(Race::Terran),
            common::Race::Zerg => Some(Race::Zerg),
            common::Race::Protoss => Some(Race::Protoss),
            common::Race::Random => Some(Race::Random),
            common::Race::NoRace => None,
        }
    }
    /// convert to protobuf data
    pub fn to_proto(&self) -> common::Race {
        match *self {
            Race::Zerg      => common::Race::Zerg,
            Race::Terran    => common::Race::Terran,
            Race::Protoss   => common::Race::Protoss,
            Race::Random    => common::Race::Random,
        }
    }
}

/// difficulty setting for built-in StarCraft II AI
#[allow(missing_docs)]
#[derive(Copy, Clone)]
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
    CheatInsane
}

impl Difficulty {
    /// convert to protobuf data
    pub fn to_proto(&self) -> sc2api::Difficulty {
        match *self {
            Difficulty::VeryEasy        => sc2api::Difficulty::VeryEasy,
            Difficulty::Easy            => sc2api::Difficulty::Easy,
            Difficulty::Medium          => sc2api::Difficulty::Medium,
            Difficulty::MediumHard      => sc2api::Difficulty::MediumHard,
            Difficulty::Hard            => sc2api::Difficulty::Hard,
            Difficulty::Harder          => sc2api::Difficulty::Harder,
            Difficulty::VeryHard        => sc2api::Difficulty::VeryHard,
            Difficulty::CheatVision     => sc2api::Difficulty::CheatVision,
            Difficulty::CheatMoney      => sc2api::Difficulty::CheatMoney,
            Difficulty::CheatInsane     => sc2api::Difficulty::CheatInsane
        }
    }
}

/// settings for players
#[derive(Copy, Clone)]
pub enum PlayerSetup {
    /// add a built-in StarCraft II bot with the given race and difficulty
    Computer {
        /// race of the computer
        race:           Race,
        /// difficulty setting
        difficulty:     Difficulty
    },
    /// add a user-controlled player
    Player {
        /// race of the player
        race:           Race
    },
    /// add a replay observer (these are separate from the other two)
    Observer
}
