use std::rc::Rc;

use sc2_proto::{raw, sc2api};
// use sc2_proto::spatial::{
//     ActionSpatialCameraMove,
//     ActionSpatialUnitCommand,
//     ActionSpatialUnitSelectionPoint,
//     ActionSpatialUnitSelectionPoint_Type as ProtoPointSelectionType,
//     ActionSpatialUnitSelectionRect,
// };

use data::{Ability, Point2, Point3, Tag, Unit, Color};
use super::super::{FromProto, IntoProto, Result};

/// action target
#[derive(Debug, Copy, Clone)]
pub enum ActionTarget {
    /// target a unit with this action
    Unit(Tag),
    /// target a location with this action
    Location(Point2),
}

/// an action (command or ability) applied to a unit or set of units
#[derive(Debug, Clone)]
pub struct Action {
    /// the ability to invoke
    ability: Ability,
    units: Vec<Tag>,
    target: Option<ActionTarget>,
}

impl Action {
    /// perform the given ability
    pub fn new(ability: Ability) -> Self {
        Self {
            ability: ability,
            units: vec![],
            target: None,
        }
    }

    /// units that this action applies to
    ///
    /// take the tags from an arbitrary iterator of units
    pub fn units<'a, T>(self, units: T) -> Self
    where
        T: Iterator<Item = &'a Rc<Unit>>,
    {
        Self {
            units: units.map(|u| u.tag).collect(),
            ..self
        }
    }

    /// units that this action applies to
    ///
    /// directly assign the unit tags
    pub fn unit_tags(self, units: Vec<Tag>) -> Self {
        Self {
            units: units,
            ..self
        }
    }

    /// set the target of the action
    pub fn target(self, target: ActionTarget) -> Self {
        Self {
            target: Some(target),
            ..self
        }
    }
}

impl FromProto<raw::ActionRawUnitCommand> for Action {
    /// convert from protobuf data
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

/// target for debugging text
#[derive(Debug, Copy, Clone)]
pub enum DebugTextTarget {
    /// screen coordinates for debug text
    Screen(Point2),
    /// world coordinates for debug text
    World(Point3),
}

/// a debug command for the game
#[derive(Debug, Clone)]
pub enum DebugCommand {
    /// shows debug text in the game instance
    Text {
        /// text to display
        text: String,
        /// target in screen or world space
        ///
        /// if the target is None, then text appears at top-left of screen.
        target: Option<DebugTextTarget>,
        /// color of the text
        color: Color,
    },

    /// shows a debug line in the game from p1 to p2
    Line {
        /// starting point of the line
        p1: Point3,
        /// ending point of the line
        p2: Point3,
        /// color of the line
        color: Color,
    },

    /// shows a debug box in the game defined by corners min and max
    Box {
        /// minimum corner of the box
        min: Point3,
        /// maximum corner of the box
        max: Point3,
        /// color of the box
        color: Color,
    },

