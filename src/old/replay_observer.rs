
use agent::Agent;
use data::{ ReplayInfo };

/// trait for all replay observers
pub trait ReplayObserver: Agent {
    /// check if this observer wants to observe the specified replay
    fn should_ignore(&self, _: &ReplayInfo, _: u32) -> bool {
        false
    }
}
