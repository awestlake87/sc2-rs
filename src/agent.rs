
use std::rc::Rc;

use super::{ Result };
use participant::Participant;
use data::{ Unit, Upgrade };

/// trait for all entities that can handle game events
pub trait Agent {
    /// called when a game is started after a load
    ///
    /// fast restarting will not trigger this event
    fn on_game_full_start(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }

    /// called whenever a game is started or restarted
    fn on_game_start(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }

    /// called whenever a game has ended
    fn on_game_end(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }

    /// called whenever the agent needs to be stepped
    ///
    /// in non-realtime games, this is called at a regular interval. in
    /// realtime games, it is called as often as possible
    fn on_step(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }

    /// called whenever one of the player's units has been destroyed
    fn on_unit_destroyed(&mut self, _: &mut Participant, _: &Rc<Unit>)
        -> Result<()>
    {
        Ok(())
    }

    /// called when a unit has been created by the player
    fn on_unit_created(&mut self, _: &mut Participant, _: &Rc<Unit>)
        -> Result<()>
    {
        Ok(())
    }

    /// called whenever a unit becomes idle
    ///
    /// this will only occur as an event, so will only be called when the unit
    /// becomes idle, not continuously for each idle unit. being idle is
    /// defined as having orders in the previous step and not currently having
    /// orders. units that were just created will also trigger the on_unit_idle
    /// event
    fn on_unit_idle(&mut self, _: &mut Participant, _: &Rc<Unit>)
        -> Result<()>
    {
        Ok(())
    }

    /// called when an upgrade is finished
    fn on_upgrade_complete(&mut self, _: &mut Participant, _: Upgrade)
        -> Result<()>
    {
        Ok(())
    }

    /// called when a unit has finished building a structure
    ///
    /// called when the unit in the previous step had a build progress less
    /// than 1.0, but is greater than or equal to 1.0 now
    fn on_building_complete(&mut self, _: &mut Participant, _: &Rc<Unit>)
        -> Result<()>
    {
        Ok(())
    }

    /// called when a nydus is placed
    fn on_nydus_detected(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }
    /// called when a nuclear launch is detected
    fn on_nuke_detected(&mut self, _: &mut Participant) -> Result<()> {
        Ok(())
    }
    /// called when the enemy unit enters vision from out of fog of war
    fn on_unit_detected(&mut self, _: &mut Participant, _: &Rc<Unit>)
        -> Result<()>
    {
        Ok(())
    }
}
