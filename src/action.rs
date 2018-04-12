//! Contains the public API for all structures dealing directly with the SC2
//! Actions. These are used to give orders to units and perform upgrades in the
//! game.

use std::rc::Rc;

use sc2_proto::{raw, sc2api};

use data::{Ability, Point2, Tag, Unit};
use {FromProto, IntoProto, Result};

pub use services::action_service::ActionClient;

/// Action target.
#[derive(Debug, Copy, Clone)]
pub enum ActionTarget {
    /// Target a unit with this action.
    Unit(Tag),
    /// Target a location with this action.
    Location(Point2),
}

/// An action (command or ability) applied to a unit or set of units.
#[derive(Debug, Clone)]
pub struct Action {
    /// The ability to invoke.
    ability: Ability,
    units: Vec<Tag>,
    target: Option<ActionTarget>,
}

impl Action {
    /// Perform the given ability.
    pub fn new(ability: Ability) -> Self {
        Self {
            ability: ability,
            units: vec![],
            target: None,
        }
    }

    /// Units that this action applies to.
    ///
    /// Take the tags from an arbitrary iterator of units.
    pub fn units<'a, T>(self, units: T) -> Self
    where
        T: Iterator<Item = &'a Rc<Unit>>,
    {
        Self {
            units: units.map(|u| u.get_tag()).collect(),
            ..self
        }
    }

    /// Tnits that this action applies to.
    ///
    /// Directly assign the unit tags.
    pub fn unit_tags(self, units: Vec<Tag>) -> Self {
        Self {
            units: units,
            ..self
        }
    }

    /// Set the target of the action.
    pub fn target(self, target: ActionTarget) -> Self {
        Self {
            target: Some(target),
            ..self
        }
    }
}

impl FromProto<raw::ActionRawUnitCommand> for Action {
    fn from_proto(action: raw::ActionRawUnitCommand) -> Result<Self> {
        Ok(Self {
            ability: Ability::from_proto(action.get_ability_id() as u32)?,
            units: {
                let mut unit_tags = vec![];

                for tag in action.get_unit_tags() {
                    unit_tags.push(*tag);
                }

                unit_tags
            },
            target: {
                if action.has_target_unit_tag() {
                    Some(ActionTarget::Unit(action.get_target_unit_tag()))
                } else if action.has_target_world_space_pos() {
                    let pos = action.get_target_world_space_pos();
                    Some(ActionTarget::Location(Point2::new(
                        pos.get_x(),
                        pos.get_y(),
                    )))
                } else {
                    None
                }
            },
        })
    }
}

impl IntoProto<sc2api::Action> for Action {
    fn into_proto(self) -> Result<sc2api::Action> {
        let mut action = sc2api::Action::new();
        {
            let cmd = action.mut_action_raw().mut_unit_command();

            cmd.set_ability_id(self.ability.into_proto()? as i32);

            match self.target {
                Some(ActionTarget::Unit(tag)) => {
                    cmd.set_target_unit_tag(tag);
                },
                Some(ActionTarget::Location(pos)) => {
                    let target = cmd.mut_target_world_space_pos();
                    target.set_x(pos.x);
                    target.set_y(pos.y);
                },
                None => (),
            }

            for tag in self.units {
                cmd.mut_unit_tags().push(tag);
            }
        }

        Ok(action)
    }
}
