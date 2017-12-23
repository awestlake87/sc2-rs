
#[macro_use]
extern crate error_chain;

extern crate cortical;
extern crate ctrlc;
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use docopt::Docopt;
use examples_common::{ USAGE, Args, get_launcher_settings };
use sc2::{ MeleeLobe };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("replay version {}", VERSION);
        return Ok(())
    }

    let lobe = MeleeLobe::new(get_launcher_settings(&args)?)?;

    cortical::run(lobe)?;

    Ok(())
});
