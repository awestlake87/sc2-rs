#![feature(proc_macro, generators)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate futures_await as futures;
extern crate glob;
extern crate nalgebra as na;
extern crate rand;
extern crate serde;
extern crate tokio_core;

extern crate sc2;

use std::path::PathBuf;

use docopt::Docopt;
use sc2::{LauncherSettings, Result};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
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
  -m <path> --map=<path>            Path to the StarCraft II map.
  -s <count> --step-size=<count>    How many steps to take per call.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_dir: Option<PathBuf>,
    pub flag_port: Option<u16>,
    pub flag_wine: bool,
    pub flag_version: bool,
    pub flag_step_size: Option<u32>,
}

pub fn create_launcher_settings(args: &Args) -> Result<LauncherSettings> {
    let mut settings = LauncherSettings::new().use_wine(args.flag_wine);

    if let Some(dir) = args.flag_dir.clone() {
        settings = settings.install_dir(dir);
    }

    if let Some(port) = args.flag_port {
        settings = settings.base_port(port);
    }

    Ok(settings)
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("replay version {}", VERSION);
        return Ok(());
    }

    unimplemented!()
});
