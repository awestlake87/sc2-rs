use futures::prelude::*;
use futures::unsync::mpsc;
use tokio_core::reactor;

use super::{Error, Result};
use data::{Difficulty, PlayerSetup, Race};
use melee::{MeleeCompetitor, MeleeRequest};

/// Build a built-in AI opponent.
pub struct ComputerBuilder {
    race: Race,
    difficulty: Difficulty,
}

impl ComputerBuilder {
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

impl MeleeCompetitor for ComputerBuilder {
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
                .map_err(|e| panic!("{:#?}", e)),
        );

        Ok(())
    }
}

struct ComputerService {
    setup: PlayerSetup,
}

impl ComputerService {
    fn new(setup: PlayerSetup) -> Self {
        Self { setup: setup }
    }
    #[async]
    fn run(self, control_rx: mpsc::Receiver<MeleeRequest>) -> Result<()> {
        #[async]
        for req in control_rx.map_err(|_| -> Error { unreachable!() }) {
            match req {
                MeleeRequest::PlayerSetup(_, tx) => {
                    tx.send(self.setup).map_err(|_| {
                        Error::from("unable to get player setup")
                    })?;
                },
                MeleeRequest::Connect(_, tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to connect"))?;
                },

                MeleeRequest::CreateGame(_, _, tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to create game"))?;
                },
                MeleeRequest::JoinGame(_, _, tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to join game"))?;
                },
                MeleeRequest::RunGame(_, tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to run game"))?;
                },
                MeleeRequest::LeaveGame(tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to leave game"))?;
                },

                MeleeRequest::Disconnect(tx) => {
                    tx.send(())
                        .map_err(|_| Error::from("unable to disconnect"))?;
                },
            }
        }

        Ok(())
    }
}
