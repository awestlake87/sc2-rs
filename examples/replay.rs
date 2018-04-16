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
use futures::prelude::*;
use sc2::{LauncherSettings, ReplayBuilder, Result};
use tokio_core::reactor;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const USAGE: &'static str = "
StarCraft II Rust API Example.

Usage:
  example (-h | --help)
  example [options]
  example --version

Options:
  -h --help                             Show this screen.
  --version                             Show version.
  --wine                                Use Wine to run StarCraft II (for Linux).
  -d <path> --dir=<path>                Path to the StarCraft II installation.
  -p <port> --port=<port>               Port to make StarCraft II listen on.
  -m <path> --map=<path>                Path to the StarCraft II map.
  -s <count> --step-size=<count>        How many steps to take per call.
  -i <count> --max-instances=<count>    Max number of instances to use at once.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dir: Option<PathBuf>,
    flag_port: Option<u16>,
    flag_wine: bool,
    flag_version: bool,
    flag_step_size: Option<u32>,
    flag_max_instances: Option<usize>,
}

fn create_launcher_settings(args: &Args) -> Result<LauncherSettings> {
    let mut settings = LauncherSettings::new().use_wine(args.flag_wine);

    if let Some(dir) = args.flag_dir.clone() {
        settings = settings.install_dir(dir);
    }

    if let Some(port) = args.flag_port {
        settings = settings.base_port(port);
    }

    Ok(settings)
}

fn create_replay(args: &Args) -> Result<ReplayBuilder> {
    let replay = ReplayBuilder::new()
        .launcher_settings(create_launcher_settings(args)?)
        .break_on_ctrlc(args.flag_wine)
        .max_instances(args.flag_max_instances.unwrap_or(2));

    Ok(replay)
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("replay version {}", VERSION);
        return Ok(());
    }

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let replay = create_replay(&args)?.handle(&handle).create()?;

    core.run(replay.into_future())?;

    Ok(())
});
