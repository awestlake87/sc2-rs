
mod control;
mod observer;

use super::agent::Agent;
use super::client::Client;
use super::instance::Instance;
use super::player::Player;
use super::game::{ GameState };

pub use self::control::{ Control };
pub use self::observer::{ Observer };

pub struct Participant {
    pub player:                 Player,
    pub instance:               Instance,
    pub client:                 Client,
    pub agent:                  Box<Agent>,

    pub player_id:              u32,
    pub game_state:             GameState,
}

/* put in participant
*** Cached Data ***
abilities_cached: bool;
unit_types_cached: bool;
upgrades_cached: bool;
buffs_cached: bool;
*/
