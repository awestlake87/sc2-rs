//! [![Build Status](https://travis-ci.org/awestlake87/sc2-rs.svg?branch=master)](https://travis-ci.org/awestlake87/sc2-rs)
//! [![Crates Version](https://img.shields.io/crates/v/sc2.svg)](https://crates.io/crates/sc2)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![Documentation](https://docs.rs/sc2/badge.svg)](https://docs.rs/crate/sc2)
//!
//! [Documentation (master)](https://awestlake87.github.io/sc2-rs/sc2/)
//!
//! This is my Rust implementation of the [StarCraft II Client API](https://github.com/Blizzard/s2client-api).
//!
//! This crate is still under heavy development, and I've only just decided the
//! direction it's going in regarding futures and streams. It currently relies on
//! the nightly `#[async]/await!`, but if enough people push to support stable
//! futures, I'll consider moving this requirement into a feature, but at the
//! moment, it's *really* convenient to use the experimental features instead of the
//! stable combinators.
//!
//! I tried to keep it close to the s2client-api in terms of the division of
//! functionality into interfaces such as the Action interface and Observer
//! interface, however there are several differences because for one idiomatic C++
//! and idiomatic Rust don't play well together (and for good reason too!), and also
//! I was very interested in the work done with futures-rs and thought that neat
//! asynchronous programming was a good fit for this library. In particular, one of
//! the core differences between s2client-api and this library is the creation of
//! bots and the consumption of events.
//!
//! s2client-api employs polymorphism to define bots, sc2-rs on the other hand uses
//! channels to communicate between the bot and the API. Let's take a look at the
//! creation of a simple bare-bones bot.
//!
//! This bot is simply designed to take a stream of game events and print messages
//! for GameStarted and GameStepped. Normally, you would use these events as
//! opportunities to observe the game state and/or dispatch orders to units. For
//! now, though, a message is good enough.
//!
//! ```no_run
//! #![feature(proc_macro, generators)]
//!
//! extern crate futures_await as futures;
//! extern crate tokio_core;
//! extern crate sc2;
//!
//! use futures::prelude::*;
//! use futures::unsync::mpsc;
//! use sc2::{
//!     agent::{AgentBuilder},
//!     ai::{OpponentBuilder},
//!     data::{GameSetup, Map, Race},
//!     observer::{Event, EventAck},
//!
//!     LauncherSettings,
//!     MeleeBuilder,
//!
//!     Result,
//!     Error,
//! };
//! use tokio_core::reactor;
//!
//! struct SimpleBot;
//!
//! impl SimpleBot {
//!     fn new() -> Self {
//!         Self { }
//!     }
//!
//!     /// Spawn our bot's coroutine on the event loop.
//!     fn spawn(
//!         self,
//!         handle: &reactor::Handle,
//!         rx: mpsc::Receiver<(Event, EventAck)>,
//!     ) -> Result<()> {
//!         handle.spawn(self.run(rx).map_err(|e| panic!("{:#?}", e)));
//!
//!         Ok(())
//!     }
//!
//!     /// Run the bot.
//!     #[async]
//!     fn run(mut self, rx: mpsc::Receiver<(Event, EventAck)>) -> Result<()> {
//!         // Loop over the game events.
//!         #[async]
//!         for (e, ack) in rx.map_err(|_| -> Error { unreachable!() }) {
//!             match e {
//!                 // Triggered once at the start of every game.
//!                 Event::GameStarted => println!("Started a new game!"),
//!                 // Triggered every time the game updates.
//!                 Event::Step => println!("Game Stepped!"),
//!             
//!                 // Ignore the other events for now.
//!                 _ => (),
//!             }
//!
//!             // Notify the coordinator that we have consumed this event.
//!             await!(ack.done())?;
//!         }
//!
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     // Create a new event loop.
//!     let mut core = reactor::Core::new().unwrap();
//!     let handle = core.handle();

//!     // Create a new Agent and set the Race to Terran.
//!     let mut agent = AgentBuilder::new().race(Race::Terran);
//!
//!     // Instantiate our simple bot.
//!     let bot = SimpleBot::new();
//!
//!     // Get the event stream from the Agent and spawn our bot's coroutine.
//!     bot.spawn(&handle, agent.take_event_stream().unwrap()).unwrap();
//!
//!     // Create a match between our bot and a default SC2 built-in AI Opponent.
//!     let melee = MeleeBuilder::new()
//!         .add_player(agent)
//!         .add_player(OpponentBuilder::new())
//!         .launcher_settings(LauncherSettings::new())
//!         .one_and_done(GameSetup::new(Map::LocalMap(
//!             "maps/Ladder/(2)Bel'ShirVestigeLE (Void).SC2Map".into()
//!         )))
//!         .step_interval(1)
//!         .handle(&handle)
//!         .create()
//!         .unwrap();
//!
//!     // Run the match to completion on the event loop.
//!     core.run(melee.into_future()).unwrap();
//! }
//! ```
//!
//! Here we create an event loop, spawn our bot as a coroutine and listen for events
//! from a Melee (PvP) match against a built-in SC2 AI opponent.
//!
//! An important thing to note is that the default LauncherSettings will only find
//! your SC2 on Windows. However, since the headless Linux version is not ideal for
//! debugging purposes, I've added support for Wine within the library for all of
//! the people like me who are too lazy to dual-boot (or just prefer Linux in
//! general). The good news is that Wine actually supports SC2, the bad news is that
//! last time I checked, the support requires newer (possibly staging versions) of
//! Wine.
//!
//! Here are some helpful links to get you started with that:
//! - [SC2 Page on Wine HQ](https://appdb.winehq.org/objectManager.php?sClass=version&iId=20882)
//! - [Ask Ubuntu thread on installing Wine Staging and SC2 for Ubuntu 16.04](https://askubuntu.com/questions/846651/installing-starcraft-2-playonlinux)