    /// shows a debug sphere in the game
    Sphere {
        /// center of the sphere
        center: Point3,
        /// radius of the sphere
        radius: f32,
        /// color of the sphere
        color: Color,
    },
}

// /// target of a feature layer command
// #[derive(Debug, Copy, Clone)]
// pub enum SpatialUnitCommandTarget {
//     /// screen coordinate target
//     Screen(Point2I),
//     /// minimap coordinate target
//     Minimap(Point2I),
// }

// /// type of point selection
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum PointSelectType {
//     /// changes selection to unit (equal to normal click)
//     Select,
//     /// toggle selection of unit (equal to shift+click)
//     Toggle,
//     /// select all units of a given type (equal to ctrl+click)
//     All,
//     /// select all units of a given type additively (equal to
//     /// shift+ctrl+click)
//     AddAll,
// }

// impl FromProto<ProtoPointSelectionType> for PointSelectType {
//     fn from_proto(select_type: ProtoPointSelectionType) -> Result<Self> {
//         Ok(match select_type {
//             ProtoPointSelectionType::Select => PointSelectType::Select,
//             ProtoPointSelectionType::Toggle => PointSelectType::Toggle,
//             ProtoPointSelectionType::AllType => PointSelectType::All,
//             ProtoPointSelectionType::AddAllType => PointSelectType::AddAll,
//         })
//     }
// }

// /// feature layer action
// #[derive(Debug, Clone)]
// pub enum SpatialAction {
//     /// issue a feature layer unit command
//     UnitCommand {
//         /// ability to invoke
//         ability: Ability,
//         /// target of command
//         target: Option<SpatialUnitCommandTarget>,
//         /// whether this action should replace or queue behind other
//         /// actions
//         queued: bool,
//     },
//     /// move the camera to a new location
//     CameraMove {
//         /// minimap location
//         center_minimap: Point2I,
//     },
//     /// select a point on the screen
//     SelectPoint {
//         /// point in screen coordinates
//         select_screen: Point2I,
//         /// point selection type
//         select_type: PointSelectType,
//     },
//     /// select a rectangle on the screen
//     SelectRect {
//         /// rectangle in screen coordinates
//         select_screen: Vec<Rect2I>,
//         /// whether selection is additive
//         select_add: bool,
//     },
// }

// impl FromProto<ActionSpatialUnitCommand> for SpatialAction {
//     fn from_proto(cmd: ActionSpatialUnitCommand) -> Result<Self> {
//         Ok(SpatialAction::UnitCommand {
//             ability: Ability::from_proto(cmd.get_ability_id() as u32)?,
//             queued: cmd.get_queue_command(),
//             target: {
//                 if cmd.has_target_screen_coord() {
//                     let pos = cmd.get_target_screen_coord();
//                     Some(SpatialUnitCommandTarget::Screen(Point2I::new(
//                         pos.get_x(),
//                         pos.get_y(),
//                     )))
//                 } else if cmd.has_target_minimap_coord() {
//                     let pos = cmd.get_target_minimap_coord();
//                     Some(SpatialUnitCommandTarget::Minimap(Point2I::new(
//                         pos.get_x(),
//                         pos.get_y(),
//                     )))
//                 } else {
//                     None
//                 }
//             },
//         })
//     }
// }

// impl FromProto<ActionSpatialCameraMove> for SpatialAction {
//     fn from_proto(cmd: ActionSpatialCameraMove) -> Result<Self> {
//         Ok(SpatialAction::CameraMove {
//             center_minimap: {
//                 let pos = cmd.get_center_minimap();
//                 Point2I::new(pos.get_x(), pos.get_y())
//             },
//         })
//     }
// }

// impl FromProto<ActionSpatialUnitSelectionPoint> for SpatialAction {
//     fn from_proto(cmd: ActionSpatialUnitSelectionPoint) -> Result<Self> {
//         Ok(SpatialAction::SelectPoint {
//             select_screen: {
//                 let pos = cmd.get_selection_screen_coord();
//                 Point2I::new(pos.get_x(), pos.get_y())
//             },
//             select_type: cmd.get_field_type().into_sc2()?,
//         })
//     }
// }

// impl FromProto<ActionSpatialUnitSelectionRect> for SpatialAction {
//     fn from_proto(cmd: ActionSpatialUnitSelectionRect) -> Result<Self> {
//         Ok(SpatialAction::SelectRect {
//             select_screen: {
//                 let mut rects = vec![];

//                 for r in cmd.get_selection_screen_coord() {
//                     rects.push(Rect2I {
//                         from: {
//                             let p = r.get_p0();
//                             Point2I::new(p.get_x(), p.get_y())
//                         },
//                         to: {
//                             let p = r.get_p1();
//                             Point2I::new(p.get_x(), p.get_y())
//                         },
//                     })
//                 }

//                 rects
//             },
//             select_add: cmd.get_selection_add(),
//         })
//     }
// }
