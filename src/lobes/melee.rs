
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use cortical::{ Lobe, Protocol, Cortex, Effector, Handle };
use ctrlc;
use futures::prelude::*;
use futures::sync::{ mpsc };

use super::super::{ Result, LauncherSettings };
use super::{ Message };
use super::launcher::{ LauncherLobe };

pub struct MeleeLobe {
    effector:       Option<Effector<Message>>,
    output:         Option<Handle>,
}

impl MeleeLobe {
    pub fn new(settings: LauncherSettings) -> Result<Cortex<Message>> {
        let mut cortex: Cortex<Message> = Cortex::new(
            MeleeLobe {
                effector: None,
                output: None,
            },
            ControlLobe::new()
        );

        let launcher = cortex.add_lobe(LauncherLobe::from(settings)?);

        let versus = cortex.get_input();
        let control = cortex.get_output();

        cortex.connect(versus, launcher);
        cortex.connect(launcher, control);

        Ok(cortex)
    }

    fn effector(&self) -> &Effector<Message> {
        self.effector.as_ref().unwrap()
    }

    fn init(mut self, effector: Effector<Message>) -> Self {
        self.effector = Some(effector);

        self
    }

    fn set_output(mut self, output: Handle) -> Self {
        assert!(self.output.is_none());

        self.output = Some(output);

        self
    }

    fn launch(self) -> Self {
        self.effector().send(self.output.unwrap(), Message::LaunchInstance);

        self
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> Self {
        match msg {
            Protocol::Init(effector) => self.init(effector),
            Protocol::AddOutput(output) => self.set_output(output),
            Protocol::Start => self.launch().launch(),

            _ => self,
        }
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
