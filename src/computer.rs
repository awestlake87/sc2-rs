
use organelle;
use organelle::{ ResultExt, Soma, Impulse, Dendrite };

use super::{ Result, Axon, Signal, Synapse, Race, Difficulty, PlayerSetup };

/// soma that acts as the built-in SC2 AI
pub enum ComputerSoma {
    /// initialize the axon
    Init(Init),
    /// respond to the setup queries
    Setup(Setup),
}

impl ComputerSoma {
    /// create a new computer soma
    pub fn new(race: Race, difficulty: Difficulty) -> Result<Self> {
        Ok(
            ComputerSoma::Init(
                Init {
                    axon: Axon::new(
                        vec![
                            Dendrite::RequireOne(Synapse::Controller),
                            Dendrite::RequireOne(Synapse::InstanceProvider),
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

impl Soma for ComputerSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, msg: Impulse<Signal, Synapse>) -> organelle::Result<Self> {
        match self {
            ComputerSoma::Init(state) => state.update(msg),
            ComputerSoma::Setup(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init {
    axon:           Axon,

    setup:          PlayerSetup,
}

impl Init {
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<ComputerSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Start => Setup::setup(self.axon, self.setup),


                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerSoma::Init(self))
        }
    }
}

pub struct Setup {
    axon:           Axon,

    setup:          PlayerSetup,
}

impl Setup {
    fn setup(axon: Axon, setup: PlayerSetup) -> Result<ComputerSoma> {
        Ok(ComputerSoma::Setup(Setup { axon: axon, setup: setup }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<ComputerSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::RequestPlayerSetup(_)) => {
                    self.axon.send_req_input(
                        Synapse::Controller, Signal::PlayerSetup(self.setup)
                    )?;

                    Ok(ComputerSoma::Setup(self))
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message")
            }
        }
        else {
            Ok(ComputerSoma::Setup(self))
        }
    }
}
