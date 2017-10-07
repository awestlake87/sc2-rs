
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

#[derive(Copy, Clone)]
pub struct Player {
    pub kind:           PlayerKind,
    pub race:           Option<Race>,
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
            race: Some(race),
            difficulty: Some(difficulty)
        }
    }

    pub fn new_participant(race: Race) -> Self {
        Self {
            kind: PlayerKind::Participant,
            race: Some(race),
            difficulty: None
        }
    }

    pub fn new_observer() -> Self {
        Self {
            kind: PlayerKind::Observer,
            race: None,
            difficulty: None
        }
    }
}
