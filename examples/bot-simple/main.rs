#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::coordinator::{ Coordinator };
use sc2::player::{ Player, Difficulty, Race };

use examples_common::{
    USAGE, Args, get_coordinator_settings, get_game_settings, poll_escape
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut events = glutin::EventsLoop::new();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("nuro version {}", VERSION);
        return;
    }

    let coordinator_settings = get_coordinator_settings(&args);
    let game_settings = get_game_settings(&args).unwrap();

    let mut coordinator = Coordinator::from_settings(
        coordinator_settings
    ).unwrap();

    let zerg_cpu = Player::new_computer(
        Race::Zerg,
        Difficulty::VeryEasy
    );
    let observer = Player::new_participant(Race::Terran);

    match coordinator.start_game(vec![ zerg_cpu, observer ], game_settings) {
        Ok(_) => println!("game started!"),
        Err(e) => eprintln!("unable to start game: {}", e)
    };

    while !poll_escape(&mut events) {
         match coordinator.update() {
             Ok(_) => (),
             Err(e) => {
                 eprintln!("update failed: {}", e);
                 break
             }
         };
    };

    match coordinator.cleanup() {
        Ok(_) => println!("shutdown successful"),
        Err(e) => eprintln!("error: {}", e)
    }
}
