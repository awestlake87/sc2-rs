
use super::{ Participant };
use super::super::{ Result, Error };
use super::super::data::{ Ability, Unit, Tag, Point2 };

pub trait Actions {
    fn command_units(&mut self, units: &Vec<Unit>, ability: Ability);
    fn command_units_to_location(
        &mut self, units: &Vec<Unit>, ability: Ability, location: Point2
    );
    fn command_units_to_target(
        &mut self, units: &Vec<Unit>, ability: Ability, target: &Unit
    );
    fn get_commands(&self) -> Vec<Tag>;
    fn send_actions(&self);
    fn toggle_autocast(unit_tags: &Vec<Tag>, ability: Ability);
}

impl Actions for Participant {
    fn command_units(&mut self, _: &Vec<Unit>, _: Ability) {
        unimplemented!("command units");
    }
    fn command_units_to_location(
        &mut self, _: &Vec<Unit>, _: Ability, _: Point2
    ) {
        unimplemented!("command units location");
    }
    fn command_units_to_target(
        &mut self, _: &Vec<Unit>, _: Ability, _: &Unit
    ) {
        unimplemented!("command units to target");
    }

    fn get_commands(&self) -> Vec<Tag> {
        self.commands.clone()
    }

    fn send_actions(&self) {
        unimplemented!("send actions");
    }

    fn toggle_autocast(_: &Vec<Tag>, _: Ability) {
        unimplemented!("toggle autocast")
    }
}
