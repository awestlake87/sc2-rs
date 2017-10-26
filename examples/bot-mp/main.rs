
extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::coordinator::{ Coordinator };
use sc2::data::{ PlayerSetup, Race };

use examples_common::{
    USAGE,
    Args,
    get_coordinator_settings,
    get_game_settings,
    poll_escape,
    TerranBot
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut events = glutin::EventsLoop::new();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("bot-mp version {}", VERSION);
        return;
    }

    let coordinator_settings = get_coordinator_settings(&args);
    let game_settings = get_game_settings(&args).unwrap();

    let mut coordinator = Coordinator::from_settings(
        coordinator_settings
    ).unwrap();

    let p1 = PlayerSetup::Player { race: Race::Terran };
    let p2 = PlayerSetup::Player { race: Race::Terran };

    match coordinator.launch_starcraft(
        vec![
            (p1, Some(Box::from(TerranBot::new()))),
            (p2, Some(Box::from(TerranBot::new())))
        ]
    ) {
        Ok(_) => println!("launched!"),
        Err(e) => println!("unable to launch game: {}", e)
    };

    let mut done = false;

    while !done {
        match coordinator.start_game(game_settings.clone()) {
            Ok(_) => println!("game started!"),
            Err(e) => {
                eprintln!("unable to start game: {}", e);
                break
            }
        };

        while !done {
             match coordinator.update() {
                 Ok(_) => (),
                 Err(e) => {
                     eprintln!("update failed: {}", e);
                     break
                 }
             };

             if poll_escape(&mut events) {
                 done = true;
             }
        }
    }

    match coordinator.cleanup() {
        Ok(_) => println!("shutdown successful"),
        Err(e) => eprintln!("error: {}", e)
    }
}
