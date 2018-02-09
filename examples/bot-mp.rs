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
use rand::random;
use sc2::{
    AgentControl,
    Error,
    GameEvent,
    Launcher,
    LauncherBuilder,
    MeleeBuilder,
    Observation,
    Player,
    Result,
    UpdateScheme,
};
use sc2::data::{
    Ability,
    Action,
    ActionTarget,
    Alliance,
    GameSetup,
    Map,
    MapInfo,
    PlayerSetup,
    Point2,
    Race,
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

const TARGET_SCV_COUNT: usize = 15;

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

pub fn get_game_setup(args: &Args) -> Result<GameSetup> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified"),
    };

    Ok(GameSetup::new(map))
}

struct TerranBot {
    control: AgentControl,
}

impl Player for TerranBot {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSetup) -> Result<(Self, PlayerSetup)> {
        Ok((self, PlayerSetup::Player(Race::Terran)))
    }

    #[async(boxed)]
    fn on_event(mut self, e: GameEvent) -> Result<Self> {
        match e {
            GameEvent::Step => {
                let observation = await!(self.control.observer().observe())?;

                self =
                    await!(self.scout_with_marines(Rc::clone(&observation)))?;
                self = await!(self.try_build_supply_depot(Rc::clone(
                    &observation
                )))?;
                self = await!(self.try_build_scv(Rc::clone(&observation)))?;
                self =
                    await!(self.try_build_barracks(Rc::clone(&observation)))?;
                self = await!(self.try_build_marine(Rc::clone(&observation)))?;
            },

            _ => (),
        }

        Ok(self)
    }
}

impl TerranBot {
    fn new(control: AgentControl) -> Self {
        Self { control: control }
    }

    fn find_enemy_structure(&self, observation: &Observation) -> Option<Tag> {
        let units = observation.filter_units(|u| {
            u.get_alliance() == Alliance::Enemy
                && (u.get_unit_type() == UnitType::TerranCommandCenter
                    || u.get_unit_type() == UnitType::TerranSupplyDepot
                    || u.get_unit_type() == UnitType::TerranBarracks)
        });

        if !units.is_empty() {
            Some(units[0].get_tag())
        } else {
            None
        }
    }

    fn find_enemy_pos(&self, map_info: &MapInfo) -> Option<Point2> {
        if map_info.get_enemy_start_locations().is_empty() {
            None
        } else {
            //TODO: should be random I think
            Some(map_info.get_enemy_start_locations()[0])
        }
    }

    #[async]
    fn scout_with_marines(self, observation: Rc<Observation>) -> Result<Self> {
        let map_info = await!(self.control.observer().get_map_info())?;

        let units = observation.filter_units(|u| {
            u.get_alliance() == Alliance::Domestic
                && u.get_unit_type() == UnitType::TerranMarine
                && u.get_orders().is_empty()
        });

        for u in units {
            match self.find_enemy_structure(&*observation) {
                Some(enemy_tag) => {
                    await!(
                        self.control.action().send_action(
                            Action::new(Ability::Attack)
                                .units([Rc::clone(&u)].iter())
                                .target(ActionTarget::Unit(enemy_tag))
                        )
                    )?;

                    return Ok(self);
                },
                None => (),
            }

            match self.find_enemy_pos(&*map_info) {
                Some(target_pos) => {
                    await!(
                        self.control.action().send_action(
                            Action::new(Ability::Smart)
                                .units([Rc::clone(&u)].iter())
                                .target(ActionTarget::Location(target_pos))
                        )
                    )?;

                    return Ok(self);
                },
                None => (),
            }
        }

        Ok(self)
    }

    #[async]
    fn try_build_supply_depot(
        self,
        observation: Rc<Observation>,
    ) -> Result<Self> {
        // if we are not supply capped, don't build a supply depot
        if observation.get_food_used() + 2 <= observation.get_food_cap() {
            return Ok(self);
        }

        // find a random SVC to build a depot
        await!(self.try_build_structure(observation, Ability::BuildSupplyDepot))
    }

    #[async]
    fn try_build_scv(self, observation: Rc<Observation>) -> Result<Self> {
        let scv_count = observation
            .filter_units(|u| u.get_unit_type() == UnitType::TerranScv)
            .len();

        if scv_count < TARGET_SCV_COUNT {
            await!(self.try_build_unit(
                observation,
                Ability::TrainScv,
                UnitType::TerranCommandCenter,
            ))
        } else {
            Ok(self)
        }
    }

    #[async]
    fn try_build_barracks(self, observation: Rc<Observation>) -> Result<Self> {
        let scv_count = observation
            .filter_units(|u| u.get_unit_type() == UnitType::TerranScv)
            .len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return Ok(self);
        }

        let barracks_count = observation
            .filter_units(|u| u.get_unit_type() == UnitType::TerranBarracks)
            .len();

        if barracks_count > 0 {
            return Ok(self);
        }

        await!(self.try_build_structure(observation, Ability::BuildBarracks))
    }

    #[async]
    fn try_build_marine(self, observation: Rc<Observation>) -> Result<Self> {
        await!(self.try_build_unit(
            observation,
            Ability::TrainMarine,
            UnitType::TerranBarracks,
        ))
    }

    #[async]
    fn try_build_unit(
        self,
        observation: Rc<Observation>,
        ability: Ability,
        unit_type: UnitType,
    ) -> Result<Self> {
        let units = observation.filter_units(|u| {
            u.get_unit_type() == unit_type && u.get_orders().is_empty()
        });

        if units.is_empty() {
            Ok(self)
        } else {
            await!(self.control.action().send_action(
                Action::new(ability).units([Rc::clone(&units[0])].iter())
            ))?;
            Ok(self)
        }
    }

    #[async]
    fn try_build_structure(
        self,
        observation: Rc<Observation>,
        ability: Ability,
    ) -> Result<Self> {
        let units = observation
            .filter_units(|u| u.get_alliance() == Alliance::Domestic);

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in u.get_orders().iter() {
                if o.get_ability() == ability {
                    return Ok(self);
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            await!(
                self.control.action().send_action(
                    Action::new(ability)
                        .units([Rc::clone(&units[u])].iter())
                        .target(ActionTarget::Location(
                            units[u].get_pos_2d() + r * 5.0,
                        )),
                )
            )?;

            Ok(self)
        } else {
            Ok(self)
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

    let melee = MeleeBuilder::new(
        sc2::AgentBuilder::factory(|control| TerranBot::new(control))
            .handle(handle.clone())
            .create()?,
        sc2::AgentBuilder::factory(|control| TerranBot::new(control))
            .handle(handle.clone())
            .create()?,
    ).launcher_settings(get_launcher_settings(&args)?)
        .repeat_forever(get_game_setup(&args)?)
        .update_scheme(UpdateScheme::Interval(args.flag_step_size.unwrap_or(1)))
        .break_on_ctrlc(args.flag_wine)
        .handle(handle)
        .create()?;

    core.run(melee.into_future())?;

    Ok(())
});
