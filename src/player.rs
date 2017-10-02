
#[derive(Copy, Clone)]
pub enum PlayerKind {
    Computer,
    Participant
}

#[derive(Copy, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss
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

pub struct Player {
    pub kind:           PlayerKind,
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
            kind: PlayerKind::Computer,
            race: race,
            difficulty: Some(difficulty)
        }
    }

    pub fn new_participant(race: Race) -> Self {
        Self {
            kind: PlayerKind::Participant,
            race: race,
            difficulty: None
        }
    }
}
