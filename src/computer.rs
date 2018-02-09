use futures::prelude::*;
use organelle::{Axon, Constraint, Impulse, Soma};
use url::Url;

use super::{Error, Result};
use data::{Difficulty, GameSetup, PlayerSetup, Race};
use launcher::GamePorts;
use melee::{MeleeCompetitor, MeleeContract, MeleeDendrite, UpdateScheme};
use synapses::{Dendrite, Synapse};

/// build a built-in AI opponent
pub struct ComputerBuilder {
    race: Race,
    difficulty: Difficulty,
}

impl ComputerBuilder {
    /// create the builder
    pub fn new() -> Self {
        Self {
            race: Race::Random,
            difficulty: Difficulty::Medium,
        }
    }

    /// set the race of the AI (default is Random)
    pub fn race(self, race: Race) -> Self {
        Self { race: race, ..self }
    }

    /// set the difficulty of the AI (default is Medium)
    pub fn difficulty(self, difficulty: Difficulty) -> Self {
        Self {
            difficulty: difficulty,
            ..self
        }
    }

    /// build the built-in AI
    pub fn create(self) -> Result<Computer> {
        Ok(Computer {
            0: ComputerSoma::axon(self.race, self.difficulty)?,
        })
    }
}

impl MeleeCompetitor for Computer {
    type Soma = Axon<ComputerSoma>;

    fn into_soma(self) -> Self::Soma {
        self.0
    }
}

/// a built-in AI opponent soma
pub struct Computer(Axon<ComputerSoma>);

pub struct ComputerSoma {
    setup: PlayerSetup,
    melee: Option<MeleeDendrite>,
}

impl ComputerSoma {
    fn axon(race: Race, difficulty: Difficulty) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                setup: PlayerSetup::Computer(race, difficulty),
                melee: None,
            },
            vec![Constraint::One(Synapse::Melee)],
            vec![],
        ))
    }
}

impl Soma for ComputerSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(_, Synapse::Melee, Dendrite::Melee(melee)) => {
                Ok(Self {
                    melee: Some(melee),

                    ..self
                })
            },

            Impulse::Start(_, main_tx, handle) => {
                handle.spawn(
                    self.melee
                        .unwrap()
                        .wrap(ComputerDendrite::new(self.setup))
                        .or_else(move |e| {
                            main_tx
                                .send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(Self {
                    melee: None,
                    ..self
                })
            },
            _ => bail!("unexpected impulse"),
        }
    }
}

struct ComputerDendrite {
    setup: PlayerSetup,
}

impl MeleeContract for ComputerDendrite {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSetup) -> Result<(Self, PlayerSetup)> {
        let setup = self.setup;
        Ok((self, setup))
    }
    /// connect to an instance
    #[async(boxed)]
    fn connect(self, _: Url) -> Result<Self> {
        Ok(self)
    }

    /// create a game
    #[async(boxed)]
    fn create_game(self, _: GameSetup, _: Vec<PlayerSetup>) -> Result<Self> {
        Ok(self)
    }

    /// join a game
    #[async(boxed)]
    fn join_game(self, _: PlayerSetup, _: Option<GamePorts>) -> Result<Self> {
        Ok(self)
    }

    /// run the game
    #[async(boxed)]
    fn run_game(self, _: UpdateScheme) -> Result<Self> {
        Ok(self)
    }
}

impl ComputerDendrite {
    fn new(setup: PlayerSetup) -> Self {
        Self { setup: setup }
    }
}
