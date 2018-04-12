use futures::prelude::*;
use futures::unsync::mpsc;

use constants::sc2_bug_tag;
use data::PlayerSetup;
use services::melee_service::MeleeRequest;
use {Error, Result};

pub struct ComputerService {
    setup: PlayerSetup,
}

impl ComputerService {
    pub fn new(setup: PlayerSetup) -> Self {
        Self { setup: setup }
    }
    #[async]
    pub fn run(self, control_rx: mpsc::Receiver<MeleeRequest>) -> Result<()> {
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
