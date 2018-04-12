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

use std::f32;
use std::path::PathBuf;

use docopt::Docopt;
use futures::prelude::*;
use futures::unsync::mpsc;
use rand::random;
use sc2::data::{Ability, Alliance, GameSetup, Map, MapInfo, Point2, Race};
use sc2::{
    action::{Action, ActionClient, ActionTarget},
    agent::AgentBuilder,
    observer::{Event, EventAck, ObserverClient},
    Error,
    LauncherSettings,
    MeleeBuilder,
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

pub fn get_game_setup(args: &Args) -> Result<GameSetup> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified"),
    };

    Ok(GameSetup::new(map))
}

struct SimpleBot {
    observer: ObserverClient,
    action: ActionClient,

    restarts: u32,
}

impl SimpleBot {
    fn new(observer: ObserverClient, action: ActionClient) -> Self {
        Self {
            observer: observer,
            action: action,

            restarts: 0,
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
            Event::GameStarted => {
                println!(
                    "starting a new game ({} restarts)",
                    self.restarts
                );
                Ok(self)
            },
            Event::Step => await!(self.on_step()),
            _ => Ok(self),
        }
    }

    #[async]
    fn on_step(self) -> Result<Self> {
        let observation = await!(self.observer.observe())?;
        let map_info = await!(self.observer.get_map_info())?;

        let step = observation.get_current_step();

        if step % 100 == 0 {
            let units = observation
                .filter_units(|u| u.get_alliance() == Alliance::Domestic);

            for u in units {
                let target = find_random_location(&map_info);
                await!(
                    self.action.send_action(
                        Action::new(Ability::Smart)
                            .units([u].iter())
                            .target(ActionTarget::Location(target))
                    )
                )?;
            }
        }

        Ok(self)
    }
}

fn find_random_location(map_info: &MapInfo) -> Point2 {
    let area = map_info.get_playable_area();
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

    let mut agent1 = AgentBuilder::new().race(Race::Terran);
    let mut agent2 = AgentBuilder::new().race(Race::Terran);

    let bot1 = SimpleBot::new(
        agent1.add_observer_client(),
        agent1.add_action_client(),
    );
    let bot2 = SimpleBot::new(
        agent2.add_observer_client(),
        agent2.add_action_client(),
    );

    bot1.spawn(&handle, agent1.take_event_stream().unwrap())?;
    bot2.spawn(&handle, agent2.take_event_stream().unwrap())?;

    let melee = MeleeBuilder::new()
        .add_player(agent1)
        .add_player(agent2)
        .launcher_settings(create_launcher_settings(&args)?)
        .repeat_forever(get_game_setup(&args)?)
        .step_interval(args.flag_step_size.unwrap_or(1))
        .break_on_ctrlc(args.flag_wine)
        .handle(&handle)
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
