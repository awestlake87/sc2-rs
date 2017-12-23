
#[macro_use]
extern crate error_chain;
extern crate glutin;
extern crate nalgebra as na;
extern crate num;
extern crate rand;
#[macro_use]
extern crate serde_derive;

extern crate sc2;

mod marine_micro_bot;
mod terran_bot;

use std::path::PathBuf;

use rand::random;

use sc2::{ CoordinatorSettings, Result, Launcher, LauncherSettings };
use sc2::data::{ Rect2, Point2, TerrainInfo, GameSettings, Map };

pub use marine_micro_bot::{ MarineMicroBot };
pub use terran_bot::{ TerranBot };

pub const USAGE: &'static str = "
StarCraft II Rust API Example.

Usage:
  example (-h | --help)
  example [options]
  example --version

Options:
  -h --help                         Show this screen.
  --version                         Show version.
  --wine                            Use Wine to run StarCraft II (for Linux).
  -d <path> --dir=<path>            Path to the StarCraft II installation.
  -p <port> --port=<port>           Port to make StarCraft II listen on.
  -m <name> --map=<name>            Path to the StarCraft II map.
  -r --realtime                     Run StarCraft II in real time
  -s <count> --step-size=<count>    How many steps to take per call.
  --replay-dir=<path>               Path to a replay pack
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_dir:                   Option<PathBuf>,
    pub flag_port:                  Option<u16>,
    pub flag_map:                   Option<PathBuf>,
    pub flag_replay_dir:            Option<PathBuf>,
    pub flag_wine:                  bool,
    pub flag_version:               bool,
    pub flag_realtime:              bool,
    pub flag_step_size:             Option<usize>,
}

pub fn get_launcher_settings(args: &Args) -> Result<LauncherSettings> {
    let default_settings = LauncherSettings::default();
    
    Ok(
        LauncherSettings {
            use_wine: args.flag_wine,
            dir: args.flag_dir.clone(),
            base_port: {
                if let Some(port) = args.flag_port {
                    port
                }
                else {
                    default_settings.base_port
                }
            }
        }
    )
}

pub fn get_coordinator_settings(args: &Args) -> Result<CoordinatorSettings> {
    let launcher = Launcher::from(get_launcher_settings(args)?)?;

    Ok(
        CoordinatorSettings {
            launcher: launcher,
            replay_files: vec![ ],

            is_realtime: args.flag_realtime,
            step_size: match args.flag_step_size {
                Some(step_size) => step_size,
                None => 1
            },
        }
    )
}

pub fn get_game_settings(args: &Args) -> Result<GameSettings> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified")
    };

    Ok(GameSettings { map: map })
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

pub fn find_random_location_in_rect(r: Rect2) -> Point2 {
    let w = r.to.x - r.from.x;
    let h = r.to.y - r.from.y;

    Point2::new(
        w * random::<f32>() + r.from.x, h * random::<f32>() + r.from.y
    )
}

pub fn find_random_location(terrain_info: &TerrainInfo) -> Point2 {
    find_random_location_in_rect(terrain_info.playable_area)
}
