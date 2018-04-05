use std::rc::Rc;

use sc2_proto::{debug, raw, sc2api};

use super::super::{FromProto, IntoProto, Result};
use data::{Ability, Color, Point2, Point3, Tag, Unit};

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

/// Target for debugging text.
#[derive(Debug, Copy, Clone)]
pub enum DebugTextTarget {
    /// Screen coordinates for debug text.
    Screen(Point2),
    /// World coordinates for debug text.
    World(Point3),
}

/// Debug text.
#[derive(Debug, Clone)]
pub struct DebugText {
    text: String,
    target: Option<DebugTextTarget>,
    color: Color,
}

impl DebugText {
    /// Text to display.
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            target: None,
            color: (0xFF, 0xFF, 0xFF),
        }
    }

    /// Target in screen or world space (default is None).
    ///
    /// If the target is None, then text appears at top-left of screen.
    pub fn target(self, target: DebugTextTarget) -> Self {
        Self {
            target: Some(target),
            ..self
        }
    }

    /// Set the color of the text (default is white).
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color,
            ..self
        }
    }
}

/// A debug line defined by a start and end point.
#[derive(Debug, Copy, Clone)]
pub struct DebugLine {
    /// Point 1 of the line.
    p1: Point3,
    /// Point 2 of the line.
    p2: Point3,
    /// Color of the line.
    color: Color,
}

impl DebugLine {
    /// Create a line from p1 to p2.
    pub fn new(p1: Point3, p2: Point3) -> Self {
        Self {
            p1: p1,
            p2: p2,
            color: (0xFF, 0xFF, 0xFF),
        }
    }

    /// Set the color of the line (default is white).
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color,
            ..self
        }
    }
}

/// A debug axis-aligned bounding box defined by two corners.
#[derive(Debug, Copy, Clone)]
pub struct DebugAabb {
    /// Minimum corner of the box.
    min: Point3,
    /// Maximum corner of the box.
    max: Point3,
    /// Color of the box.
    color: Color,
}

impl DebugAabb {
    /// Create an AABB.
    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: min,
            max: max,
            color: (0xFF, 0xFF, 0xFF),
        }
    }

    /// Set the color of the box (default is white).
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color,
            ..self
        }
    }
}

/// A debug sphere defined by a point in world space and a radius.
#[derive(Debug, Copy, Clone)]
pub struct DebugSphere {
    /// Center of the sphere.
    center: Point3,
    /// Radius of the sphere.
    radius: f32,
    /// Color of the sphere.
    color: Color,
}

impl DebugSphere {
    /// Create a debug sphere.
    pub fn new(center: Point3, radius: f32) -> Self {
        Self {
            center: center,
            radius: radius,
            color: (0xFF, 0xFF, 0xFF),
        }
    }

    /// Set the color of the sphere (default is white).
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color,
            ..self
        }
    }
}

/// A debug command for the game.
#[derive(Debug, Clone)]
pub enum DebugCommand {
    /// Shows debug text in the game instance.
    Text(DebugText),
    /// Shows a debug line in the game from p1 to p2.
    Line(DebugLine),
    /// Shows a debug axis-aligned bounding box in the game.
    Aabb(DebugAabb),
    /// Shows a debug sphere in the game.
    Sphere(DebugSphere),
}

impl From<DebugText> for DebugCommand {
    fn from(text: DebugText) -> Self {
        DebugCommand::Text(text)
    }
}

impl From<DebugLine> for DebugCommand {
    fn from(line: DebugLine) -> Self {
        DebugCommand::Line(line)
    }
}

impl From<DebugAabb> for DebugCommand {
    fn from(aabb: DebugAabb) -> Self {
        DebugCommand::Aabb(aabb)
    }
}

impl From<DebugSphere> for DebugCommand {
    fn from(sphere: DebugSphere) -> Self {
        DebugCommand::Sphere(sphere)
    }
}

impl IntoProto<debug::DebugCommand> for DebugCommand {
    fn into_proto(self) -> Result<debug::DebugCommand> {
        match self {
            DebugCommand::Text(DebugText {
                text,
                target,
                color,
            }) => {
                let mut cmd = debug::DebugCommand::new();
                let mut debug_text = debug::DebugText::new();

                debug_text.set_text(text);

                match target {
                    Some(DebugTextTarget::Screen(p)) => {
                        debug_text.mut_virtual_pos().set_x(p.x);
                        debug_text.mut_virtual_pos().set_y(p.y);
                    },
                    Some(DebugTextTarget::World(p)) => {
                        debug_text.mut_world_pos().set_x(p.x);
                        debug_text.mut_world_pos().set_y(p.y);
                        debug_text.mut_world_pos().set_z(p.z);
                    },
                    None => (),
                }

                debug_text.mut_color().set_r(color.0 as u32);
                debug_text.mut_color().set_g(color.1 as u32);
                debug_text.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_text().push(debug_text);

                Ok(cmd)
            },
            DebugCommand::Line(DebugLine { p1, p2, color }) => {
                let mut cmd = debug::DebugCommand::new();
                let mut debug_line = debug::DebugLine::new();

                debug_line.mut_line().mut_p0().set_x(p1.x);
                debug_line.mut_line().mut_p0().set_y(p1.y);
                debug_line.mut_line().mut_p0().set_z(p1.z);

                debug_line.mut_line().mut_p1().set_x(p2.x);
                debug_line.mut_line().mut_p1().set_y(p2.y);
                debug_line.mut_line().mut_p1().set_z(p2.z);

                debug_line.mut_color().set_r(color.0 as u32);
                debug_line.mut_color().set_g(color.1 as u32);
                debug_line.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_lines().push(debug_line);

                Ok(cmd)
            },
            DebugCommand::Aabb(DebugAabb {
                min,
                max,
                color,
            }) => {
                let mut cmd = debug::DebugCommand::new();
                let mut debug_box = debug::DebugBox::new();

                debug_box.mut_min().set_x(min.x);
                debug_box.mut_min().set_y(min.y);
                debug_box.mut_min().set_z(min.z);

                debug_box.mut_max().set_x(max.x);
                debug_box.mut_max().set_y(max.y);
                debug_box.mut_max().set_z(max.z);

                debug_box.mut_color().set_r(color.0 as u32);
                debug_box.mut_color().set_g(color.1 as u32);
                debug_box.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_boxes().push(debug_box);

                Ok(cmd)
            },
            DebugCommand::Sphere(DebugSphere {
                center,
                radius,
                color,
            }) => {
                let mut cmd = debug::DebugCommand::new();
                let mut debug_sphere = debug::DebugSphere::new();

                debug_sphere.mut_p().set_x(center.x);
                debug_sphere.mut_p().set_y(center.y);
                debug_sphere.mut_p().set_z(center.z);

                debug_sphere.set_r(radius);

                debug_sphere.mut_color().set_r(color.0 as u32);
                debug_sphere.mut_color().set_g(color.1 as u32);
                debug_sphere.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_spheres().push(debug_sphere);

                Ok(cmd)
            },
        }
    }
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
