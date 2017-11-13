
extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::{
    Agent, Coordinator, Participant, Observation, Actions, User, Result
};
use sc2::data::{ PlayerSetup, Race, Alliance, Ability, ActionTarget };

use examples_common::{
    USAGE,
    Args,
    get_coordinator_settings,
    get_game_settings,
    poll_escape,
    find_random_location
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Bot {
    last_update: u32
}

impl Bot {
    fn new() -> Self {
        Self { last_update: 0 }
    }
}

impl Agent for Bot {
    fn on_game_full_start(&mut self, _: &mut Participant) -> Result<()> {
        println!("FULL FUCK YEYA!");

        Ok(())
    }
    fn on_game_start(&mut self, _: &mut Participant) -> Result<()> {
        println!("FUCK YEYA!");

        Ok(())
    }

    fn on_step(&mut self, game: &mut Participant) -> Result<()> {
        if game.get_game_loop() > self.last_update + 100 {
            self.last_update = game.get_game_loop();

            let units = game.filter_units(
                |unit| unit.alliance == Alliance::Domestic
            );

            for unit in units {
                let target = find_random_location(game.get_terrain_info()?);

                game.command_units(
                    &vec![ unit ],
                    Ability::Smart,
                    Some(ActionTarget::Location(target))
                );
            }
            println!("player {} 100 steps...", game.get_player_id().unwrap());
        }

        Ok(())
    }
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
    let game_settings = get_game_settings(&args).unwrap();

    let mut coordinator = Coordinator::from_settings(
        coordinator_settings
    ).unwrap();

    let p1 = PlayerSetup::Player {
        race: Race::Zerg,
    };
    let p2 = PlayerSetup::Player {
        race: Race::Terran,
    };

    match coordinator.launch_starcraft(
        vec![
            (p1, Some(User::Agent(Box::from(Bot::new())))),
            (p2, Some(User::Agent(Box::from(Bot::new()))))
        ]
    ) {
        Ok(_) => println!("launched!"),
        Err(e) => println!("unable to launch game: {}", e)
    }

    match coordinator.start_game(game_settings) {
        Ok(_) => println!("game started!"),
        Err(e) => eprintln!("unable to start game: {}", e)
    }

    while !poll_escape(&mut events) {
         match coordinator.update() {
             Ok(_) => (),
             Err(e) => {
                 eprintln!("update failed: {}", e);
                 break
             }
         };
    }
}
