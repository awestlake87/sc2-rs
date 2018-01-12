
use organelle;
use organelle::{ ResultExt, Cell, Protocol, Constraint };

use super::{ Result, Soma, Message, Role, Race, Difficulty, PlayerSetup };

/// cell that acts as the built-in SC2 AI
pub enum ComputerCell {
    /// initialize the soma
    Init(Init),
    /// respond to the setup queries
    Setup(Setup),
}

impl ComputerCell {
    /// create a new computer cell
    pub fn new(race: Race, difficulty: Difficulty) -> Result<Self> {
        Ok(
            ComputerCell::Init(
                Init {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::Controller),
                            Constraint::RequireOne(Role::InstanceProvider),
                        ],
                        vec![ ],
                    )?,

                    setup: PlayerSetup::Computer {
                        race: race,
                        difficulty: difficulty,
                    },
                }
            )
        )
    }
}

impl Cell for ComputerCell {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> organelle::Result<Self> {
        match self {
            ComputerCell::Init(state) => state.update(msg),
            ComputerCell::Setup(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::CellError
        )
    }
}

pub struct Init {
    soma:           Soma,

    setup:          PlayerSetup,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ComputerCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma, self.setup),


                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerCell::Init(self))
        }
    }
}

pub struct Setup {
    soma:           Soma,

    setup:          PlayerSetup,
}

impl Setup {
    fn setup(soma: Soma, setup: PlayerSetup) -> Result<ComputerCell> {
        Ok(ComputerCell::Setup(Setup { soma: soma, setup: setup }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ComputerCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::RequestPlayerSetup(_)) => {
                    self.soma.send_req_input(
                        Role::Controller, Message::PlayerSetup(self.setup)
                    )?;

                    Ok(ComputerCell::Setup(self))
                },

                Protocol::Message(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerCell::Setup(self))
        }
    }
}
