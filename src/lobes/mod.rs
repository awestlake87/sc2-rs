
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
use url::Url;
use uuid::Uuid;

use data::{ GameSettings, Race };

#[derive(Debug)]
/// the messages that can be sent between Sc2 capable
pub enum Message {
    /// launch an instance
    LaunchInstance,
    /// the pool of instances to choose from
    InstancePool(Vec<(Uuid, Url)>),

    /// allow a lobe to take complete control of an instance
    AssignInstance(Uuid, Url),
    /// attempt to connect to instance
    AttemptConnect(Url),

    /// create a game with the given settings
    CreateGame(GameSettings),
    /// join an existing game
    JoinGame(Race),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// defines the roles that govern how connections between lobes are made
pub enum Role {
    /// launches new game instances or kills them
    InstanceManager,
    /// broadcasts idle instances
    InstancePool,
    /// controls agents or observer
    Controller,
    /// provides agent interface to bots
    Agent,
}

/// type alias for an Sc2 Effector
pub type Effector = cortical::Effector<Message, Role>;
/// type alias for an Sc2 Cortex
pub type Cortex = cortical::Cortex<Message, Role>;
