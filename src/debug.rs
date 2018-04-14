//! Contains the public API of the SC2 Debug commands.

use sc2_proto::debug;

use data::{Color, Point2, Point3};
use {IntoProto, Result};

pub use services::action_service::DebugClient;

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