#![warn(missing_docs)]
#![recursion_limit = "1024"]
#![feature(proc_macro, generators)]

#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate colored;
extern crate ctrlc;
extern crate futures_await as futures;
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

mod constants;
mod instance;
mod launcher;
mod services;

pub mod action;
pub mod agent;
pub mod ai;
pub mod data;
pub mod debug;
pub mod observer;

pub use self::launcher::LauncherSettings;
pub use self::services::melee_service::MeleeBuilder;

use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(std::io::Error) #[doc="Link io errors."];

        Ctrlc(ctrlc::Error) #[doc="Link to Ctrl-C errors."];
        FutureCanceled(futures::Canceled) #[doc="Link to futures."];
        UrlParse(url::ParseError) #[doc="Link to url parse errors."];
        Protobuf(protobuf::ProtobufError) #[doc="Link to protobuf errors."];
        Timer(tokio_timer::TimerError) #[doc="Link to timer errors."];
        Tungstenite(tungstenite::Error) #[doc="Link to tungstenite errors."];
    }
    errors {
        /// Executable was not supplied to the coordinator.
        ExeNotSpecified {
            description("Executable was not supplied to the coordinator")
            display("StarCraft II exe was not specified")
        }
        /// Executable supplied to the coordinator does not exist.
        ExeDoesNotExist(exe: PathBuf) {
            description("Executable supplied to the coordinator does not exist")
            display("StarCraft II exe does not exist at {:?}", exe)
        }

        /// Auto-detecting the SC2 installation was unsuccessful.
        AutoDetectFailed(msg: String) {
            description("Auto-detecting the SC2 installation was unsuccessful")
            display("SC2 Auto-detect failed {}", msg)
        }

        /// An invalid map path was supplied to the library.
        InvalidMapPath(msg: String) {
            description("An invalid map path was supplied to the library")
            display("Invalid map path - {}", msg)
        }

        /// A required field was not provided to a builder.
        ///
        /// Often, a builder will have no suitable default for a value. These
        /// fields require the user to supply a value. When the builder is
        /// finalized, it will check these values and if it is missing a
        /// requirement, you should expect this error.
        MissingRequirement(msg: String) {
            description("A required field was not provided to a builder")
            display("Missing requirement - {}", msg)
        }

        /// Match settings are invalid.
        InvalidMatch(msg: String) {
            description("Match settings are invalid"),
            display("Invalid Match - {}", msg)
        }

        /// Client failed to open connection to the game instance.
        ClientOpenFailed(msg: String) {
            description("Client failed to open connection to the game instance")
            display("Client open failed - {}", msg)
        }
        /// Client failed to send a message to the game instance.
        ClientSendFailed(msg: String) {
            description("Client failed to send a message to the game instance")
            display("Client send failed - {}", msg)
        }
        /// Client failed to receive a message from the game instance.
        ClientRecvFailed(msg: String) {
            description("Client failed to receive a message from the game instance")
            display("Client recv failed - {}", msg)
        }
        /// Client failed to initiate close handshake.
        ClientCloseFailed(msg: String) {
            description("Client failed to complete close handshake")
            display("Client close failed - {}", msg)
        }

        /// Errors received from game instance.
        GameErrors(errors: Vec<String>) {
            description("Errors received from game instance")
            display("Received errors: {:?}", errors)
        }

        /// EventAck receiver was dropped or closed.
        ///
        /// This should not happen in sc2-rs, but any external libraries with
        /// more flexible event subscribers may encounter this problem, and this
        /// error will allow them the freedom to deal with it in their own way.
        EventAckCanceled(msg: String) {
            description("EventAck receiver was dropped or closed")
            display("Event ACK canceled {}", msg)
        }

        /// Invalid protobuf data from game instance.
        InvalidProtobuf(msg: String) {
            description("Invalid protobuf data from game instance")
            display("Unable to convert protobuf data: {}", msg)
        }
    }
}

trait FromProto<T>
where
    Self: Sized,
{
    /// Convert from protobuf data.
    fn from_proto(p: T) -> Result<Self>;
}

trait IntoSc2<T> {
    fn into_sc2(self) -> Result<T>;
}

impl<T, U> IntoSc2<U> for T
where
    U: FromProto<T>,
{
    fn into_sc2(self) -> Result<U> {
        U::from_proto(self)
    }
}

trait IntoProto<T> {
    /// Convert into protobuf data
    fn into_proto(self) -> Result<T>;
}
