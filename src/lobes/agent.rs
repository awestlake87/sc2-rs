
use cortical;
use cortical::{ Effector, Protocol, Lobe };
use ctrlc;
use futures::prelude::*;
use futures::sync::mpsc;

use super::{ Message };

pub struct AgentLobe {

}

impl AgentLobe {
    pub fn new() -> Self {
        Self { }
    }

    fn init(self, effector: Effector<Message>) -> cortical::Result<Self> {
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

        let done = false;
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

impl Lobe for AgentLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> cortical::Result<Self> {
        match msg {
            Protocol::Init(effector) => self.init(effector),

            _ => Ok(self),
        }
    }
}
