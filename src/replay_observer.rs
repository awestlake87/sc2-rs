
use agent::Agent;
use data::{ ReplayInfo };

pub trait ReplayObserver: Agent {
    fn should_ignore(&self, _: &ReplayInfo, _: u32) -> bool {
        false
    }
}
