
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
    race: sc2::data::Race,
    effector: Option<sc2::Effector>,
    agent: Option<Handle>,
}

impl PlayerLobe {
    fn new(race: sc2::data::Race) -> Self {
        Self { race: race, effector: None, agent: None }
    }

    fn init(mut self, effector: sc2::Effector) -> sc2::Result<Self> {
        self.effector = Some(effector);

        Ok(self)
    }

    fn add_input(mut self, input: Handle, role: sc2::Role)
        -> sc2::Result<Self>
    {
        if role == sc2::Role::Agent {
            if self.agent.is_none() {
                self.agent = Some(input);

                Ok(self)
            }
            else {
                bail!("player can only have 1 agent")
            }
        }
        else {
            bail!("invalid role {:#?}", role)
        }
    }

    fn start(self) -> sc2::Result<Self> {
        if self.agent.is_some() {
            Ok(self)
        }
        else {
            bail!("agent not specified")
        }
    }

    fn join_game(self) -> sc2::Result<Self> {
        self.effector().send(
            self.agent.unwrap(), sc2::Message::JoinGame(self.race)
        );

        Ok(self)
    }

    fn effector(&self) -> &sc2::Effector {
        self.effector.as_ref().unwrap()
    }
}

impl Lobe for PlayerLobe {
    type Message = sc2::Message;
    type Role = sc2::Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),
            Protocol::AddInput(input, role) => {
                self.add_input(input, role)
            },
            Protocol::Start => self.start(),

            Protocol::Message(_, sc2::Message::CreateGame(_)) => {
                self.join_game()
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
        sc2::MeleeLobe::new(
            sc2::MeleeSettings {
                launcher: get_launcher_settings(&args)?,

                players: (
                    PlayerLobe::new(sc2::data::Race::Zerg),
                    PlayerLobe::new(sc2::data::Race::Terran)
                ),

                suite: sc2::MeleeSuite::Single(get_game_settings(&args)?),
            }
        )?
    );

    cortex.add_lobe(sc2::CtrlcBreakerLobe::new());

    cortical::run(cortex)?;

    Ok(())
});
