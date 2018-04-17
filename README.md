[![Build Status](https://travis-ci.org/awestlake87/sc2-rs.svg?branch=master)](https://travis-ci.org/awestlake87/sc2-rs)
[![Crates Version](https://img.shields.io/crates/v/sc2.svg)](https://crates.io/crates/sc2)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://docs.rs/sc2/badge.svg)](https://docs.rs/crate/sc2)

[Documentation (master)](https://awestlake87.github.io/sc2-rs/sc2/)

This is my Rust implementation of the [StarCraft II Client API](https://github.com/Blizzard/s2client-api).

This crate is still under heavy development, and I've only just decided the 
direction it's going in regarding futures and streams. It currently relies on 
the nightly `#[async]/await!`, but if enough people push to support stable 
futures, I'll consider moving this requirement into a feature, but at the 
moment, it's *really* convenient to use the experimental features instead of the 
stable combinators.

I tried to keep it close to the s2client-api in terms of the division of 
functionality into interfaces such as the Action interface and Observer 
interface, however there are several differences because for one idiomatic C++ 
and idiomatic Rust don't play well together (and for good reason too!), and also
I was very interested in the work done with futures-rs and thought that neat
asynchronous programming was a good fit for this library. In particular, one of
the core differences between s2client-api and this library is the creation of
bots and the consumption of events.

s2client-api employs polymorphism to define bots, sc2-rs on the other hand uses
channels to communicate between the bot and the API. Let's take a look at the
creation of a simple bare-bones bot.

```rust
#![feature(proc_macro, generators)]

extern crate futures_await as futures;
extern crate tokio_core;
extern crate sc2;

use futures::prelude::*;
use futures::unsync::mpsc;
use sc2::{
    melee::{AgentBuilder, MeleeSetup},
    ai::{OpponentBuilder},
    data::{Map, Race},
    observer::{Event, EventAck},
    
    LauncherSettings,
    MeleeBuilder,

    Result,
    Error,
};
use tokio_core::reactor;

struct SimpleBot;

impl SimpleBot {
    fn new() -> Self {
        Self { }
    }

    /// Spawn our bot's coroutine on the event loop.
    fn spawn(
        self,
        handle: &reactor::Handle,
        rx: mpsc::Receiver<(Event, EventAck)>,
    ) -> Result<()> {
        handle.spawn(self.run(rx).map_err(|e| panic!("{:#?}", e)));

        Ok(())
    }

    /// Run the bot.
    #[async]
    fn run(mut self, rx: mpsc::Receiver<(Event, EventAck)>) -> Result<()> {
        // Loop over the game events.
        #[async]
        for (e, ack) in rx.map_err(|_| -> Error { unreachable!() }) {
            match e {
                // Triggered once at the start of every game.
                Event::GameStarted => println!("Started a new game!"),
                // Triggered every time the game updates.
                Event::Step => println!("Game Stepped!"),
            
                // Ignore the other events for now.
                _ => (),
            }

            // Notify the coordinator that we have consumed this event.
            await!(ack.done())?;
        }

        Ok(())
    }
}
```

This bot is simply designed to take a stream of game events and print messages
for GameStarted and GameStepped. Normally, you would use these events as
opportunities to observe the game state and/or dispatch orders to units. For
now, though, a message is good enough.

```rust
fn main() {
    // Create a new event loop.
    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();
    
    // Create a new Agent and set the Race to Terran.
    let mut agent = AgentBuilder::new().race(Race::Terran);

    // Instantiate our simple bot.
    let bot = SimpleBot::new();

    // Get the event stream from the Agent and spawn our bot's coroutine.
    bot.spawn(&handle, agent.take_event_stream().unwrap()).unwrap();

    // Create a match between our bot and a default SC2 built-in AI Opponent.
    let melee = MeleeBuilder::new()
        .add_player(agent)
        .add_player(OpponentBuilder::new())
        .launcher_settings(LauncherSettings::new())
        .one_and_done(MeleeSetup::new(Map::LocalMap(
            "maps/Ladder/(2)Bel'ShirVestigeLE (Void).SC2Map".into()
        )))
        .step_interval(1)
        .handle(&handle)
        .create()
        .unwrap();

    // Run the match to completion on the event loop.
    core.run(melee.into_future()).unwrap();
}
```

Here we create an event loop, spawn our bot as a coroutine and listen for events
from a Melee (PvP) match against a built-in SC2 AI opponent. 

An important thing to note is that the default LauncherSettings will only find 
your SC2 on Windows. However, since the headless Linux version is not ideal for 
debugging purposes, I've added support for Wine within the library for all of 
the people like me who are too lazy to dual-boot (or just prefer Linux in 
general). The good news is that Wine actually supports SC2, the bad news is that
last time I checked, the support requires newer (possibly staging versions) of 
Wine. 

Here are some helpful links to get you started with that:
- [SC2 Page on Wine HQ](https://appdb.winehq.org/objectManager.php?sClass=version&iId=20882)
- [Ask Ubuntu thread on installing Wine Staging and SC2 for Ubuntu 16.04](https://askubuntu.com/questions/846651/installing-starcraft-2-playonlinux)
