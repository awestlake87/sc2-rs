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
use futures::unsync::mpsc;
use sc2::{
    ai::OpponentBuilder,
    data::{Difficulty, Map, Point2, Race},
    debug::{DebugClient, DebugCommand, DebugText, DebugTextTarget},
    melee::{AgentBuilder, MeleeBuilder, MeleeSetup},
    observer::{Event, EventAck, ObserverClient},
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
  -h --help                         Show this screen.
  --version                         Show version.
  --wine                            Use Wine to run StarCraft II (for Linux).
  -d <path> --dir=<path>            Path to the StarCraft II installation.
  -p <port> --port=<port>           Port to make StarCraft II listen on.
  -m <path> --map=<path>            Path to the StarCraft II map.
  -r --realtime                     Run StarCraft II in real time
  -s <count> --step-size=<count>    How many steps to take per call.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dir: Option<PathBuf>,
    flag_port: Option<u16>,
    flag_map: Option<PathBuf>,
    flag_replay_dir: Option<PathBuf>,
    flag_wine: bool,
    flag_version: bool,
    flag_realtime: bool,
    flag_step_size: Option<u32>,
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

fn create_melee(args: &Args) -> Result<MeleeBuilder> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified"),
    };

    let mut melee = MeleeBuilder::new()
        .launcher_settings(create_launcher_settings(&args)?)
        .one_and_done(MeleeSetup::new(map))
        .break_on_ctrlc(args.flag_wine);

    if args.flag_realtime && args.flag_step_size.is_some() {
        bail!("Realtime and Step Size flags are incompatible")
    } else {
        if args.flag_realtime {
            melee = melee.step_realtime();
        } else if let Some(ref step_size) = args.flag_step_size {
            melee = melee.step_interval(*step_size);
        }
    }

    Ok(melee)
}

struct DebugBot {
    observer: ObserverClient,
    debug: DebugClient,
}

impl DebugBot {
    fn new(observer: ObserverClient, debug: DebugClient) -> Self {
        Self {
            observer: observer,
            debug: debug,
        }
    }
    fn spawn(
        self,
        handle: &reactor::Handle,
        rx: mpsc::Receiver<(Event, EventAck)>,
    ) -> Result<()> {
        handle.spawn(self.run(rx).map_err(|e| panic!("{:#?}", e)));

        Ok(())
    }

    #[async]
    fn run(mut self, rx: mpsc::Receiver<(Event, EventAck)>) -> Result<()> {
        #[async]
        for (e, ack) in rx.map_err(|_| -> Error { unreachable!() }) {
            self = await!(self.on_event(e))?;

            await!(ack.done())?;
        }

        Ok(())
    }

    #[async]
    fn on_event(self, e: Event) -> Result<Self> {
        match e {
            Event::Step => await!(self.on_step()),
            _ => Ok(self),
        }
    }

    #[async]
    fn on_step(self) -> Result<Self> {
        let observation = await!(self.observer.observe())?;
        let unit_type_data = await!(self.observer.get_unit_data())?;

        await!(self.debug.send_debug(
            DebugText::new("in the corner".to_string()).color((0xFF, 0, 0)),
        ))?;
        await!(
            self.debug.send_debug(
                DebugText::new("screen pos".to_string())
                    .target(DebugTextTarget::Screen(Point2::new(1.0, 1.0)))
                    .color((0, 0xFF, 0))
            )
        )?;

        let mut commands: Vec<DebugCommand> = observation
            .get_units()
            .iter()
            .map(|u| {
                DebugText::new(
                    unit_type_data[&u.get_unit_type()]
                        .get_name()
                        .into(),
                ).target(DebugTextTarget::World(u.get_pos()))
                    .color((0xFF, 0xFF, 0xFF))
                    .into()
            })
            .collect();

        for cmd in commands {
            await!(self.debug.send_debug(cmd))?;
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

    let mut agent = AgentBuilder::new().race(Race::Terran);
    let bot = DebugBot::new(
        agent.add_observer_client(),
        agent.add_debug_client(),
    );
    bot.spawn(&handle, agent.take_event_stream().unwrap())?;

    let zerg = OpponentBuilder::new()
        .race(Race::Zerg)
        .difficulty(Difficulty::VeryEasy);

    let melee = create_melee(&args)?
        .add_player(agent)
        .add_player(zerg)
        .handle(&handle)
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
