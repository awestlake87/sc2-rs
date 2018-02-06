use futures::prelude::*;
use organelle::{Axon, Constraint, Impulse, Soma};

use super::{Error, Result};
use agent::{AgentContract, AgentDendrite};
use data::{Difficulty, GameSettings, PlayerSetup, Race};
use synapses::{PlayerDendrite, PlayerSynapse, PlayerTerminal};

/// a built-in AI opponent soma
pub struct ComputerSoma {
    setup: PlayerSetup,
    agent: Option<AgentDendrite>,
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
                agent: None,
            },
            vec![Constraint::One(PlayerSynapse::Agent)],
            vec![
                Constraint::One(PlayerSynapse::Observer),
                Constraint::One(PlayerSynapse::Action),
            ],
        ))
    }
}

impl Soma for ComputerSoma {
    type Synapse = PlayerSynapse;
    type Error = Error;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(
                _,
                PlayerSynapse::Agent,
                PlayerDendrite::Agent(agent),
            ) => Ok(Self {
                agent: Some(agent),

                ..self
            }),
            Impulse::AddTerminal(
                _,
                PlayerSynapse::Observer,
                PlayerTerminal::Observer(_),
            )
            | Impulse::AddTerminal(
                _,
                PlayerSynapse::Action,
                PlayerTerminal::Action(_),
            ) => Ok(self),

            Impulse::Start(_, main_tx, handle) => {
                handle.spawn(
                    self.agent
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
                    agent: None,
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

impl AgentContract for ComputerDendrite {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
        let setup = self.setup;
        Ok((self, setup))
    }
}

impl ComputerDendrite {
    fn new(setup: PlayerSetup) -> Self {
        Self { setup: setup }
    }
}
