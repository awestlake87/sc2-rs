#![feature(proc_macro, conservative_impl_trait, generators)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate futures_await as futures;
extern crate glob;
extern crate nalgebra as na;
extern crate organelle;
extern crate rand;
extern crate serde;
extern crate tokio_core;

extern crate sc2;

use std::f32;
use std::path::PathBuf;

use docopt::Docopt;
use futures::prelude::*;
use organelle::{visualizer, Soma};
use rand::random;
use sc2::{Agent, AgentControl, Error, LauncherSettings, MeleeBuilder, Result};
use sc2::data::{
    Ability,
    ActionTarget,
    Alliance,
    Command,
    GameEvent,
    GameSettings,
    Map,
    MapInfo,
    PlayerSetup,
    Point2,
    Race,
    UpdateScheme,
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

struct SimpleBot {
    control: AgentControl,

    restarts: u32,
}

impl Agent for SimpleBot {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
        Ok((self, PlayerSetup::Player { race: Race::Terran }))
    }

    #[async(boxed)]
    fn on_event(self, e: GameEvent) -> Result<Self> {
        match e {
            GameEvent::GameStarted => {
                println!("starting a new game ({} restarts)", self.restarts);
                Ok(self)
            },
            GameEvent::Step => await!(self.on_step()),
            _ => Ok(self),
        }
    }
}

impl SimpleBot {
    fn new(control: AgentControl) -> Self {
        Self {
            control: control,

            restarts: 0,
        }
    }

    #[async]
    fn on_step(self) -> Result<Self> {
        let observation = await!(self.control.observer().observe())?;
        let map_info = await!(self.control.observer().get_map_info())?;

        let step = observation.current_step;

        if step % 100 == 0 {
            let units =
                observation.filter_units(|u| u.alliance == Alliance::Domestic);

            for u in units {
                let target = find_random_location(&map_info);
                await!(self.control.action().send_command(Command::Action {
                    units: vec![u],
                    ability: Ability::Smart,
                    target: Some(ActionTarget::Location(target)),
                }))?;
            }
        }

        Ok(self)
    }
}

fn find_random_location(map_info: &MapInfo) -> Point2 {
    let area = map_info.playable_area;
    let (w, h) = area.get_dimensions();

    Point2::new(
        w * random::<f32>() + area.from.x,
        h * random::<f32>() + area.from.y,
    )
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("bot-micro version {}", VERSION);
        return Ok(());
    }

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();

    let mut organelle = MeleeBuilder::new(
        sc2::AgentBuilder::factory(|control| SimpleBot::new(control))
            .create(handle.clone())?,
        sc2::AgentBuilder::factory(|control| SimpleBot::new(control))
            .create(handle.clone())?,
    ).launcher_settings(get_launcher_settings(&args)?)
        .suite(sc2::MeleeSuite::EndlessRepeat(get_game_settings(&args)?))
        .update_scheme(UpdateScheme::Interval(args.flag_step_size.unwrap_or(1)))
        .create(handle.clone())?;

    organelle.add_soma(sc2::CtrlcBreakerSoma::axon());
    organelle.add_soma(visualizer::Soma::organelle(
        visualizer::Settings::default(),
        handle.clone(),
    )?);

    core.run(organelle.run(handle))?;

    Ok(())
});
