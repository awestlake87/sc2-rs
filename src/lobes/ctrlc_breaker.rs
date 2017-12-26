
use cortical;
use cortical::{ Lobe, Protocol };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::super::{ Result };
use super::{ Message, Soma, Role };

/// lobe that stops the cortex upon Ctrl-C
pub struct CtrlcBreakerLobe {
    soma:           Soma,
}

impl CtrlcBreakerLobe {
    /// create a new Ctrl-C breaker lobe
    pub fn new() -> Result<Self> {
        Ok(Self { soma: Soma::new(vec![ ], vec![ ])? })
    }

    fn init(self) -> cortical::Result<Self> {
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

        let ctrlc_effector = self.soma.effector()?.clone();

        self.soma.spawn(
            rx.for_each(
                move |_| {
                    ctrlc_effector.stop();
                    Ok(())
                }
            )
        )?;

        Ok(self)
    }
}

impl Lobe for CtrlcBreakerLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Init(_) => self.init(),

            _ => Ok(self),
        }
    }
}
