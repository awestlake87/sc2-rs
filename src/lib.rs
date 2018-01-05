#![warn(missing_docs)]
#![recursion_limit = "1024"]

//! StarCraft II API for Rust
//!
//! this API is intended to provide functionality similar to that of Blizzard
//! and Google's [StarCraft II API](https://github.com/Blizzard/s2client-api)

#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate cortical;
extern crate ctrlc;
extern crate futures;
extern crate glob;
extern crate nalgebra as na;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate sc2_proto;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;
extern crate uuid;

mod agent;
mod client;
mod computer;
mod ctrlc_breaker;
mod data;
mod frame;
mod instance;
mod launcher;
mod melee;
mod observer;

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use futures::sync::mpsc::{ Sender };
use url::Url;
use uuid::Uuid;

pub use self::agent::{ AgentLobe };
pub use self::client::{ ClientRequest, ClientResponse, TransactionId };
pub use self::computer::{ ComputerLobe };
pub use self::ctrlc_breaker::{ CtrlcBreakerLobe };
pub use self::data::{
    Color,
    Rect,
    Rect2,
    Point2,
    Point3,
    Vector2,
    Vector3,

    UnitType,
    Ability,
    Upgrade,
    Buff,
    Alliance,

    ActionTarget,
    Tag,
    ImageData,
    Visibility,
    BuffData,
    UpgradeData,
    AbilityData,
    Effect,
    UnitTypeData,
    Score,
    Unit,
    TerrainInfo,
    PowerSource,
    DisplayType,
    SpatialAction,
    Action,
    Map,
    GamePorts,
    PortSet,
    PlayerSetup,
    GameSettings,
    Race,
    Difficulty,
};
pub use self::frame::{
    FrameData,
    Command,
    DebugCommand,
    DebugTextTarget,
    MapState,
    GameEvent,
    GameState,
    GameData,
};
pub use self::launcher::{ LauncherLobe, LauncherSettings };
pub use self::melee::{ MeleeSuite, MeleeSettings, MeleeLobe };
pub use self::observer::{ ObserverLobe };

error_chain! {
    links {
        Cortical(cortical::Error, cortical::ErrorKind) #[doc="cortical glue"];
    }
    foreign_links {
        Io(std::io::Error) #[doc="link io errors"];
        UrlParseError(url::ParseError) #[doc="link to url parse errors"];
        Protobuf(protobuf::ProtobufError) #[doc="link to protobuf errors"];
    }
    errors {
        /// exe was not supplied to the coordinator
        ExeNotSpecified {
            description("exe not specified")
            display("StarCraft II exe was not specified")
        }
        /// exe supplied to the coordinator does not exist
        ExeDoesNotExist(exe: PathBuf) {
            description("exe file does not exist")
            display("StarCraft II exe does not exist at {:?}", exe)
        }

        /// client failed to open connection to the game instance
        ClientOpenFailed {
            description("unable to open connection to the game instance")
            display("client open failed")
        }
        /// client failed to send a message to the game instance
        ClientSendFailed {
            description("unable to send message to the game instance")
            display("client send failed")
        }
        /// client failed to receive a message from the game instance
        ClientRecvFailed {
            description("unable to receive message from game instance")
            display("client recv failed")
        }

        /// errors received from game instance
        GameErrors(errors: Vec<String>) {
            description("errors in game response")
            display("received errors: {:?}", errors)
        }
        /// an error occurred in agent callback
        AgentError {
            description("error occurred in agent callback")
            display("error occurred in agent callback")
        }

        /// invalid protobuf data from game instance
        InvalidProtobuf(msg: String) {
            description("unable to convert protobuf data to game data")
            display("unable to convert protobuf data: {}", msg)
        }
    }
}

trait FromProto<T> where Self: Sized {
    /// convert from protobuf data
    fn from_proto(p: T) -> Result<Self>;
}

trait IntoSc2<T> {
    fn into_sc2(self) -> Result<T>;
}

impl<T, U> IntoSc2<U> for T where U: FromProto<T> {
    fn into_sc2(self) -> Result<U> {
        U::from_proto(self)
    }
}

trait IntoProto<T> {
    /// convert into protobuf data
    fn into_proto(self) -> Result<T>;
}

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

    /// send some request to the game instance
    ClientRequest(ClientRequest),
    /// game instance responded within the expected timeframe
    ClientResponse(ClientResponse),
    /// game instance timed out on request
    ClientTimeout(TransactionId),
    /// client has closed
    ClientClosed,
    /// client encountered a websocket error
    ClientError(Error),

    /// internal-use client successfully connected to instance
    Connected(Sender<tungstenite::Message>),
    /// internal-use client received a message
    ClientReceive(tungstenite::Message),

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
    GameReady(PlayerSetup, Option<GamePorts>),
    /// join an existing game
    JoinGame(GamePorts),

    /// request update interval from player
    RequestUpdateInterval,
    /// respond with preferred update interval in game steps
    UpdateInterval(u32),

    /// game started
    GameStarted,

    /// handle game update
    Update(Rc<FrameData>),
    /// notify the stepper that the lobe is done updating
    UpdateComplete(Vec<Command>, Vec<DebugCommand>),
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
    /// observes game state
    Observer,

    /// provides periodic updates from other lobes to agent
    Stepper,
}

/// type alias for an Sc2 Cortex
pub type Cortex = cortical::Cortex<Message, Role>;
/// type alias for an Sc2 Soma
pub type Soma = cortical::Soma<Message, Role>;
