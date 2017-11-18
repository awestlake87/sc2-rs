
extern crate docopt;
#[macro_use] extern crate error_chain;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::{
    Agent, Coordinator, User, Result, Command, FrameData 
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
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        println!("game started!");

        Ok(vec![ ])
    }

    fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        let mut commands = vec![ ];

        if frame.state.current_step > self.last_update + 100 {
            self.last_update = frame.state.current_step;

            let units = frame.state.filter_units(
                |unit| unit.alliance == Alliance::Domestic
            );

            for unit in units {
                let target = find_random_location(&frame.data.terrain_info);

                commands.push(
                    Command::Action {
                        units: vec![ unit ],
                        ability: Ability::Smart,
                        target: Some(ActionTarget::Location(target))
                    }
                );
            }
            println!("player {} 100 steps...", frame.state.player_id);
        }

        Ok(commands)
    }
}

quick_main!(
    || -> Result<()> {
        let mut events = glutin::EventsLoop::new();

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
        ;

        if args.flag_version {
            println!("bot-simple version {}", VERSION);
            return Ok(())
        }

        let coordinator_settings = get_coordinator_settings(&args)?;
        let game_settings = get_game_settings(&args)?;

        let mut coordinator = Coordinator::from_settings(
            coordinator_settings
        )?;

        let p1 = PlayerSetup::Player {
            race: Race::Zerg,
        };
        let p2 = PlayerSetup::Player {
            race: Race::Terran,
        };

        coordinator.launch_starcraft(
            vec![
                (p1, Some(User::Agent(Box::from(Bot::new())))),
                (p2, Some(User::Agent(Box::from(Bot::new()))))
            ]
        )?;

        println!("launched!");

        coordinator.start_game(game_settings)?;

        println!("game started!");

        while !poll_escape(&mut events) {
             coordinator.update()?;
        }

        Ok(())
    }
);
