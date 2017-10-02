
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
    pub player_type:    PlayerType,
    pub race:           Race,
    pub difficulty:     Option<Difficulty>
}

impl Player {
    pub fn new_computer(
        race: Race,
        difficulty: Difficulty
    )
        -> Self
    {
        Self {
            player_type: PlayerType::Computer,
            race: race,
            difficulty: Some(difficulty)
        }
    }

    pub fn new_participant(race: Race) -> Self {
        Self {
            player_type: PlayerType::Participant,
            race: race,
            difficulty: None
        }
    }
}
