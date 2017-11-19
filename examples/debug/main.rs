
extern crate docopt;
#[macro_use] extern crate error_chain;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;
use sc2::{
    Agent,
    Coordinator,
    DebugTextTarget,
    User,
    Result,
    Command,
    FrameData
};
use sc2::colors;
use sc2::data::{
    PlayerSetup, Race, Difficulty, Point2, UnitType, Vector3
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
    fn start(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        let mut commands = vec![
            Command::DebugText {
                text: "in the corner".to_string(),
                target: None,
                color: colors::RED
            },
            Command::DebugText {
                text: "screen pos".to_string(),
                target: Some(
                    DebugTextTarget::Screen(Point2::new(1.0, 1.0))
                ),
                color: colors::GREEN
            }
        ];

        for u in &frame.state.units {
            if let Some(data) = frame.data.unit_type_data.get(&u.unit_type) {
                commands.push(
                    Command::DebugText {
                        text: data.name.clone(),
                        target: Some(DebugTextTarget::World(u.pos)),
                        color: colors::WHITE
                    }
                );
            }
        }

        let hatcheries = frame.state.filter_units(
            |u| match u.unit_type {
                UnitType::ZergHatchery => true,
                _ => false
            }
        );

        for hatchery in hatcheries {
            commands.push(
                Command::DebugSphere {
                    center: hatchery.pos, radius: 5.0, color: colors::BLUE
                }
            );

            let min = hatchery.pos + Vector3::new(-5.0, -5.0, 2.0);
            let max = hatchery.pos + Vector3::new(5.0, 5.0, 0.0);

            commands.push(
                Command::DebugBox {
                    min: min, max: max, color: colors::RED
                }
            );

            commands.push(
                Command::DebugLine {
                    p1: min, p2: max, color: colors::BLACK
                }
            );
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
            println!("debug version {}", VERSION);
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
        let p2 = PlayerSetup::Computer {
            race: Race::Terran,
            difficulty: Difficulty::VeryEasy
        };

        coordinator.launch_starcraft(
            vec![
                (p1, Some(User::Agent(Box::from(DebugBot::new())))),
                (p2, None)
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
