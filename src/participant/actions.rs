
use super::{ Participant };
use super::super::{ Result, Error };
use super::super::game::{ Unit, Tag, AbilityId };

use na::{ Vector2 };

pub trait Actions {
    fn command_units(&mut self, units: &Vec<Unit>, ability: AbilityId);
    fn command_units_to_location(
        &mut self, units: &Vec<Unit>, ability: AbilityId, location: Vector2<f64>
    );
    fn command_units_to_target(
        &mut self, units: &Vec<Unit>, ability: AbilityId, target: &Unit
    );
    fn get_commands(&self) -> Vec<Tag>;
    fn send_actions(&self);
    fn toggle_autocast(unit_tags: &Vec<Tag>, ability: AbilityId);
}

impl Actions for Participant {
    fn command_units(&mut self, units: &Vec<Unit>, ability: AbilityId) {
        unimplemented!("command units");
    }
    fn command_units_to_location(
        &mut self, units: &Vec<Unit>, ability: AbilityId, location: Vector2<f64>
    ) {
        unimplemented!("command units location");
    }
    fn command_units_to_target(
        &mut self, units: &Vec<Unit>, ability: AbilityId, target: &Unit
    ) {
        unimplemented!("command units to target");
    }

    fn get_commands(&self) -> Vec<Tag> {
        unimplemented!("get commands");
    }

    fn send_actions(&self) {
        unimplemented!("send actions");
    }

    fn toggle_autocast(unit_tags: &Vec<Tag>, ability: AbilityId) {
        unimplemented!("toggle autocast")
    }
}
