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

use std::path::PathBuf;

use docopt::Docopt;
use futures::prelude::*;
use sc2::{
    AgentControl,
    Error,
    GameEvent,
    Launcher,
    LauncherBuilder,
    MeleeBuilder,
    Player,
    Result,
    UpdateScheme,
};
use sc2::data::{
    DebugCommand,
    DebugText,
    DebugTextTarget,
    Difficulty,
    GameSettings,
    Map,
    PlayerSetup,
    Point2,
    Race,
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

pub fn get_launcher_settings(args: &Args) -> Result<Launcher> {
    let mut builder = LauncherBuilder::new().use_wine(args.flag_wine);

    if let Some(dir) = args.flag_dir.clone() {
        builder = builder.install_dir(dir);
    }

    if let Some(port) = args.flag_port {
        builder = builder.base_port(port);
    }

    Ok(builder.create()?)
}

pub fn get_game_settings(args: &Args) -> Result<GameSettings> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified"),
    };

    Ok(GameSettings { map: map })
}

struct DebugBot {
    control: AgentControl,
}

impl Player for DebugBot {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
        Ok((self, PlayerSetup::Player { race: Race::Terran }))
    }

    #[async(boxed)]
    fn on_event(self, e: GameEvent) -> Result<Self> {
        match e {
            GameEvent::Step => await!(self.on_step()),
            _ => Ok(self),
        }
    }
}

impl DebugBot {
    fn new(control: AgentControl) -> Self {
        Self { control: control }
    }

    #[async]
    fn on_step(self) -> Result<Self> {
        let observation = await!(self.control.observer().observe())?;
        let unit_type_data = await!(self.control.observer().get_unit_data())?;

        await!(self.control.action().send_debug(
            DebugText::new("in the corner".to_string()).color((0xFF, 0, 0)),
        ))?;
        await!(
            self.control.action().send_debug(
                DebugText::new("screen pos".to_string())
                    .target(DebugTextTarget::Screen(Point2::new(1.0, 1.0)))
                    .color((0, 0xFF, 0))
            )
        )?;

        let mut commands: Vec<DebugCommand> = observation
            .units
            .iter()
            .map(|u| {
                DebugText::new(unit_type_data[&u.unit_type].name.clone())
                    .target(DebugTextTarget::World(u.get_pos()))
                    .color((0xFF, 0xFF, 0xFF))
                    .into()
            })
            .collect();

        for cmd in commands {
            await!(self.control.action().send_debug(cmd))?;
        }

        Ok(self)
    }
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

    let melee = MeleeBuilder::new(
        sc2::AgentBuilder::factory(|control| DebugBot::new(control))
            .handle(handle.clone())
            .create()?,
        sc2::ComputerBuilder::new()
            .race(Race::Zerg)
            .difficulty(Difficulty::VeryEasy)
            .create()?,
    ).launcher_settings(get_launcher_settings(&args)?)
        .one_and_done(get_game_settings(&args)?)
        .update_scheme(UpdateScheme::Interval(args.flag_step_size.unwrap_or(1)))
        .break_on_ctrlc(args.flag_wine)
        .handle(handle.clone())
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
