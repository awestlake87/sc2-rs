#![feature(generators)]

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
use futures::prelude::await;
use futures::unsync::mpsc;
use rand::random;
use sc2::{
    action::{Action, ActionClient, ActionTarget},
    agent::AgentBuilder,
    data::{Ability, Alliance, GameSetup, Map, MapInfo, Point2, Race},
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
        .one_and_done(GameSetup::new(map))
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

    let melee = create_melee(&args)?
        .add_player(agent1)
        .add_player(agent2)
        .handle(&handle)
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
