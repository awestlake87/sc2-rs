
use cortical;
use cortical::{ Lobe, Constraint, Protocol };

use super::{
    Result,

    Message,
    Role,
    Soma,
};

pub enum ObserverLobe {
    Init(Init),

    FetchGameData(FetchGameData),
    FetchTerrainData(FetchTerrainData),

    Observe(Observe),
}

impl ObserverLobe {
    pub fn cortex() -> Result<Self> {
        Ok(
            ObserverLobe::Init(
                Init {
                    soma: Soma::new(
                        vec![ Constraint::RequireOne(Role::Observer) ],
                        vec![ ],
                    )?
                }
            )
        )
    }
}

impl Lobe for ObserverLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> cortical::Result<Self> {
        Ok(self)
    }
}

pub struct Init {
    soma: Soma,
}

pub struct FetchGameData {
    soma: Soma,
}

pub struct FetchTerrainData {
    soma: Soma,
}

pub struct Observe {
    soma: Soma,
}
