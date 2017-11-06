
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use std::collections::{ HashMap };
use std::path::{ MAIN_SEPARATOR };

use docopt::Docopt;
use glob::glob;

use sc2::agent::{ Agent };
use sc2::replay_observer::{ ReplayObserver };
use sc2::coordinator::{ Coordinator };
use sc2::data::{ PlayerSetup, Unit, UnitType };
use sc2::participant::{ Participant, User, Observer };

use examples_common::{
    USAGE, Args, get_coordinator_settings, poll_escape
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Replay {
    game:               u32,
    units_built:        HashMap<UnitType, u32>
}

impl Replay {
    fn new() -> Self {
        Self {
            game: 0,
            units_built: HashMap::new()
        }
    }
}

impl Agent for Replay {
    fn on_game_start(&mut self, _: &mut Participant) {
        self.game += 1;
        self.units_built.clear();
    }
    fn on_unit_created(&mut self, _: &mut Participant, u: &Unit) {
        *self.units_built.entry(u.unit_type).or_insert(0) += 1;
    }

    fn on_game_end(&mut self, p: &mut Participant) {
        let unit_data = p.get_unit_type_data();

        println!("\ngame {} units created: ", self.game);

        for (unit_type, built) in &self.units_built {
            match unit_data.get(unit_type) {
                Some(data) => println!("{}: {}", data.name, built),
                _ => ()
            }
        }
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
        println!("replay version {}", VERSION);
        return;
    }

    let mut coordinator_settings = get_coordinator_settings(&args);

    let replay_glob = glob(
        &format!(
            "{}{}*.SC2Replay",
            args.flag_replay_dir.unwrap().to_string_lossy(),
            if args.flag_wine {
                '\\'
            }
            else {
                MAIN_SEPARATOR
            }
        )
    ).expect("failed to read glob pattern");

    let mut i = 0;
    for entry in replay_glob {
        coordinator_settings.replay_files.push(entry.unwrap());

        i += 1;

        if i >= 100 {
            break
        }
    }

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
