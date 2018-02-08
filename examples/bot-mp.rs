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
use organelle::{visualizer, Axon, Constraint, Impulse, Organelle, Soma};
use rand::random;
use sc2::{
    ActionTerminal,
    Agent,
    AgentDendrite,
    Error,
    LauncherSettings,
    MeleeBuilder,
    ObserverTerminal,
    PlayerDendrite,
    PlayerSynapse,
    PlayerTerminal,
    Result,
};
use sc2::data::{
    Ability,
    ActionTarget,
    Alliance,
    Command,
    GameEvent,
    GameSettings,
    Map,
    MapInfo,
    Observation,
    PlayerSetup,
    Point2,
    Race,
    Tag,
    UnitType,
    UpdateScheme,
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

struct TerranBot {
    observer: ObserverTerminal,
    action: ActionTerminal,
}

impl Agent for TerranBot {
    type Error = Error;

    #[async(boxed)]
    fn get_player_setup(self, _: GameSettings) -> Result<(Self, PlayerSetup)> {
        Ok((self, PlayerSetup::Player { race: Race::Terran }))
    }

    #[async(boxed)]
    fn on_event(mut self, e: GameEvent) -> Result<Self> {
        match e {
            GameEvent::Step => {
                let observation = await!(self.observer.clone().observe())?;

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
    fn new(observer: ObserverTerminal, action: ActionTerminal) -> Self {
        Self {
            observer: observer,
            action: action,
        }
    }

    fn find_enemy_structure(&self, observation: &Observation) -> Option<Tag> {
        let units = observation.filter_units(|u| {
            u.alliance == Alliance::Enemy
                && (u.unit_type == UnitType::TerranCommandCenter
                    || u.unit_type == UnitType::TerranSupplyDepot
                    || u.unit_type == UnitType::TerranBarracks)
        });

        if !units.is_empty() {
            Some(units[0].tag)
        } else {
            None
        }
    }

    fn find_enemy_pos(&self, map_info: &MapInfo) -> Option<Point2> {
        if map_info.enemy_start_locations.is_empty() {
            None
        } else {
            //TODO: should be random I think
            Some(map_info.enemy_start_locations[0])
        }
    }

    #[async]
    fn scout_with_marines(self, observation: Rc<Observation>) -> Result<Self> {
        let map_info = await!(self.observer.clone().get_map_info())?;

        let units = observation.filter_units(|u| {
            u.alliance == Alliance::Domestic
                && u.unit_type == UnitType::TerranMarine
                && u.orders.is_empty()
        });

        for u in units {
            match self.find_enemy_structure(&*observation) {
                Some(enemy_tag) => {
                    await!(self.action.clone().send_command(
                        Command::Action {
                            units: vec![Rc::clone(&u)],
                            ability: Ability::Attack,
                            target: Some(ActionTarget::UnitTag(enemy_tag)),
                        }
                    ))?;

                    return Ok(self);
                },
                None => (),
            }

            match self.find_enemy_pos(&*map_info) {
                Some(target_pos) => {
                    await!(self.action.clone().send_command(
                        Command::Action {
                            units: vec![Rc::clone(&u)],
                            ability: Ability::Smart,
                            target: Some(ActionTarget::Location(target_pos)),
                        }
                    ))?;

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
        if observation.food_used + 2 <= observation.food_cap {
            return Ok(self);
        }

        // find a random SVC to build a depot
        await!(self.try_build_structure(observation, Ability::BuildSupplyDepot))
    }

    #[async]
    fn try_build_scv(self, observation: Rc<Observation>) -> Result<Self> {
        let scv_count = observation
            .filter_units(|u| u.unit_type == UnitType::TerranScv)
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
            .filter_units(|u| u.unit_type == UnitType::TerranScv)
            .len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return Ok(self);
        }

        let barracks_count = observation
            .filter_units(|u| u.unit_type == UnitType::TerranBarracks)
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
        let units = observation
            .filter_units(|u| u.unit_type == unit_type && u.orders.is_empty());

        if units.is_empty() {
            Ok(self)
        } else {
            await!(self.action.clone().send_command(Command::Action {
                units: vec![Rc::clone(&units[0])],
                ability: ability,
                target: None,
            }))?;
            Ok(self)
        }
    }

    #[async]
    fn try_build_structure(
        self,
        observation: Rc<Observation>,
        ability: Ability,
    ) -> Result<Self> {
        let units =
            observation.filter_units(|u| u.alliance == Alliance::Domestic);

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in &u.orders {
                if o.ability == ability {
                    return Ok(self);
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            await!(self.action.clone().send_command(Command::Action {
                units: vec![Rc::clone(&units[u])],
                ability: ability,
                target: Some(ActionTarget::Location(
                    units[u].get_pos_2d() + r * 5.0,
                )),
            }))?;

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

    let mut organelle = MeleeBuilder::new(
        sc2::AgentBuilder::factory(|observer, action| {
            TerranBot::new(observer, action)
        }).create(handle.clone())?,
        sc2::AgentBuilder::factory(|observer, action| {
            TerranBot::new(observer, action)
        }).create(handle.clone())?,
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
