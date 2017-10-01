
use super::{ Result, Error };
use instance::Instance;

pub enum PlayerType {
    Computer,
    Participant
}

pub enum Race {
    Terran,
    Zerg,
    Protoss
}

pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    MediumHard,
    Hard,
    HardVeryHard,
    VeryHard,
    CheatVision,
    CheatMoney,
    CheatInsane
}

pub struct Player {
    pub instance:       Instance,

    pub player_type:    PlayerType,
    pub race:           Race,
    pub difficulty:     Option<Difficulty>
}

impl Player {
    pub fn new_computer(
        instance: Instance,
        race: Race,
        difficulty: Difficulty
    )
        -> Self
    {
        Self {
            instance: instance,
            player_type: PlayerType::Computer,
            race: race,
            difficulty: Some(difficulty)
        }
    }

    pub fn new_participant(instance: Instance, race: Race) -> Self {
        Self {
            instance: instance,
            player_type: PlayerType::Participant,
            race: race,
            difficulty: None
        }
    }
}
