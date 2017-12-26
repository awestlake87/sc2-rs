
#[macro_use]
extern crate error_chain;

extern crate cortical;
extern crate ctrlc;
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use cortical::{ Lobe, Protocol, Handle, Cortex, ResultExt };
use docopt::Docopt;
use examples_common::{ USAGE, Args, get_launcher_settings, get_game_settings };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct PlayerLobe {
    race:               sc2::data::Race,
    effector:           sc2::RequiredOnce<sc2::Effector>,
    agent:              sc2::RequiredOnce<Handle>,
}

impl PlayerLobe {
    fn new(race: sc2::data::Race) -> Self {
        Self {
            race: race,
            effector: sc2::RequiredOnce::new(),
            agent: sc2::RequiredOnce::new(),
        }
    }

    fn init(mut self, effector: sc2::Effector) -> sc2::Result<Self> {
        self.effector.set(effector)?;

        Ok(self)
    }

    fn add_input(mut self, input: Handle, role: sc2::Role)
        -> sc2::Result<Self>
    {
        match role {
            sc2::Role::Agent => self.agent.set(input)?,

            _ => bail!("invalid role {:#?}", role)
        }

        Ok(self)
    }

    fn start(self) -> sc2::Result<Self> {
        Ok(self)
    }

    fn on_req_player_setup(self, src: Handle) -> sc2::Result<Self> {
        assert_eq!(src, *self.agent.get()?);

        self.effector.get()?.send(
            *self.agent.get()?,
            sc2::Message::PlayerSetup(
                sc2::data::PlayerSetup::Player { race: self.race }
            )
        );

        Ok(self)
    }
}

impl Lobe for PlayerLobe {
    type Message = sc2::Message;
    type Role = sc2::Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => {
                self.init(effector)
            },
            Protocol::AddInput(input, role) => {
                self.add_input(input, role)
            },
            Protocol::Start => {
                self.start()
            },

            Protocol::Message(src, sc2::Message::RequestPlayerSetup(_)) => {
                self.on_req_player_setup(src)
            }

            _ => Ok(self)
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

quick_main!(|| -> sc2::Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
    ;

    if args.flag_version {
        println!("replay version {}", VERSION);
        return Ok(())
    }

    let mut cortex = Cortex::new(
        sc2::MeleeLobe::cortex(
            sc2::MeleeSettings {
                launcher: get_launcher_settings(&args)?,

                players: (
                    PlayerLobe::new(sc2::data::Race::Zerg),
                    PlayerLobe::new(sc2::data::Race::Terran)
                ),

                suite: sc2::MeleeSuite::OneAndDone(get_game_settings(&args)?),
            }
        )?
    );

    cortex.add_lobe(sc2::CtrlcBreakerLobe::new());

    cortical::run(cortex)?;

    Ok(())
});
