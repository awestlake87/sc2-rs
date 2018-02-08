use futures::prelude::*;
use organelle::{Axon, Constraint, Impulse, Soma};
use url::Url;

use super::{Error, Result};
use data::{
    Difficulty,
    GamePorts,
    GameSettings,
    PlayerSetup,
    Race,
    UpdateScheme,
};
use melee::{MeleeContract, MeleeDendrite};
use synapses::{Dendrite, Synapse, Terminal};

/// a built-in AI opponent soma
pub struct ComputerSoma {
    setup: PlayerSetup,
    melee: Option<MeleeDendrite>,
}

impl ComputerSoma {
    /// create a built-in AI to fight
    pub fn axon(race: Race, difficulty: Difficulty) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                setup: PlayerSetup::Computer {
                    race: race,
                    difficulty: difficulty,
                },
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
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
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
    fn create_game(self, _: GameSettings, _: Vec<PlayerSetup>) -> Result<Self> {
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
