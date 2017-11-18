
extern crate docopt;
#[macro_use] extern crate error_chain;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::{ Result, Coordinator, User };
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

quick_main!(
    || -> Result<()> {
        let mut events = glutin::EventsLoop::new();

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
        ;

        if args.flag_version {
            println!("bot-mp version {}", VERSION);
            return Ok(())
        }

        let coordinator_settings = get_coordinator_settings(&args)?;
        let game_settings = get_game_settings(&args)?;

        let mut coordinator = Coordinator::from_settings(
            coordinator_settings
        )?;

        let p1 = PlayerSetup::Player { race: Race::Terran };
        let p2 = PlayerSetup::Player { race: Race::Terran };

        coordinator.launch_starcraft(
            vec![
                (p1, Some(User::Agent(Box::from(TerranBot::new())))),
                (p2, Some(User::Agent(Box::from(TerranBot::new()))))
            ]
        )?;

        println!("launched!");

        let mut done = false;

        while !done {
            coordinator.start_game(game_settings.clone())?;
            println!("game started!");

            while !done {
                 if !coordinator.update()? {
                     println!("stop updating");
                     break
                 }

                 if poll_escape(&mut events) {
                     done = true;
                 }
            }
        }

        Ok(())
    }
);
