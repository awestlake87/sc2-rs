
extern crate docopt;
#[macro_use] extern crate error_chain;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::{ Result, Coordinator, User };
use sc2::data::{ PlayerSetup, Difficulty, Race };

use examples_common::{
    USAGE,
    Args,
    get_coordinator_settings,
    get_game_settings,
    poll_escape,
    MarineMicroBot
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
            println!("bot-micro version {}", VERSION);
            return Ok(())
        }

        let coordinator_settings = get_coordinator_settings(&args)?;
        let game_settings = get_game_settings(&args)?;

        let mut coordinator = Coordinator::from_settings(
            coordinator_settings
        )?;

        let marines = PlayerSetup::Player { race: Race::Terran };
        let zerg = PlayerSetup::Computer {
            race: Race::Zerg,
            difficulty: Difficulty::VeryEasy
        };

        coordinator.launch_starcraft(
            vec![
                (marines, Some(User::Agent(Box::from(MarineMicroBot::new())))),
                (zerg, None)
            ]
        )?;
        coordinator.start_game(game_settings)?;

        while !poll_escape(&mut events) {
             coordinator.update()?;
        }

        Ok(())
    }
);
