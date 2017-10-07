
#[macro_use]
extern crate serde_derive;

extern crate sc2;

use std::path::PathBuf;

use sc2::coordinator::{ CoordinatorSettings };
use sc2::game::{ GameSettings, Map };

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
  -m <path> --map=<path>        Path to the StarCraft II map.
  --wine                        Use Wine to run StarCraft II (for Linux users)
  --version                     Show version.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_dir:       Option<PathBuf>,
    pub flag_port:      Option<u16>,
    pub flag_map:       PathBuf,
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

pub fn get_game_settings(args: &Args) -> GameSettings {
    GameSettings {
        map: Map::LocalMap(args.flag_map.clone())
    }
}
