
extern crate glutin;
#[macro_use]
extern crate serde_derive;

extern crate sc2;

use std::path::PathBuf;

use sc2::coordinator::{ CoordinatorSettings };
use sc2::data::{ GameSettings, Map };
use sc2::{ Result, Error };

pub const USAGE: &'static str = "
Simple StarCraft II Bot.

Usage:
  bot-simple (-h | --help)
  bot-simple [options]
  bot-simple --version

Options:
  -h --help                     Show this screen.
  -d <path> --dir=<path>        Path to the StarCraft II directory.
  -p <port> --port=<port>       Port to make StarCraft II listen on.
  -m <name> --map=<name>        Name of the Blizzard StarCraft II map.
  --local-map=<path>            Path to a local StarCraft II map.
  --wine                        Use Wine to run StarCraft II (for Linux users).
  --version                     Show version.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_dir:       Option<PathBuf>,
    pub flag_port:      Option<u16>,
    pub flag_map:       Option<String>,
    pub flag_local_map: Option<PathBuf>,
    pub flag_wine:      bool,
    pub flag_version:   bool
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

    Ok(
        GameSettings {
            map: map
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
