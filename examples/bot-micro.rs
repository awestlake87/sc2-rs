
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate organelle;
extern crate docopt;
extern crate glob;
extern crate serde;

extern crate sc2;

use std::path::PathBuf;
use std::rc::Rc;

use organelle::{ Organelle, Cell, Protocol, ResultExt, Constraint };
use docopt::Docopt;
use sc2::{
    Result,
    Message,
    Role,
    Soma,
    FrameData,
    PlayerSetup,
    Race,
    GameSettings,
    LauncherSettings,
    Map,
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
    pub flag_dir:                   Option<PathBuf>,
    pub flag_port:                  Option<u16>,
    pub flag_map:                   Option<PathBuf>,
    pub flag_replay_dir:            Option<PathBuf>,
    pub flag_wine:                  bool,
    pub flag_version:               bool,
    pub flag_realtime:              bool,
    pub flag_step_size:             Option<u32>,
}

pub fn get_launcher_settings(args: &Args) -> Result<LauncherSettings> {
    let default_settings = LauncherSettings::default();

    Ok(
        LauncherSettings {
            use_wine: args.flag_wine,
            dir: args.flag_dir.clone(),
            base_port: {
                if let Some(port) = args.flag_port {
                    port
                }
                else {
                    default_settings.base_port
                }
            }
        }
    )
}

pub fn get_game_settings(args: &Args) -> Result<GameSettings> {
    let map = match args.flag_map {
        Some(ref map) => Map::LocalMap(map.clone()),
        None => bail!("no map specified")
    };

    Ok(GameSettings { map: map })
}

pub enum MarineMicroCell {
    Init(Init),
    Setup(Setup),

    InGame(InGame),
}

impl MarineMicroCell {
    pub fn organelle(interval: u32) -> Result<Self> {
        Ok(
            MarineMicroCell::Init(
                Init {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::Agent),
                        ],
                        vec![ ],
                    )?,
                    interval: interval,
                }
            )
        )
    }
}

impl Cell for MarineMicroCell {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> organelle::Result<MarineMicroCell>
    {
        match self {
            MarineMicroCell::Init(state) => state.update(msg),
            MarineMicroCell::Setup(state) => state.update(msg),

            MarineMicroCell::InGame(state) => state.update(msg),
        }.chain_err(
            || organelle::ErrorKind::CellError
        )
    }
}

pub struct Init {
    soma:           Soma,
    interval:       u32,
}

impl Init {
    fn update(mut self, msg: Protocol<Message, Role>)
        -> Result<MarineMicroCell>
    {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Start => Setup::setup(self.soma, self.interval),

                _ => bail!("unexpected message")
            }
        }
        else {
            Ok(MarineMicroCell::Init(self))
        }
    }
}

pub struct Setup {
    soma:           Soma,
    interval:       u32,
}

impl Setup {
    fn setup(soma: Soma, interval: u32) -> Result<MarineMicroCell> {
        Ok(MarineMicroCell::Setup(Setup { soma: soma, interval: interval }))
    }

    fn update(mut self, msg: Protocol<Message, Role>)-> Result<MarineMicroCell> {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::RequestPlayerSetup(_)) => {
                    self.soma.send_req_input(
                        Role::Agent,
                        Message::PlayerSetup(
                            PlayerSetup::Player {
                                race: Race::Terran
                            }
                        )
                    )?;

                    Ok(MarineMicroCell::Setup(self))
                },
                Protocol::Message(_, Message::RequestUpdateInterval) => {
                    self.soma.send_req_input(
                        Role::Agent, Message::UpdateInterval(self.interval)
                    )?;

                    Ok(MarineMicroCell::Setup(self))
                },
                Protocol::Message(_, Message::GameStarted) => {
                    InGame::start(self.soma)
                },

                _ => bail!("unexpected message"),
            }
        }
        else {
            Ok(MarineMicroCell::Setup(self))
        }
    }
}

pub struct InGame {
    soma:           Soma,
}

impl InGame {
    fn start(soma: Soma) -> Result<MarineMicroCell> {
        Ok(MarineMicroCell::InGame(InGame { soma: soma }))
    }

    fn update(mut self, msg: Protocol<Message, Role>)
        -> Result<MarineMicroCell>
    {
        if let Some(msg) = self.soma.update(msg)? {
            match msg {
                Protocol::Message(_, Message::Observation(frame)) => {
                    self.on_frame(frame)
                },

                _ => bail!("unexpected message")
            }
        }
        else {
            Ok(MarineMicroCell::InGame(self))
        }
    }

    fn on_frame(self, _: Rc<FrameData>) -> Result<MarineMicroCell> {
        self.soma.send_req_input(Role::Agent, Message::UpdateComplete)?;

        Ok(MarineMicroCell::InGame(self))
    }
}

quick_main!(
    || -> Result<()> {
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
        ;

        if args.flag_version {
            println!("bot-micro version {}", VERSION);
            return Ok(())
        }

        let mut organelle = Organelle::new(
            sc2::MeleeCell::organelle(
                sc2::MeleeSettings {
                    launcher: get_launcher_settings(&args)?,
                    players: (
                        sc2::AgentCell::organelle(
                            MarineMicroCell::organelle(
                                args.flag_step_size.unwrap_or(1)
                            )?
                        )?,
                        sc2::ComputerCell::new(
                            sc2::Race::Zerg, sc2::Difficulty::VeryEasy
                        )?
                    ),
                    suite: sc2::MeleeSuite::OneAndDone(
                        get_game_settings(&args)?
                    )
                }
            )?
        );

        organelle.add_cell(sc2::CtrlcBreakerCell::new()?);

        organelle.run()?;

        Ok(())
    }
);
