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

use std::path::{PathBuf, MAIN_SEPARATOR};

use docopt::Docopt;
use futures::prelude::*;
use glob::glob;
use sc2::{
    replay::{Replay, ReplayBuilder, ReplaySink},
    Error,
    LauncherSettings,
    Result,
};
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
  -r <path> --replay-dir=<path>         Path to a replay pack.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dir: Option<PathBuf>,
    flag_port: Option<u16>,
    flag_wine: bool,
    flag_version: bool,
    flag_step_size: Option<u32>,
    flag_max_instances: Option<usize>,
    flag_replay_dir: Option<PathBuf>,
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

#[async]
fn glob_replays(replay_dir: PathBuf, sink: ReplaySink) -> Result<()> {
    let pattern = format!(
        "{}{}*.SC2Replay",
        replay_dir.to_string_lossy(),
        MAIN_SEPARATOR
    );
    let replay_glob = glob(&pattern).unwrap();

    let mut at_least_one = false;
    let mut num = 0;

    for replay in replay_glob {
        match replay {
            Ok(path) => {
                await!(
                    sink.clone()
                        .send(Replay::LocalReplay(path))
                        .map_err(|_| Error::from("unable to send replay"))
                )?;
            },
            Err(e) => eprintln!("{:?}", e),
        }

        at_least_one = true;

        num += 1;

        if num > 10 {
            break;
        }
    }

    if !at_least_one {
        bail!(
            "No replays matching the pattern {:?} were found",
            pattern
        )
    }

    Ok(())
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

    let replay = create_replay(&args)?.handle(&handle);

    if args.flag_replay_dir.is_none() {
        bail!("Replay Directory is required")
    }

    handle.spawn(
        glob_replays(
            args.flag_replay_dir.unwrap(),
            replay.add_replay_sink(),
        ).map_err(|e| panic!("glob failed! - {:#?}", e)),
    );

    core.run(replay.create()?.into_future())?;

    Ok(())
});
