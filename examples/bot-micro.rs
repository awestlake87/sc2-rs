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
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use docopt::Docopt;
use futures::prelude::*;
use futures::unsync::mpsc;
use sc2::{
    action::{Action, ActionClient, ActionTarget},
    ai::OpponentBuilder,
    data::{Ability, Alliance, Difficulty, Map, Point2, Race, Unit, Vector2},
    melee::{AgentBuilder, MeleeBuilder, MeleeSetup},
    observer::{Event, EventAck, Observation, ObserverClient},
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
  -m <path> --map=<path>            Override the default Marine Micro map path.
  -r --realtime                     Run StarCraft II in real time.
  -s <count> --step-size=<count>    How many steps to take per call.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dir: Option<PathBuf>,
    flag_port: Option<u16>,
    flag_map: Option<PathBuf>,
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
    static DEFAULT_MAP: &str = "./maps/Example/MarineMicro.SC2Map";
    const DEFAULT_STEP: u32 = 1;

    let map = Map::LocalMap(
        args.flag_map
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_MAP)),
    );

    let mut melee = MeleeBuilder::new()
        .launcher_settings(create_launcher_settings(&args)?)
        .one_and_done(MeleeSetup::new(map))
        .break_on_ctrlc(args.flag_wine);

    if args.flag_realtime && args.flag_step_size.is_some() {
        bail!("Realtime and Step Size flags are incompatible")
    } else {
        if args.flag_realtime {
            melee = melee.step_realtime();
        } else {
            melee = melee
                .step_interval(args.flag_step_size.unwrap_or(DEFAULT_STEP));
        }
    }

    Ok(melee)
}

struct MarineMicroBot {
    observer: ObserverClient,
    action: ActionClient,

    targeted_zergling: Option<Rc<Unit>>,
    move_back: bool,
    backup_target: Option<Point2>,
    backup_start: Option<Point2>,
}

impl MarineMicroBot {
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
    fn on_event(mut self, e: Event) -> Result<Self> {
        match e {
            Event::GameStarted => {
                self.move_back = false;
                self.targeted_zergling = None;

                Ok(self)
            },
            Event::UnitDestroyed(unit) => await!(self.on_unit_destroyed(unit)),
            Event::Step => await!(self.on_step()),
            _ => Ok(self),
        }
    }

    fn new(observer: ObserverClient, action: ActionClient) -> Self {
        Self {
            observer: observer,
            action: action,

            targeted_zergling: None,
            move_back: false,
            backup_target: None,
            backup_start: None,
        }
    }

    #[async]
    fn on_step(mut self) -> Result<Self> {
        let observation = await!(self.observer.observe())?;

        let marines = observation
            .filter_units(|u| u.get_alliance() == Alliance::Domestic);

        let marine_pos = match get_center_of_mass(&marines) {
            Some(pos) => pos,
            None => return Ok(self),
        };

        self.targeted_zergling = get_nearest_enemy(&*observation, marine_pos);

        if let Some(zergling) = self.targeted_zergling.clone() {
            if !self.move_back {
                await!(
                    self.action.send_action(
                        Action::new(Ability::Attack)
                            .units(marines.iter())
                            .target(ActionTarget::Unit(zergling.get_tag()))
                    )
                )?;
            } else {
                if let Some(backup_target) = self.backup_target {
                    await!(
                        self.action.send_action(
                            Action::new(Ability::Smart)
                                .units(marines.iter())
                                .target(ActionTarget::Location(backup_target))
                        )
                    )?;

                    if na::distance(&marine_pos, &backup_target) < 1.5 {
                        self.move_back = false;
                    }
                }
            }
        }

        Ok(self)
    }

    #[async]
    fn on_unit_destroyed(mut self, unit: Rc<Unit>) -> Result<Self> {
        let observation = await!(self.observer.observe())?;

        if let Some(targeted_zergling) =
            mem::replace(&mut self.targeted_zergling, None)
        {
            if unit.get_tag() == targeted_zergling.get_tag() {
                let marines = observation
                    .filter_units(|u| u.get_alliance() == Alliance::Domestic);
                let zerglings = observation
                    .filter_units(|u| u.get_alliance() == Alliance::Enemy);

                let marine_pos = match get_center_of_mass(&marines) {
                    Some(pos) => pos,
                    None => return Ok(self),
                };
                let zerg_pos = match get_center_of_mass(&zerglings) {
                    Some(pos) => pos,
                    None => return Ok(self),
                };

                let diff = marine_pos - zerg_pos;
                let direction = na::normalize(&diff);

                self.move_back = true;
                self.backup_start = Some(marine_pos);
                self.backup_target = Some(Point2::from_coordinates(
                    marine_pos.coords + direction * 3.0,
                ));
            }
        }
        Ok(self)
    }
}

fn get_center_of_mass(units: &[Rc<Unit>]) -> Option<Point2> {
    if units.len() == 0 {
        None
    } else {
        let sum = units
            .iter()
            .fold(Vector2::new(0.0, 0.0), |acc, u| {
                acc + u.get_pos_2d().coords
            });

        Some(Point2::from_coordinates(
            sum / (units.len() as f32),
        ))
    }
}

fn get_nearest_enemy(
    observation: &Observation,
    pos: Point2,
) -> Option<Rc<Unit>> {
    let units =
        observation.filter_units(|u| u.get_alliance() == Alliance::Enemy);

    let mut min = f32::MAX;
    let mut nearest = None;

    for u in units {
        let d = na::distance_squared(&u.get_pos_2d(), &pos);

        if d < min {
            min = d;
            nearest = Some(u);
        }
    }

    nearest
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

    let mut bot_agent = AgentBuilder::new().race(Race::Terran);
    let bot = MarineMicroBot::new(
        bot_agent.add_observer_client(),
        bot_agent.add_action_client(),
    );

    bot.spawn(&handle, bot_agent.take_event_stream().unwrap())?;

    let zerg = OpponentBuilder::new()
        .race(Race::Zerg)
        .difficulty(Difficulty::VeryEasy);

    let melee = create_melee(&args)?
        .add_player(bot_agent)
        .add_player(zerg)
        .handle(&handle)
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
