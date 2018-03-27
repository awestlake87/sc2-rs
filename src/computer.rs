use futures::prelude::*;
use tokio_core::reactor;
use url::Url;

use super::{Error, Result};
use data::{Difficulty, GameSetup, PlayerSetup, Race};
use launcher::GamePorts;
use melee::{MeleeCompetitor, MeleeContract, MeleeDendrite, UpdateScheme};

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
        Self {
            race: race,
            ..self
        }
    }

    /// set the difficulty of the AI (default is Medium)
    pub fn difficulty(self, difficulty: Difficulty) -> Self {
        Self {
            difficulty: difficulty,
            ..self
        }
    }
}

impl MeleeCompetitor for ComputerBuilder {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        controller: MeleeDendrite,
    ) -> Result<()> {
        handle.spawn(
            controller
                .wrap(ComputerDendrite::new(PlayerSetup::Computer(
                    self.race,
                    self.difficulty,
                )))
                .map_err(|e| panic!("{:#?}", e)),
        );

        Ok(())
    }
}

pub struct ComputerSoma {
    setup: PlayerSetup,
    melee: Option<MeleeDendrite>,
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
