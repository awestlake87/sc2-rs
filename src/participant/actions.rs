
use std::rc::Rc;

use sc2_proto::sc2api;

use super::{ Participant };
use super::super::{ Result };
use super::super::data::{ Ability, Action, ActionTarget, Unit, Tag, Point2 };

pub trait Actions {
    fn command_units(&mut self, units: &Vec<Rc<Unit>>, ability: Ability);
    fn command_units_to_location(
        &mut self, units: &Vec<Rc<Unit>>, ability: Ability, location: Point2
    );
    fn command_units_to_target(
        &mut self, units: &Vec<Rc<Unit>>, ability: Ability, target: &Unit
    );
    fn get_commands(&self) -> &Vec<Tag>;
    fn send_actions(&mut self) -> Result<()>;
    fn toggle_autocast(&mut self, unit_tags: &Vec<Tag>, ability: Ability);
}

impl Actions for Participant {
    fn command_units(&mut self, units: &Vec<Rc<Unit>>, ability: Ability) {
        self.requested_actions.push(
            Action {
                ability: ability,
                unit_tags: units.iter().map(|u| u.tag).collect(),
                target: None,
            }
        );
    }
    fn command_units_to_location(
        &mut self, units: &Vec<Rc<Unit>>, ability: Ability, location: Point2
    ) {
        self.requested_actions.push(
            Action {
                ability: ability,
                unit_tags: units.iter().map(|u| u.tag).collect(),
                target: Some(ActionTarget::Position(location)),
            }
        );
    }
    fn command_units_to_target(
        &mut self, units: &Vec<Rc<Unit>>, ability: Ability, target: &Unit
    ) {
        self.requested_actions.push(
            Action {
                ability: ability,
                unit_tags: units.iter().map(|u| u.tag).collect(),
                target: Some(ActionTarget::UnitTag(target.tag)),
            }
        )
    }

    fn get_commands(&self) -> &Vec<Tag> {
        &self.commands
    }

    fn send_actions(&mut self) -> Result<()> {
        self.commands.clear();

        let mut req = sc2api::Request::new();

        {
            let req_actions = req.mut_action().mut_actions();

            for action in &self.requested_actions {
                let mut a = sc2api::Action::new();
                {
                    let cmd = a.mut_action_raw().mut_unit_command();

                    cmd.set_ability_id(action.ability.as_id() as i32);

                    match action.target {
                        Some(ActionTarget::UnitTag(tag)) => {
                            cmd.set_target_unit_tag(tag);
                        },
                        Some(ActionTarget::Position(pos)) => {
                            let target = cmd.mut_target_world_space_pos();
                            target.set_x(pos.x);
                            target.set_y(pos.y);
                        },
                        None => ()
                    };

                    let unit_tags = cmd.mut_unit_tags();

                    for tag in &action.unit_tags {
                        unit_tags.push(*tag);
                    }
                }

                req_actions.push(a);
            }

            for action in req_actions.iter() {
                if action.has_action_raw() {
                    let raw = action.get_action_raw();

                    if raw.has_unit_command() {
                        for tag in raw.get_unit_command().get_unit_tags() {
                            self.commands.push(*tag);
                        }
                    }
                }
            }
        }

        if !self.requested_actions.is_empty() {
            self.requested_actions.clear();
            self.send(req)?;
            let rsp = self.recv()?;
            println!("received {:#?}", rsp);
        }

        Ok(())
    }

    fn toggle_autocast(&mut self, _: &Vec<Tag>, _: Ability) {
        unimplemented!("toggle autocast")
    }
}
