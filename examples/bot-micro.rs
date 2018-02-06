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

use docopt::Docopt;
use futures::prelude::*;
use organelle::{visualizer, Axon, Constraint, Impulse, Organelle, Soma};
use sc2::{
    ActionTerminal,
    AgentContract,
    AgentDendrite,
    Error,
    LauncherSettings,
    ObserverTerminal,
    PlayerDendrite,
    PlayerSynapse,
    PlayerTerminal,
    Result,
};
use sc2::data::{
    Difficulty,
    GameEvent,
    GameSettings,
    Map,
    PlayerSetup,
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

pub struct MarineMicroSoma {
    agent: Option<AgentDendrite>,
    observer: Option<ObserverTerminal>,
    action: Option<ActionTerminal>,
}

impl MarineMicroSoma {
    pub fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                agent: None,
                observer: None,
                action: None,
            },
            vec![Constraint::One(PlayerSynapse::Agent)],
            vec![
                Constraint::One(PlayerSynapse::Observer),
                Constraint::One(PlayerSynapse::Action),
            ],
        ))
    }
}

impl Soma for MarineMicroSoma {
    type Synapse = PlayerSynapse;
    type Error = Error;

    #[async(boxed)]
    fn update(self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddDendrite(
                _,
                PlayerSynapse::Agent,
                PlayerDendrite::Agent(rx),
            ) => Ok(Self {
                agent: Some(rx),
                ..self
            }),
            Impulse::AddTerminal(
                _,
                PlayerSynapse::Observer,
                PlayerTerminal::Observer(tx),
            ) => Ok(Self {
                observer: Some(tx),
                ..self
            }),
            Impulse::AddTerminal(
                _,
                PlayerSynapse::Action,
                PlayerTerminal::Action(tx),
            ) => Ok(Self {
                action: Some(tx),
                ..self
            }),

            Impulse::Start(_, main_tx, handle) => {
                handle.spawn(
                    self.agent
                        .unwrap()
                        .wrap(MarineMicroDendrite::new(
                            self.observer.unwrap(),
                            self.action.unwrap(),
                        ))
                        .or_else(move |e| {
                            main_tx
                                .send(Impulse::Error(e.into()))
                                .map(|_| ())
                                .map_err(|_| ())
                        }),
                );

                Ok(Self {
                    agent: None,
                    observer: None,
                    action: None,
                })
            },
            _ => bail!("unexpected impulse"),
        }
    }
}

struct MarineMicroDendrite {
    observer: ObserverTerminal,
    action: ActionTerminal,
}

impl AgentContract for MarineMicroDendrite {
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
            },
            _ => (),
        }

        Ok(self)
    }
}

impl MarineMicroDendrite {
    fn new(observer: ObserverTerminal, action: ActionTerminal) -> Self {
        Self {
            observer: observer,
            action: action,
        }
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

    let mut organelle = Organelle::new(
        sc2::MeleeSoma::organelle(
            sc2::MeleeSettings {
                launcher: get_launcher_settings(&args)?,
                players: (
                    sc2::AgentSoma::organelle(
                        MarineMicroSoma::axon()?,
                        handle.clone(),
                    )?,
                    sc2::ComputerSoma::axon(Race::Zerg, Difficulty::VeryEasy)?,
                ),
                suite: sc2::MeleeSuite::OneAndDone(get_game_settings(&args)?),
                update_scheme: UpdateScheme::Interval(
                    args.flag_step_size.unwrap_or(1),
                ),
            },
            handle.clone(),
        )?,
        handle.clone(),
    );

    organelle.add_soma(sc2::CtrlcBreakerSoma::axon());
    organelle.add_soma(visualizer::Soma::organelle(
        visualizer::Settings::default(),
        handle.clone(),
    )?);

    core.run(organelle.run(handle))?;

    Ok(())
});
