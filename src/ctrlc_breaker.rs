
use cortical;
use cortical::{ Lobe, Protocol, Effector, };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::{ Result, Message, Role };

/// lobe that stops the cortex upon Ctrl-C
pub struct CtrlcBreakerLobe {
}

impl CtrlcBreakerLobe {
    /// create a new Ctrl-C breaker lobe
    pub fn new() -> Result<Self> {
        Ok(Self { })
    }

    fn init(self, effector: Effector<Message, Role>)
        -> cortical::Result<Self>
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

impl Lobe for CtrlcBreakerLobe {
    type Message = Message;
    type Role = Role;

    fn update(mut self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),

            _ => Ok(self),
        }
    }
}
