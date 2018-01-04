
#[macro_use]
extern crate error_chain;

extern crate cortical;
extern crate docopt;
extern crate glob;
extern crate glutin;

extern crate sc2;
extern crate examples_common;

use cortical::{ Cortex };
use docopt::Docopt;
use examples_common::{
    USAGE, Args, get_launcher_settings, get_game_settings, MarineMicroLobe
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

quick_main!(
    || -> sc2::Result<()> {
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit())
        ;

        if args.flag_version {
            println!("bot-micro version {}", VERSION);
            return Ok(())
        }

        let mut cortex = Cortex::new(
            sc2::MeleeLobe::cortex(
                sc2::MeleeSettings {
                    launcher: get_launcher_settings(&args)?,
                    players: (
                        sc2::AgentLobe::cortex(
                            MarineMicroLobe::cortex(
                                args.flag_step_size.unwrap_or(1)
                            )?
                        )?,
                        sc2::ComputerLobe::new(
                            sc2::Race::Zerg, sc2::Difficulty::VeryEasy
                        )?
                    ),
                    suite: sc2::MeleeSuite::OneAndDone(
                        get_game_settings(&args)?
                    )
                }
            )?
        );

        cortex.add_lobe(sc2::CtrlcBreakerLobe::new()?);

        cortical::run(cortex)?;

        Ok(())
    }
);
