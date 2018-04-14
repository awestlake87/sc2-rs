[![pipeline status](https://gitlab.com/awestlake87/sc2-rs/badges/master/pipeline.svg)](https://gitlab.com/awestlake87/sc2-rs/commits/master)
[![crates version](https://img.shields.io/crates/v/sc2.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Documentation (master)](https://awestlake87.gitlab.io/sc2-rs/sc2/)

Rust implementation of StarCraft II Client API

TODO: Provide links to s2client-api.

This crate is still under heavy development, and I've only just decided the 
direction it's going in regarding futures and streams. It currently relies on 
the nightly `#[async]/await!` at the moment. If enough people push to support 
stable futures, I'll consider moving this requirement into a feature, but at the 
moment, it's *really* convenient to use the experimental instead of the stable
combinators.

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
            self = await!(self.on_event(e))?;

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
    bot1.spawn(&handle, agent.take_event_stream().unwrap()).unwrap();

    // Create a match between our bot and a default SC2 built-in AI Opponent.
    let melee = MeleeBuilder::new()
        .add_player(agent)
        .add_player(OpponentBuilder::new())
        .launcher_settings(LauncherSettings::new())
        .one_and_done(GameSetup::new(Map::LocalMap(
            "maps/Ladder/(2)Bel'ShirVestigeLE (Void).SC2Map"
        )))
        .step_interval(1)
        .handle(&handle)
        .create().unwrap();

    // Run the match to completion on the event loop.
    core.run(melee.into_future()).unwrap();

    Ok(())
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
Wine. I'll try to provide links to guide users through it, though.

TODO: Provide links to Wine SC2 setup walkthrough.
