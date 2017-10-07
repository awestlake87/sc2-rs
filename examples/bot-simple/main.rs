#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate docopt;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;

use sc2::coordinator::{ Coordinator };
use sc2::player::{ Player, Difficulty, Race };

use examples_common::{
    USAGE, Args, get_coordinator_settings, get_game_settings
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
