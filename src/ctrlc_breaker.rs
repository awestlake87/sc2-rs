
use organelle;
use organelle::{ Cell, Protocol, Effector, };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::{ Result, Message, Role };

/// cell that stops the organelle upon Ctrl-C
pub struct CtrlcBreakerCell {
}

impl CtrlcBreakerCell {
    /// create a new Ctrl-C breaker cell
    pub fn new() -> Result<Self> {
        Ok(Self { })
    }

    fn init(self, effector: Effector<Message, Role>)
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

impl Cell for CtrlcBreakerCell {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> organelle::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),

            _ => Ok(self),
        }
    }
}
