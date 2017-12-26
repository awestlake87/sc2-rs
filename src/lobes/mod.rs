
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

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

/// type alias for an Sc2 Cortex
pub type Cortex = cortical::Cortex<Message, Role>;
/// type alias for an Sc2 Soma
pub type Soma = cortical::Soma<Message, Role>;
