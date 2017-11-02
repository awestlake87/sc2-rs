
use sc2_proto::sc2api;
use sc2_proto::common;

#[derive(Copy, Clone)]
pub enum PlayerKind {
    Computer,
    Participant,
    Observer
}

#[derive(Copy, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
    Random,
}

impl Race {
    pub fn from_proto(race: common::Race) -> Option<Self> {
        match race {
            common::Race::Terran => Some(Race::Terran),
            common::Race::Zerg => Some(Race::Zerg),
            common::Race::Protoss => Some(Race::Protoss),
            common::Race::Random => Some(Race::Random),
            common::Race::NoRace => None,
        }
    }
    pub fn to_proto(&self) -> common::Race {
        match *self {
            Race::Zerg      => common::Race::Zerg,
            Race::Terran    => common::Race::Terran,
            Race::Protoss   => common::Race::Protoss,
            Race::Random    => common::Race::Random,
        }
    }
}

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

#[derive(Copy, Clone)]
pub enum PlayerSetup {
    Computer {
        race:           Race,
        difficulty:     Difficulty
    },
    Player {
        race:           Race
    },
    Observer
}
