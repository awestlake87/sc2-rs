
use organelle;
use organelle::{ Sheath, Neuron, Impulse };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::{ Result, Axon, Signal, Synapse };

/// soma that stops the organelle upon Ctrl-C
pub struct CtrlcBreakerSoma;

impl CtrlcBreakerSoma {
    /// create a new Ctrl-C breaker soma
    pub fn sheath() -> Result<Sheath<Self>> {
        Ok(Sheath::new(Self { }, vec![ ], vec![ ])?)
    }

    fn init(self, axon: &Axon) -> organelle::Result<Self> {
        let (tx, rx) = mpsc::channel(1);

        ctrlc::set_handler(
            move || {
                tx.clone()
                    .send(())
                    .wait()
                    .unwrap()
                ;
            }
        ).unwrap();

        let ctrlc_effector = axon.effector()?.clone();

        axon.effector()?.spawn(
            rx.for_each(
                move |_| {
                    ctrlc_effector.stop();
                    Ok(())
                }
            )
        );

        Ok(self)
    }
}

impl Neuron for CtrlcBreakerSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, axon: &Axon, msg: Impulse<Signal, Synapse>)
        -> organelle::Result<Self>
    {
        match msg {
            Impulse::Start => self.init(axon),

            _ => Ok(self),
        }
    }
}
