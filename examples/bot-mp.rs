#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate glob;
extern crate organelle;
extern crate rand;
extern crate serde;

extern crate sc2;

use std::path::PathBuf;
use std::rc::Rc;

use docopt::Docopt;
use organelle::{Dendrite, Impulse, Organelle, ResultExt, Soma};
use rand::random;
use sc2::{
    Ability,
    ActionTarget,
    Alliance,
    Axon,
    Command,
    FrameData,
    GameSettings,
    LauncherSettings,
    Map,
    PlayerSetup,
    Point2,
    Race,
    Result,
    Signal,
    Synapse,
    Tag,
    UnitType,
    Vector2,
};

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

const TARGET_SCV_COUNT: usize = 15;

pub enum TerranSoma {
    Init(Init),
    Setup(Setup),

    InGame(InGame),
}

impl TerranSoma {
    pub fn organelle(interval: u32) -> Result<Self> {
        Ok(TerranSoma::Init(Init {
            axon: Axon::new(
                vec![Dendrite::RequireOne(Synapse::Agent)],
                vec![],
            )?,
            interval: interval,
        }))
    }
}

impl Soma for TerranSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(
        self,
        msg: Impulse<Signal, Synapse>,
    ) -> organelle::Result<TerranSoma> {
        match self {
            TerranSoma::Init(state) => state.update(msg),
            TerranSoma::Setup(state) => state.update(msg),

            TerranSoma::InGame(state) => state.update(msg),
        }.chain_err(|| organelle::ErrorKind::SomaError)
    }
}

pub struct Init {
    axon: Axon,
    interval: u32,
}

impl Init {
    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<TerranSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Start => Setup::setup(self.axon, self.interval),

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        } else {
            Ok(TerranSoma::Init(self))
        }
    }
}

pub struct Setup {
    axon: Axon,
    interval: u32,
}

impl Setup {
    fn setup(axon: Axon, interval: u32) -> Result<TerranSoma> {
        Ok(TerranSoma::Setup(Setup {
            axon: axon,
            interval: interval,
        }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<TerranSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::RequestPlayerSetup(_)) => {
                    self.axon.send_req_input(
                        Synapse::Agent,
                        Signal::PlayerSetup(PlayerSetup::Player {
                            race: Race::Terran,
                        }),
                    )?;

                    Ok(TerranSoma::Setup(self))
                },
                Impulse::Signal(_, Signal::RequestUpdateInterval) => {
                    self.axon.send_req_input(
                        Synapse::Agent,
                        Signal::UpdateInterval(self.interval),
                    )?;

                    Ok(TerranSoma::Setup(self))
                },
                Impulse::Signal(_, Signal::GameStarted) => {
                    InGame::start(self.axon, self.interval)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        } else {
            Ok(TerranSoma::Setup(self))
        }
    }
}

pub struct InGame {
    axon: Axon,
    interval: u32,
}

impl InGame {
    fn start(axon: Axon, interval: u32) -> Result<TerranSoma> {
        Ok(TerranSoma::InGame(InGame {
            axon: axon,
            interval: interval,
        }))
    }

    fn update(mut self, msg: Impulse<Signal, Synapse>) -> Result<TerranSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::Observation(frame)) => {
                    self.on_frame(frame)
                },

                Impulse::Signal(_, Signal::GameEnded) => {
                    Setup::setup(self.axon, self.interval)
                },

