
extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::agent::{ Agent };
use sc2::replay_observer::{ ReplayObserver };
use sc2::coordinator::{ Coordinator };
use sc2::data::{ PlayerSetup, Unit };
use sc2::participant::{ Participant, User };

use examples_common::{
    USAGE, Args, get_coordinator_settings, poll_escape
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Replay {
    units_built:        Vec<u32>
}

impl Replay {
    fn new() -> Self {
        Self { units_built: vec![ ] }
    }
}

impl Agent for Replay {
    fn on_game_start(&mut self, _: &mut Participant) {
        println!("replay started!");
    }

    fn on_unit_created(&mut self, _: &mut Participant, _: &Unit) {
        println!("unit created");
    }

    fn on_step(&mut self, _: &mut Participant) {
    }

    fn on_game_end(&mut self, _: &mut Participant) {
        println!("game ended");
    }
}

impl ReplayObserver for Replay {

}

fn main() {
    let mut events = glutin::EventsLoop::new();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("bot-simple version {}", VERSION);
        return;
    }

    let coordinator_settings = get_coordinator_settings(&args);

    let mut coordinator = Coordinator::from_settings(
        coordinator_settings
    ).unwrap();

    let replay = PlayerSetup::Observer;

    match coordinator.launch_starcraft(
        vec![ (replay, Some(User::Observer(Box::from(Replay::new())))) ]
    ) {
        Ok(_) => println!("launched!"),
        Err(e) => println!("unable to launch game: {}", e)
    };

    let mut done = false;

    while !done {
         match coordinator.update() {
             Ok(true) => (),
             Ok(false) => {
                 println!("stop updating");
                 break
             },
             Err(e) => {
                 eprintln!("update failed: {}", e);
                 break
             }
         };

         if poll_escape(&mut events) {
             done = true;
         }
    }

    match coordinator.cleanup() {
        Ok(_) => println!("shutdown successful"),
        Err(e) => eprintln!("error: {}", e)
    }
}
