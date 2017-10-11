
mod control;
mod observer;

use sc2_proto::sc2api::{ Response };

use super::{ Result, Error };
use super::agent::Agent;
use super::client::Client;
use super::instance::Instance;
use super::player::Player;
use super::game::{ GameState };

pub use self::control::{ Control };
pub use self::observer::{ Observer };

#[derive(PartialEq, Copy, Clone)]
pub enum AppState {
    Normal,
    Crashed,
    Timeout,
    TimeoutZombie
}

pub struct Participant {
    pub player:                 Player,
    pub instance:               Instance,
    pub client:                 Client,
    pub agent:                  Box<Agent>,

    pub app_state:              AppState,
    pub player_id:              Option<u32>,
    pub game_state:             GameState,
}

impl Participant {
    pub fn new(
        instance: Instance, client: Client, player: Player, agent: Box<Agent>
    )
        -> Participant
    {
        Participant {
            player: player,
            instance: instance,
            client: client,
            agent: agent,

            app_state: AppState::Normal,
            player_id: None,
            game_state: GameState {
                current_game_loop: 0
            }
        }
    }

    fn recv(&mut self) -> Result<Response> {
        if self.app_state != AppState::Normal {
            return Err(Error::Todo("app is in a bad state"))
        }

        let rsp = match self.client.recv() {
            Ok(rsp) => rsp,
            Err(e) => {
                unimplemented!("receive error {}", e);
            }
        };

        if rsp.get_error().len() == 0 {
            Ok(rsp)
        }
        else {
            // the game instance is not responsive
            self.app_state = AppState::Timeout;
            unimplemented!("distinguish between a crash/hang")
        }
    }
}

/* put in participant
*** Cached Data ***
abilities_cached: bool;
unit_types_cached: bool;
upgrades_cached: bool;
buffs_cached: bool;
*/
