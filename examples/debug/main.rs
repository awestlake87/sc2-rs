
extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use std::rc::Rc;

use docopt::Docopt;
use sc2::{
    Agent,
    Coordinator,
    Participant,
    Debugging,
    Observation,
    DebugCommand,
    DebugTextTarget,
    User
};
use sc2::colors;
use sc2::data::{
    PlayerSetup, Race, Difficulty, Point2, Unit, UnitType, Vector3
};

use examples_common::{
    USAGE,
    Args,
    get_coordinator_settings,
    get_game_settings,
    poll_escape
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct DebugBot {

}

impl DebugBot {
    fn new() -> Self {
        Self { }
    }
}

impl Agent for DebugBot {
    fn on_game_start(&mut self, p: &mut Participant) {
        p.command_debug(
            DebugCommand::DrawText {
                text: "in the corner".to_string(),
                target: None,
                color: colors::RED
            }
        );

        p.command_debug(
            DebugCommand::DrawText {
                text: "screen pos".to_string(),
                target: Some(
                    DebugTextTarget::Screen(Point2::new(1.0, 1.0))
                ),
                color: colors::GREEN
            }
        );
    }

    fn on_unit_created(&mut self, p: &mut Participant, u: &Rc<Unit>) {
        let name = p.get_unit_type_data()[&u.unit_type].name.clone();

        p.command_debug(
            DebugCommand::DrawText {
                text: name,
                target: Some(DebugTextTarget::World(u.pos)),
                color: colors::WHITE
            }
        );

        let hatcheries = p.filter_units(
            |u| match u.unit_type {
                UnitType::ZergHatchery => true,
                _ => false
            }
        );

        for hatchery in hatcheries {
            p.command_debug(
                DebugCommand::DrawSphere {
                    center: hatchery.pos, radius: 5.0, color: colors::BLUE
                }
            );

            let min = hatchery.pos + Vector3::new(-5.0, -5.0, 2.0);
            let max = hatchery.pos + Vector3::new(5.0, 5.0, 0.0);

            p.command_debug(
                DebugCommand::DrawBox {
                    min: min, max: max, color: colors::RED
                }
            );

            p.command_debug(
                DebugCommand::DrawLine {
                    p1: min, p2: max, color: colors::BLACK
                }
            );
        }
    }
}

fn main() {
    let mut events = glutin::EventsLoop::new();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("debug version {}", VERSION);
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
    let p2 = PlayerSetup::Computer {
        race: Race::Terran,
        difficulty: Difficulty::VeryEasy
    };

    match coordinator.launch_starcraft(
        vec![
            (p1, Some(User::Agent(Box::from(DebugBot::new())))),
            (p2, None)
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