                Impulse::Signal(_, msg) => {
                    bail!("unexpected message {:#?}", msg)
                },
                _ => bail!("unexpected protocol message"),
            }
        } else {
            Ok(TerranSoma::InGame(self))
        }
    }

    fn on_frame(self, frame: Rc<FrameData>) -> Result<TerranSoma> {
        let commands = self.create_commands(&*frame)?;

        let agent = self.axon.req_input(Synapse::Agent)?;

        let mut messages: Vec<Signal> = commands
            .into_iter()
            .map(|cmd| Signal::Command(cmd))
            .collect();

        messages.push(Signal::UpdateComplete);

        self.axon.effector()?.send_in_order(agent, messages);

        Ok(TerranSoma::InGame(self))
    }

    fn create_commands(&self, frame: &FrameData) -> Result<Vec<Command>> {
        let mut commands = vec![];
        // if there are marines and the command center is not found, send them
        // scouting.
        if let Some(cmd) = self.scout_with_marines(&frame) {
            commands.push(cmd);
        }

        // build supply depots if they are needed
        if let Some(cmd) = self.try_build_supply_depot(&frame) {
            commands.push(cmd);
            return Ok(commands);
        }

        // build terran SCV's if they are needed
        if let Some(cmd) = self.try_build_scv(&frame) {
            commands.push(cmd);
            return Ok(commands);
        }

        // build barracks if they are ready to be built
        if let Some(cmd) = self.try_build_barracks(&frame) {
            commands.push(cmd);
            return Ok(commands);
        }

        // just keep building marines if possible
        if let Some(cmd) = self.try_build_marine(&frame) {
            commands.push(cmd);
            return Ok(commands);
        }

        Ok(commands)
    }

    fn find_enemy_structure(&self, frame: &FrameData) -> Option<Tag> {
        let units = frame.state.filter_units(|u| {
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

    fn find_enemy_pos(&self, frame: &FrameData) -> Option<Point2> {
        if frame.data.terrain_info.enemy_start_locations.is_empty() {
            None
        } else {
            //TODO: should be random I think
            Some(frame.data.terrain_info.enemy_start_locations[0])
        }
    }

    fn scout_with_marines(&self, frame: &FrameData) -> Option<Command> {
        let units = frame.state.filter_units(|u| {
            u.alliance == Alliance::Domestic
                && u.unit_type == UnitType::TerranMarine
                && u.orders.is_empty()
        });

        for ref u in units {
            match self.find_enemy_structure(frame) {
                Some(enemy_tag) => {
                    return Some(Command::Action {
                        units: vec![Rc::clone(u)],
                        ability: Ability::Attack,
                        target: Some(ActionTarget::UnitTag(enemy_tag)),
                    })
                },
                None => (),
            }

            match self.find_enemy_pos(frame) {
                Some(target_pos) => {
                    return Some(Command::Action {
                        units: vec![Rc::clone(u)],
                        ability: Ability::Smart,
                        target: Some(ActionTarget::Location(target_pos)),
                    })
                },
                None => (),
            }
        }

        None
    }

    fn try_build_supply_depot(&self, frame: &FrameData) -> Option<Command> {
        // if we are not supply capped, don't build a supply depot
        if frame.state.food_used + 2 <= frame.state.food_cap {
            return None;
        }

        // find a random SVC to build a depot
        self.try_build_structure(frame, Ability::BuildSupplyDepot)
    }

    fn try_build_scv(&self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame
            .state
            .filter_units(|u| u.unit_type == UnitType::TerranScv)
            .len();

        if scv_count < TARGET_SCV_COUNT {
            self.try_build_unit(
                frame,
                Ability::TrainScv,
                UnitType::TerranCommandCenter,
            )
        } else {
            None
        }
    }

    fn try_build_barracks(&self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame
            .state
            .filter_units(|u| u.unit_type == UnitType::TerranScv)
            .len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return None;
        }

        let barracks_count = frame
            .state
            .filter_units(|u| u.unit_type == UnitType::TerranBarracks)
            .len();

        if barracks_count > 0 {
            return None;
        }

        self.try_build_structure(frame, Ability::BuildBarracks)
    }

    fn try_build_marine(&self, frame: &FrameData) -> Option<Command> {
        self.try_build_unit(
            frame,
            Ability::TrainMarine,
            UnitType::TerranBarracks,
        )
    }

    fn try_build_unit(
        &self,
        frame: &FrameData,
        ability: Ability,
        unit_type: UnitType,
    ) -> Option<Command> {
        let units = frame
            .state
            .filter_units(|u| u.unit_type == unit_type && u.orders.is_empty());

        if units.is_empty() {
            None
        } else {
            Some(Command::Action {
                units: vec![Rc::clone(&units[0])],
                ability: ability,
                target: None,
            })
        }
    }

    fn try_build_structure(
        &self,
        frame: &FrameData,
        ability: Ability,
    ) -> Option<Command> {
        let units = frame
            .state
            .filter_units(|u| u.alliance == Alliance::Domestic);

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in &u.orders {
                if o.ability == ability {
                    return None;
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            Some(Command::Action {
                units: vec![Rc::clone(&units[u])],
                ability: ability,
                target: Some(ActionTarget::Location(
                    Point2::new(units[u].pos.x, units[u].pos.y) + r * 5.0,
                )),
            })
        } else {
            None
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

    let mut organelle =
        Organelle::new(sc2::MeleeSoma::organelle(sc2::MeleeSettings {
            launcher: get_launcher_settings(&args)?,
            players: (
                sc2::AgentSoma::organelle(TerranSoma::organelle(
                    args.flag_step_size.unwrap_or(1),
                )?)?,
                sc2::AgentSoma::organelle(TerranSoma::organelle(
                    args.flag_step_size.unwrap_or(1),
                )?)?,
            ),
            suite: sc2::MeleeSuite::EndlessRepeat(get_game_settings(&args)?),
        })?);

    organelle.add_soma(sc2::CtrlcBreakerSoma::sheath()?);

    organelle.run()?;

    Ok(())
});
