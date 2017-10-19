
extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::agent::{ Agent };
use sc2::coordinator::{ Coordinator };
use sc2::data::{ PlayerSetup, Difficulty, Race, Alliance, Ability };
use sc2::participant::{ Participant, Observer, Actions };
use sc2::utils::{ find_random_location };

use examples_common::{
    USAGE,
    Args,
    get_coordinator_settings,
    get_game_settings,
    poll_escape,
    MarineMicroBot
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

    let marines = PlayerSetup::Player { race: Race::Terran };
    let zerg = PlayerSetup::Computer {
        race: Race::Zerg,
        difficulty: Difficulty::VeryEasy
    };

    match coordinator.launch_starcraft(
        vec![
            (marines, Some(Box::from(MarineMicroBot::new()))),
            (zerg, None)
        ]
    ) {
        Ok(_) => println!("launched!"),
        Err(e) => println!("unable to launch game: {}", e)
    };

    match coordinator.start_game(game_settings) {
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
