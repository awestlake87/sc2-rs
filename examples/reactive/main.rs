
#[macro_use]
extern crate error_chain;

extern crate cortical;
extern crate ctrlc;
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use cortical::{ Lobe, Protocol, Handle, Cortex };
use docopt::Docopt;
use examples_common::{ USAGE, Args, get_launcher_settings, get_game_settings };

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct PlayerLobe {
    agent: Option<Handle>,
}

impl PlayerLobe {
    fn new() -> Self {
        Self { agent: None }
    }
}

impl Lobe for PlayerLobe {
    type Message = sc2::Message;
    type Constraint = sc2::Constraint;

    fn update(mut self, msg: Protocol<Self::Message, Self::Constraint>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(_) => {
                self.agent = None;

                Ok(self)
            },
            Protocol::AddInput(input, constraint) => {
                if constraint == sc2::Constraint::Agent {
                    if self.agent.is_none() {
                        self.agent = Some(input);

                        Ok(self)
                    }
                    else {
                        bail!("player can only have 1 agent")
                    }
                }
                else {
                    bail!("invalid constraint {:#?}", constraint)
                }
            },
            Protocol::Start => {
                if self.agent.is_some() {
                    Ok(self)
                }
                else {
                    bail!("agent not specified")
                }
            },

            _ => Ok(self)
        }
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

                players: (PlayerLobe::new(), PlayerLobe::new()),

                suite: sc2::MeleeSuite::Single(get_game_settings(&args)?),
            }
        )?
    );

    cortex.add_lobe(sc2::CtrlcBreakerLobe::new());

    cortical::run(cortex)?;

    Ok(())
});
