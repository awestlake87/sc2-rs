
use cortical;
use cortical::{ Lobe, Protocol };

use super::{ Message, Effector, Constraint };

pub struct AgentLobe {

}

impl AgentLobe {
    pub fn new() -> Self {
        Self { }
    }

    fn init(self, effector: Effector) -> cortical::Result<Self> {
        Ok(self)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Constraint = Constraint;

    fn update(self, msg: Protocol<Message, Constraint>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),

            _ => Ok(self),
        }
    }
}
