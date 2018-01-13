
use organelle;
use organelle::{ Soma, Impulse, Effector, };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::{ Result, Signal, Synapse };

/// soma that stops the organelle upon Ctrl-C
pub struct CtrlcBreakerSoma {
}

impl CtrlcBreakerSoma {
    /// create a new Ctrl-C breaker soma
    pub fn new() -> Result<Self> {
        Ok(Self { })
    }

    fn init(self, effector: Effector<Signal, Synapse>)
        -> organelle::Result<Self>
    {
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

        let ctrlc_effector = effector.clone();

        effector.spawn(
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

impl Soma for CtrlcBreakerSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(self, msg: Impulse<Signal, Synapse>)
        -> organelle::Result<Self>
    {
        match msg {
            Impulse::Init(effector) => self.init(effector),

            _ => Ok(self),
        }
    }
}
