
use cortical::{ Lobe, Protocol, Effector };

use super::{ Message };

pub struct LauncherLobe {
    effector:           Option<Effector<Message>>,
}

impl LauncherLobe {
    pub fn new() -> Self {
        Self { effector: None }
    }

    fn init(mut self, effector: Effector<Message>) -> Self {
        self.effector = Some(effector);

        self
    }

    fn effector(&self) -> &Effector<Message> {
        self.effector.as_ref().unwrap()
    }
}

impl Lobe for LauncherLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> Self {
        match msg {
            Protocol::Init(effector) => self.init(effector),



            _ => self
        }
    }
}
