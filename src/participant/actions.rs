
use std::rc::Rc;

use sc2_proto::sc2api;

use super::{ Result };
use data::{ Ability, Action, ActionTarget, Unit };
use participant::{ Participant };

/// UNSTABLE trait for sending actions to the game instance
pub trait Actions {
    /// issue a command to the given units
    fn command_units(
        &mut self, units: &Vec<Rc<Unit>>,
        ability: Ability,
        target: Option<ActionTarget>
    );

    /// send the requested actions to the game instance
    fn send_actions(&mut self) -> Result<()>;

    /// toggles the autocast of an ability on a list of units
    fn toggle_autocast(&mut self, unit_tags: &Vec<Rc<Unit>>, ability: Ability)
        -> Result<()>
    ;
}

impl Actions for Participant {
    fn command_units(
        &mut self,
        units: &Vec<Rc<Unit>>,
        ability: Ability,
        target: Option<ActionTarget>
    ) {
        self.requested_actions.push(
            Action {
                ability: ability,
                unit_tags: units.iter().map(|u| u.tag).collect(),
                target: target,
            }
        );
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
                        Some(ActionTarget::Location(pos)) => {
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
            self.recv()?;
        }

        Ok(())
    }

    fn toggle_autocast(&mut self, _: &Vec<Rc<Unit>>, _: Ability)
        -> Result<()>
    {
        unimplemented!("toggle autocast")
    }
}
