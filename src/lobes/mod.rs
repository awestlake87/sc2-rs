
mod agent;
mod client;
mod ctrlc_breaker;
mod instance;
mod launcher;
mod melee;
mod observer;

pub use self::ctrlc_breaker::{ CtrlcBreakerLobe };
pub use self::launcher::{ LauncherLobe, LauncherSettings };
pub use self::melee::{ MeleeSuite, MeleeSettings, MeleeLobe };
pub use self::observer::{ ObserverLobe };

use std::collections::HashMap;

use cortical;
use url::Url;
use uuid::Uuid;

use super::{ Result };
use data::{ GameSettings, GamePorts, PortSet, PlayerSetup };

#[derive(Debug)]
/// the messages that can be sent between Sc2 capable
pub enum Message {
    /// launch an instance
    LaunchInstance,
    /// the pool of instances to choose from
    InstancePool(HashMap<Uuid, (Url, PortSet)>),
    /// the pool of game ports to choose from (num_instances / 2)
    PortsPool(Vec<GamePorts>),

    /// allow a lobe to take complete control of an instance
    ProvideInstance(Uuid, Url),
    /// attempt to connect to instance
    AttemptConnect(Url),

    /// client successfully connected to instance
    Connected,
    /// agent is ready for a game to begin
    Ready,

    /// request player setup
    RequestPlayerSetup(GameSettings),
    /// respond with player setup
    PlayerSetup(PlayerSetup),

    /// create a game with the given settings and list of participants
    CreateGame(GameSettings, Vec<PlayerSetup>),
    /// game was created with the given settings
    GameCreated,
    /// notify agents that game is ready to join with the given player setup
    GameReady(PlayerSetup, GamePorts),
    /// join an existing game
    JoinGame(GamePorts),
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
