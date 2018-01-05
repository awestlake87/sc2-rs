
use cortical;
use cortical::{ ResultExt, Lobe, Protocol, Constraint };

use super::{ Result, Soma, Message, Role, Race, Difficulty, PlayerSetup };

pub enum ComputerLobe {
    Init(Init),
    Setup(Setup),
}

impl ComputerLobe {
    pub fn new(race: Race, difficulty: Difficulty) -> Result<Self> {
        Ok(
            ComputerLobe::Init(
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

impl Lobe for ComputerLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> cortical::Result<Self> {
        match self {
            ComputerLobe::Init(state) => state.update(msg),
            ComputerLobe::Setup(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct Init {
    soma:           Soma,

    setup:          PlayerSetup,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ComputerLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma, self.setup),

                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerLobe::Init(self))
        }
    }
}

pub struct Setup {
    soma:           Soma,

    setup:          PlayerSetup,
}

impl Setup {
    fn setup(soma: Soma, setup: PlayerSetup) -> Result<ComputerLobe> {
        Ok(ComputerLobe::Setup(Setup { soma: soma, setup: setup }))
    }

    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ComputerLobe> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::RequestPlayerSetup(_)) => {
                    self.soma.send_req_input(
                        Role::Controller, Message::PlayerSetup(self.setup)
                    )?;

                    Ok(ComputerLobe::Setup(self))
                },

                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerLobe::Setup(self))
        }
    }
}
