//! Contains public API for creating built-in SC2 AI opponents.

use futures::prelude::*;
use futures::unsync::mpsc;
use tokio_core::reactor;

use constants::sc2_bug_tag;
use data::{Difficulty, PlayerSetup, Race};
use services::computer_service::ComputerService;
use services::melee_service::{MeleeCompetitor, MeleeRequest};

use Result;

/// Build a built-in AI opponent.
pub struct OpponentBuilder {
    race: Race,
    difficulty: Difficulty,
}

impl OpponentBuilder {
    /// Create the builder.
    pub fn new() -> Self {
        Self {
            race: Race::Random,
            difficulty: Difficulty::Medium,
        }
    }

    /// Set the race of the AI (default is Random).
    pub fn race(self, race: Race) -> Self {
        Self {
            race: race,
            ..self
        }
    }

    /// Set the difficulty of the AI (default is Medium).
    pub fn difficulty(self, difficulty: Difficulty) -> Self {
        Self {
            difficulty: difficulty,
            ..self
        }
    }
}

impl MeleeCompetitor for OpponentBuilder {
    fn spawn(
        &mut self,
        handle: &reactor::Handle,
        control_rx: mpsc::Receiver<MeleeRequest>,
    ) -> Result<()> {
        handle.spawn(
            ComputerService::new(PlayerSetup::Computer(
                self.race,
                self.difficulty,
            )).run(control_rx)
                .map_err(|e| {
                    panic!(
                        "{}: OpponentService ended unexpectedly - {:#?}",
                        sc2_bug_tag(),
                        e
                    )
                }),
        );

        Ok(())
    }
}
