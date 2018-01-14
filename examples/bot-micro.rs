#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate docopt;
extern crate glob;
extern crate organelle;
extern crate serde;

extern crate sc2;

use std::path::PathBuf;
use std::rc::Rc;

use docopt::Docopt;
use organelle::{Dendrite, Impulse, Organelle, ResultExt, Soma};
use sc2::{
    Axon,
    FrameData,
    GameSettings,
    LauncherSettings,
    Map,
    PlayerSetup,
    Race,
    Result,
    Signal,
    Synapse,
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

pub enum MarineMicroSoma {
    Init(Init),
    Setup(Setup),

    InGame(InGame),
}

impl MarineMicroSoma {
    pub fn organelle(interval: u32) -> Result<Self> {
        Ok(MarineMicroSoma::Init(Init {
            axon: Axon::new(
                vec![Dendrite::RequireOne(Synapse::Agent)],
                vec![],
            )?,
            interval: interval,
        }))
    }
}

impl Soma for MarineMicroSoma {
    type Signal = Signal;
    type Synapse = Synapse;

    fn update(
        self,
        msg: Impulse<Signal, Synapse>,
    ) -> organelle::Result<MarineMicroSoma> {
        match self {
            MarineMicroSoma::Init(state) => state.update(msg),
            MarineMicroSoma::Setup(state) => state.update(msg),

            MarineMicroSoma::InGame(state) => state.update(msg),
        }.chain_err(|| organelle::ErrorKind::SomaError)
    }
}

pub struct Init {
    axon: Axon,
    interval: u32,
}

impl Init {
    fn update(
        mut self,
        msg: Impulse<Signal, Synapse>,
    ) -> Result<MarineMicroSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Start => Setup::setup(self.axon, self.interval),

                _ => bail!("unexpected message"),
            }
        } else {
            Ok(MarineMicroSoma::Init(self))
        }
    }
}

pub struct Setup {
    axon: Axon,
    interval: u32,
}

impl Setup {
    fn setup(axon: Axon, interval: u32) -> Result<MarineMicroSoma> {
        Ok(MarineMicroSoma::Setup(Setup {
            axon: axon,
            interval: interval,
        }))
    }

    fn update(
        mut self,
        msg: Impulse<Signal, Synapse>,
    ) -> Result<MarineMicroSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::RequestPlayerSetup(_)) => {
                    self.axon.send_req_input(
                        Synapse::Agent,
                        Signal::PlayerSetup(PlayerSetup::Player {
                            race: Race::Terran,
                        }),
                    )?;

                    Ok(MarineMicroSoma::Setup(self))
                },
                Impulse::Signal(_, Signal::RequestUpdateInterval) => {
                    self.axon.send_req_input(
                        Synapse::Agent,
                        Signal::UpdateInterval(self.interval),
                    )?;

                    Ok(MarineMicroSoma::Setup(self))
                },
                Impulse::Signal(_, Signal::GameStarted) => {
                    InGame::start(self.axon)
                },

                _ => bail!("unexpected message"),
            }
        } else {
            Ok(MarineMicroSoma::Setup(self))
        }
    }
}

pub struct InGame {
    axon: Axon,
}

impl InGame {
    fn start(axon: Axon) -> Result<MarineMicroSoma> {
        Ok(MarineMicroSoma::InGame(InGame { axon: axon }))
    }

    fn update(
        mut self,
        msg: Impulse<Signal, Synapse>,
    ) -> Result<MarineMicroSoma> {
        if let Some(msg) = self.axon.update(msg)? {
            match msg {
                Impulse::Signal(_, Signal::Observation(frame)) => {
                    self.on_frame(frame)
                },

                _ => bail!("unexpected message"),
            }
        } else {
            Ok(MarineMicroSoma::InGame(self))
        }
    }

    fn on_frame(self, _: Rc<FrameData>) -> Result<MarineMicroSoma> {
        self.axon
            .send_req_input(Synapse::Agent, Signal::UpdateComplete)?;

        Ok(MarineMicroSoma::InGame(self))
    }
}

quick_main!(|| -> Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("bot-micro version {}", VERSION);
        return Ok(());
    }

    let mut organelle =
        Organelle::new(sc2::MeleeSoma::organelle(sc2::MeleeSettings {
            launcher: get_launcher_settings(&args)?,
            players: (
                sc2::AgentSoma::organelle(MarineMicroSoma::organelle(
                    args.flag_step_size.unwrap_or(1),
                )?)?,
                sc2::ComputerSoma::sheath(
                    sc2::Race::Zerg,
                    sc2::Difficulty::VeryEasy,
                )?,
            ),
            suite: sc2::MeleeSuite::OneAndDone(get_game_settings(&args)?),
        })?);

    organelle.add_soma(sc2::CtrlcBreakerSoma::sheath()?);

    organelle.run()?;

    Ok(())
});
