
extern crate glutin;
extern crate nalgebra as na;
extern crate num;
#[macro_use]
extern crate serde_derive;

extern crate sc2;

pub mod marine_micro_bot;

use std::path::PathBuf;

use sc2::coordinator::{ CoordinatorSettings };
use sc2::data::{ GameSettings, Map };
use sc2::{ Result, Error };

pub use marine_micro_bot::{ MarineMicroBot };

pub const USAGE: &'static str = "
StarCraft II Rust API Example.

Usage:
  example (-h | --help)
  example [options]
  example --version

Options:
  -h --help                         Show this screen.
  -d <path> --dir=<path>            Path to the StarCraft II installation.
  -p <port> --port=<port>           Port to make StarCraft II listen on.
  -m <name> --map=<name>            Name of the Blizzard StarCraft II map.
  -r --realtime                     Run StarCraft II in real time
  -s <count> --step-size=<count>    How many steps to take per call.
  --local-map=<path>                Path to a local StarCraft II map.
  --wine                            Use Wine to run StarCraft II (for Linux).
  --version                         Show version.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_dir:                   Option<PathBuf>,
    pub flag_port:                  Option<u16>,
    pub flag_map:                   Option<String>,
    pub flag_local_map:             Option<PathBuf>,
    pub flag_wine:                  bool,
    pub flag_version:               bool,
    pub flag_realtime:              bool,
    pub flag_step_size:             Option<usize>,
}

pub fn get_coordinator_settings(args: &Args) -> CoordinatorSettings {
    let default_settings = CoordinatorSettings::default();

    CoordinatorSettings {
        use_wine: args.flag_wine,
        dir: args.flag_dir.clone(),
        port: match args.flag_port {
            Some(port) => port,
            None => default_settings.port
        },
        ..default_settings
    }
}

pub fn get_game_settings(args: &Args) -> Result<GameSettings> {
    let map = match args.flag_map {
        Some(ref map) => match args.flag_local_map {
            None => Map::BlizzardMap(map.clone()),
            _ => return Err(Error::Todo("multiple maps specified"))
        },
        None => match args.flag_local_map {
            Some(ref map) => Map::LocalMap(map.clone()),
            None => return Err(Error::Todo("no map specified"))
        }
    };

    let step_size = match args.flag_step_size {
        Some(step_size) => step_size,
        None => 1
    };

    Ok(
        GameSettings {
            map: map,
            is_realtime: args.flag_realtime,
            step_size: step_size,
        }
    )
}

pub fn poll_escape(events: &mut glutin::EventsLoop) -> bool {
    let mut escape = false;

    events.poll_events(
        |e| match e {
            glutin::Event::DeviceEvent { event, .. } => match event {
                glutin::DeviceEvent::Key(
                    glutin::KeyboardInput { virtual_keycode, .. }
                ) => {
                    match virtual_keycode {
                        Some(glutin::VirtualKeyCode::Escape) => {
                            escape = true;
                        }
                        _ => ()
                    }
                },
                _ => ()
            },
            _ => ()
        }
    );

    escape
}
