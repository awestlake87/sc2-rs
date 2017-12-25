
mod agent;
mod ctrlc_breaker;
mod launcher;
mod melee;
mod observer;

pub use self::ctrlc_breaker::{ CtrlcBreakerLobe };
pub use self::launcher::{ LauncherLobe };
pub use self::melee::{ MeleeSuite, MeleeSettings, MeleeLobe };
pub use self::observer::{ ObserverLobe };

use cortical;
use cortical::{ Handle };
use uuid::Uuid;

use super::{ Error };
use data::{ GameSettings };

#[derive(Debug)]
pub enum Message {
    LaunchInstance,
    InstancePool(Vec<Uuid>),

    RequestMelee {
        transaction: Uuid,
        player1: Handle,
        player2: Handle,
        settings: GameSettings,
    },
    AssignInstance(Uuid),

    InvalidRequest {
        transaction: Uuid,
        error: Error,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Constraint {
    /// launches new game instances or kills them
    InstanceManager,
    /// broadcasts idle instances
    InstancePool,
    /// assign instances to lobes
    InstanceAssignment,
    /// provides agent interface to bots
    Agent,
}

pub type Protocol = cortical::Protocol<Message, Constraint>;
pub type Effector = cortical::Effector<Message, Constraint>;
pub type Cortex = cortical::Cortex<Message, Constraint>;
