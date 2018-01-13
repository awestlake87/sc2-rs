
use organelle;
use organelle::{ Sheath, ResultExt, Neuron, Impulse, Dendrite };

use super::{
    Result, Axon, Signal, Synapse, Race, Difficulty, PlayerSetup
};

/// soma that acts as the built-in SC2 AI
pub enum ComputerSoma {
    /// initialize the axon
    Init(Init),
    /// respond to the setup queries
    Setup(Setup),
}

impl ComputerSoma {
    /// create a new computer soma
    pub fn sheath(race: Race, difficulty: Difficulty) -> Result<Sheath<Self>> {
        Ok(
            Sheath::new(
                ComputerSoma::Init(
                    Init {
                        setup: PlayerSetup::Computer {
                            race: race,
                            difficulty: difficulty,
                        },
                    }
                ),
                vec![
                    Dendrite::RequireOne(Synapse::Controller),
                    Dendrite::RequireOne(Synapse::InstanceProvider),
                ],
                vec![ ],
            )?
        )
    }
}

impl Neuron for ComputerSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> organelle::Result<Self>
    {
        match self {
            ComputerSoma::Init(state) => state.update(axon, msg),
            ComputerSoma::Setup(state) => state.update(axon, msg),
        }.chain_err(
            || organelle::ErrorKind::SomaError
        )
    }
}

pub struct Init {
    setup:          PlayerSetup,
}

impl Init {
    fn update(self, _axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ComputerSoma>
    {
        match msg {
            Impulse::Start => Setup::setup(self.setup),


            Impulse::Signal(_, msg) => {
                bail!("unexpected message {:#?}", msg)
            },
            _ => bail!("unexpected protocol message")
        }
    }
}

pub struct Setup {
    setup:          PlayerSetup,
}

impl Setup {
    fn setup(setup: PlayerSetup) -> Result<ComputerSoma> {
        Ok(ComputerSoma::Setup(Setup { setup: setup }))
    }

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> Result<ComputerSoma>
    {
        match msg {
            Impulse::Signal(_, Signal::RequestPlayerSetup(_)) => {
                axon.send_req_input(
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
}
