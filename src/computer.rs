use futures::prelude::*;
use futures::unsync::mpsc;
use tokio_core::reactor;

use super::{Error, Result};
use constants::sc2_bug_tag;
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
                .map_err(|e| {
                    panic!(
                        "{}: ComputerService ended unexpectedly - {:#?}",
                        sc2_bug_tag(),
                        e
                    )
                }),
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
                    tx.send(self.setup).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to rsp player setup",
                            sc2_bug_tag()
                        )
                    })?;
                },
                MeleeRequest::Connect(_, tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!("{}: Unable to ack connect", sc2_bug_tag())
                    })?;
                },

                MeleeRequest::CreateGame(_, _, tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to ack create game",
                            sc2_bug_tag()
                        )
                    })?;
                },
                MeleeRequest::JoinGame(_, _, tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to ack join game",
                            sc2_bug_tag()
                        )
                    })?;
                },
                MeleeRequest::RunGame(_, tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to ack run game",
                            sc2_bug_tag()
                        )
                    })?;
                },
                MeleeRequest::LeaveGame(tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to ack leave game",
                            sc2_bug_tag()
                        )
                    })?;
                },

                MeleeRequest::Disconnect(tx) => {
                    tx.send(()).map_err(|_| -> Error {
                        unreachable!(
                            "{}: Unable to ack disconnect",
                            sc2_bug_tag()
                        )
                    })?;
                },
            }
        }

        Ok(())
    }
}
