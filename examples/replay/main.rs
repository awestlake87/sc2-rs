
extern crate docopt;
#[macro_use] extern crate error_chain;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use std::collections::{ HashMap };
use std::path::{ MAIN_SEPARATOR };

use docopt::Docopt;
use glob::glob;

use sc2::{
    Agent,
    Coordinator,
    User,
    ReplayObserver,
    Result,
    ResultExt,
    ErrorKind,
    FrameData,
    Command,
    GameEvent
};
use sc2::data::{ PlayerSetup, UnitType };

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
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        self.game += 1;
        self.units_built.clear();

        Ok(vec![ ])
    }

    fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        for e in frame.events {
            if let GameEvent::UnitCreated(u) = e {
                *self.units_built.entry(u.unit_type).or_insert(0) += 1;
            }
        }

        Ok(vec![ ])
    }

    fn end(&mut self, frame: FrameData) -> Result<()> {
        println!("\ngame {} units created: ", self.game);

        for (unit_type, built) in &self.units_built {
            match frame.data.unit_type_data.get(unit_type) {
                Some(data) => println!("{}: {}", data.name, built),
                _ => ()
            }
        }

        Ok(())
    }
}

impl ReplayObserver for Replay {

}

quick_main!(
    || -> Result<()> {
        let mut events = glutin::EventsLoop::new();

        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
        ;

        if args.flag_version {
            println!("replay version {}", VERSION);
            return Ok(())
        }

        let mut coordinator_settings = get_coordinator_settings(&args)?;

        if args.flag_replay_dir.is_none() {
            bail!("replay directory not specified")
        }

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
        ).chain_err(|| ErrorKind::Msg("glob error".into()))?;

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
        )?;

        let replay = PlayerSetup::Observer;

        coordinator.launch_starcraft(
            vec![ (replay, Some(User::Observer(Box::from(Replay::new())))) ]
        )?;

        println!("launched!");

        let mut done = false;

        while !done {
             if !coordinator.update()? {
                 break
             }

             if poll_escape(&mut events) {
                 done = true;
             }
        }

        Ok(())
    }
);
