#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate docopt;
extern crate glutin;
#[macro_use]
extern crate serde_derive;

extern crate sc2;

use std::path::PathBuf;

use docopt::Docopt;

use sc2::coordinator::{ Coordinator, CoordinatorSettings };
use sc2::game::{ GameSettings, Map };
use sc2::player::{ Player, Difficulty, Race };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Simple StarCraft II Bot.

Usage:
  bot-simple (-h | --help)
  bot-simple [options]
  bot-simple --version

Options:
  -h --help                 Show this screen.
  -d <path> --dir=<path>    Path to the StarCraft II directory.
  -p <port> --port=<port>   Port to make StarCraft II listen on.
  -m <path> --map=<path>    Path to the StarCraft II map.
  --version                 Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dir: Option<PathBuf>,
    flag_port: Option<u16>,
    flag_map: PathBuf,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("nuro version {}", VERSION);
        return;
    }

    let coordinator_settings = get_coordinator_settings(&args);
    let game_settings = get_game_settings(&args);

    let mut coordinator = Coordinator::from_settings(
        coordinator_settings
    ).unwrap();

    let zerg_inst = coordinator.launch().expect("unable to launch Zerg cpu");
    let zerg_cpu = Player::new_computer(
        Race::Zerg,
        Difficulty::VeryEasy
    );

    let observe_inst = coordinator.launch().expect(
        "unable to launch observer"
    );
    let observer = Player::new_observer();

    match coordinator.start_game(
        vec![ (zerg_inst, zerg_cpu), (observe_inst, observer) ],
        game_settings
    ) {
        Ok(_) => println!("game started!"),
        Err(e) => eprintln!("unable to start game: {}", e)
    };

    let mut events = glutin::EventsLoop::new();
    let mut done = false;

    while !done {
         match coordinator.update() {
             Ok(_) => (),
             Err(e) => {
                 eprintln!("update failed: {}", e);
                 break
             }
         };

         events.poll_events(
             |e| match e {
                 glutin::Event::DeviceEvent { event, .. } => match event {
                     glutin::DeviceEvent::Key(
                         glutin::KeyboardInput { virtual_keycode, .. }
                     ) => {
                         match virtual_keycode {
                             Some(glutin::VirtualKeyCode::Escape) => {
                                 done = true;
                             }
                             _ => ()
                         }
                     },
                     _ => ()
                 },
                 _ => ()
             }
         );
    };

    match coordinator.cleanup() {
        Ok(_) => println!("shutdown successful"),
        Err(e) => eprintln!("error: {}", e)
    }
}

fn get_coordinator_settings(args: &Args) -> CoordinatorSettings {
    let default_settings = CoordinatorSettings::default();

    CoordinatorSettings {
        dir: args.flag_dir.clone(),
        port: match args.flag_port {
            Some(port) => port,
            None => default_settings.port
        },
        ..default_settings
    }
}

fn get_game_settings(args: &Args) -> GameSettings {
    GameSettings {
        map: Map::LocalMap(args.flag_map.clone())
    }
}
