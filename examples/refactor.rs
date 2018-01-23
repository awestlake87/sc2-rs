#![feature(proc_macro, conservative_impl_trait, generators)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate futures_await as futures;
extern crate glob;
extern crate organelle;
extern crate rand;
extern crate serde;
extern crate tokio_core;

extern crate sc2;

use std::path::PathBuf;
use std::rc::Rc;

use docopt::Docopt;
use futures::prelude::*;
use organelle::{Axon, Constraint, Impulse, Organelle, ResultExt, Soma};
use rand::random;
use sc2::{
    Ability,
    ActionTarget,
    Alliance,
    CtrlcBreakerSoma,
    //Command,
    Error,
    //FrameData,
    GameSettings,
    LauncherSettings,
    Map,
    PlayerSetup,
    Point2,
    Race,
    Result,
    Synapse,
    Tag,
    UnitType,
    Vector2,
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
    pub flag_dir: Option<PathBuf>,
    pub flag_port: Option<u16>,
    pub flag_map: Option<PathBuf>,
    pub flag_replay_dir: Option<PathBuf>,
    pub flag_wine: bool,
    pub flag_version: bool,
    pub flag_realtime: bool,
    pub flag_step_size: Option<u32>,
}

pub fn get_launcher_settings(args: &Args) -> Result<LauncherSettings> {
    let default_settings = LauncherSettings::default();

    Ok(LauncherSettings {
        use_wine: args.flag_wine,
        dir: args.flag_dir.clone(),
        base_port: {
            if let Some(port) = args.flag_port {
                port
            } else {
                default_settings.base_port
            }
        },
    })
}

pub fn get_game_settings(args: &Args) -> Result<GameSettings> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified"),
    };

    Ok(GameSettings { map: map })
}

pub struct TerranSoma;

impl TerranSoma {
    pub fn organelle(_interval: u32) -> Result<Axon<Self>> {
        Ok(Axon::new(Self {}, vec![], vec![]))
    }
}

impl Soma for TerranSoma {
    type Synapse = Synapse;
    type Error = Error;
    type Future = Box<Future<Item = Self, Error = Self::Error>>;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            _ => bail!("unexpected impulse"),
        }
    }
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("bot-mp version {}", VERSION);
        return Ok(());
    }

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let mut organelle = Organelle::new(
        sc2::MeleeSoma::organelle(
            sc2::MeleeSettings {
                launcher: get_launcher_settings(&args)?,
                players: (
                    sc2::AgentSoma::organelle(
                        TerranSoma::organelle(
                            args.flag_step_size.unwrap_or(1),
                        )?,
                        handle.clone(),
                    )?,
                    sc2::AgentSoma::organelle(
                        TerranSoma::organelle(
                            args.flag_step_size.unwrap_or(1),
                        )?,
                        handle.clone(),
                    )?,
                ),
                suite: sc2::MeleeSuite::EndlessRepeat(get_game_settings(
                    &args,
                )?),
            },
            handle.clone(),
        )?,
        handle.clone(),
    );

    organelle.add_soma(sc2::CtrlcBreakerSoma::axon());

    core.run(organelle.run(handle))?;

    Ok(())
});
