
mod agent;
mod client;
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

use super::{ Result };
use data::{ GameSettings, Race };

#[derive(Debug)]
/// the messages that can be sent between Sc2 capable
pub enum Message {
    /// launch an instance
    LaunchInstance,
    /// the pool of instances to choose from
    InstancePool(Vec<(Uuid, Url)>),

    /// allow a lobe to take complete control of an instance
    ProvideInstance(Uuid, Url),
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
    Launcher,
    /// broadcasts idle instances
    InstancePool,
    /// provides instances to clients
    InstanceProvider,

    /// controls agents or observer
    Controller,
    /// provides agent interface to bots
    Agent,
    /// provides client interface to agents or observers
    Client,
}

/// type alias for an Sc2 Effector
pub type Effector = cortical::Effector<Message, Role>;
/// type alias for an Sc2 Cortex
pub type Cortex = cortical::Cortex<Message, Role>;

/// useful structure for storing lobe handles and effectors
pub struct RequiredOnce<T> {
    value: Option<T>,
}

impl<T> RequiredOnce<T> {
    /// create a new empty value
    pub fn new() -> Self {
        Self { value: None }
    }

    /// whether or not the value has been set
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }

    /// set the value or Err if it has already been set
    pub fn set(&mut self, value: T) -> Result<()> {
        if self.value.is_none() {
            self.value = Some(value);

            Ok(())
        }
        else {
            bail!("this can only be set once")
        }
    }

    /// get the value or Err if it has not been set
    pub fn get(&self) -> Result<&T> {
        if self.value.is_some() {
            Ok(self.value.as_ref().unwrap())
        }
        else {
            bail!("required value was never set")
        }
    }
}
