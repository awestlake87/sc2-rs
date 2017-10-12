
use super::{ Participant };
use super::super::{ Result, Error };
use super::super::ability::{ Ability };
use super::super::unit::{ Unit, Tag };

use na::{ Vector2 };

pub trait Actions {
    fn command_units(&mut self, units: &Vec<Unit>, ability: Ability);
    fn command_units_to_location(
        &mut self, units: &Vec<Unit>, ability: Ability, location: Vector2<f32>
    );
    fn command_units_to_target(
        &mut self, units: &Vec<Unit>, ability: Ability, target: &Unit
    );
    fn get_commands(&self) -> Vec<Tag>;
    fn send_actions(&self);
    fn toggle_autocast(unit_tags: &Vec<Tag>, ability: Ability);
}

impl Actions for Participant {
    fn command_units(&mut self, units: &Vec<Unit>, ability: Ability) {
        unimplemented!("command units");
    }
    fn command_units_to_location(
        &mut self, units: &Vec<Unit>, ability: Ability, location: Vector2<f32>
    ) {
        unimplemented!("command units location");
    }
    fn command_units_to_target(
        &mut self, units: &Vec<Unit>, ability: Ability, target: &Unit
    ) {
        unimplemented!("command units to target");
    }

    fn get_commands(&self) -> Vec<Tag> {
        unimplemented!("get commands");
    }

    fn send_actions(&self) {
        unimplemented!("send actions");
    }

    fn toggle_autocast(unit_tags: &Vec<Tag>, ability: Ability) {
        unimplemented!("toggle autocast")
    }
}
