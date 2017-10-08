
mod control;

use super::agent::Agent;
use super::client::Client;
use super::instance::Instance;
use super::player::Player;

pub use self::control::{ Control };

pub struct Participant {
    pub player:                 Player,
    pub instance:               Instance,
    pub client:                 Client,
    pub agent:                  Box<Agent>,
    //observer
    //control
}
