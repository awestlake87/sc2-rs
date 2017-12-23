
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use cortical::{ Lobe, Protocol, Cortex, Effector };
use ctrlc;
use futures::prelude::*;
use futures::sync::{ mpsc };

use super::super::{ Result };
use super::{ Message };
use super::launcher::{ LauncherLobe };

pub struct MeleeLobe {

}

impl MeleeLobe {
    pub fn new() -> Cortex<Message> {
        let mut cortex: Cortex<Message> = Cortex::new(
            MeleeLobe { }, ControlLobe::new()
        );

        let launcher = cortex.add_lobe(LauncherLobe::new());

        let versus = cortex.get_input();
        let control = cortex.get_output();

        cortex.connect(versus, launcher);
        cortex.connect(launcher, control);

        cortex
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> Self {
        self
    }
}

pub struct ControlLobe {

}

impl ControlLobe {
    fn new() -> Self {
        Self { }
    }
}

impl Lobe for ControlLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> Self {
        match msg {
            Protocol::Init(effector) => {
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
            },

            _ => (),
        }
        self
    }
}
