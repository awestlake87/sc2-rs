
#[macro_use]
extern crate error_chain;

extern crate cortical;
extern crate ctrlc;
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use cortical::{ Lobe, Protocol };
use docopt::Docopt;
use examples_common::{ USAGE, Args, get_launcher_settings, get_game_settings };
use sc2::{ Message, MeleeSuite, MeleeLobe, MeleeSettings };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct PlayerLobe {

}

impl PlayerLobe {
    fn new() -> Self {
        Self { }
    }
}

impl Lobe for PlayerLobe {
    type Message = Message;

    fn update(self, _: Protocol<Self::Message>) -> cortical::Result<Self> {
        Ok(self)
    }
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("replay version {}", VERSION);
        return Ok(())
    }

    let lobe = MeleeLobe::new(
        MeleeSettings {
            launcher: get_launcher_settings(&args)?,

            player1: PlayerLobe::new(),
            player2: PlayerLobe::new(),

            suite: MeleeSuite::Single(get_game_settings(&args)?),
        }
    )?;

    cortical::run(lobe)?;

    Ok(())
});
