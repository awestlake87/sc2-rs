
use sc2_proto::sc2api;
use sc2_proto::debug;

use super::super::{ Result };
use super::super::colors::{ Color };
use super::super::data::{ Point2, Point3 };
use super::{ Participant };

/// target for debugging text
pub enum DebugTextTarget {
    /// screen coordinates for debug text
    Screen(Point2),
    /// world coordinates for debug text
    World(Point3)
}

/// debugging command
pub enum DebugCommand {
    /// shows debug text in the game instance
    DrawText {
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
    DrawLine {
        /// starting point of the line
        p1: Point3,
        /// ending point of the line
        p2: Point3,
        /// color of the line
        color: Color,
    },

    /// shows a debug box in the game defined by corners min and max
    DrawBox {
        /// minimum corner of the box
        min: Point3,
        /// maximum corner of the box
        max: Point3,
        /// color of the box
        color: Color,
    },

    /// shows a debug sphere in the game
    DrawSphere {
        /// center of the sphere
        center: Point3,
        /// radius of the sphere
        radius: f32,
        /// color of the sphere
        color: Color,
    }
}

impl DebugCommand {
    /// convert into a debug command
    pub fn into_proto(self) -> debug::DebugCommand {
        let mut cmd = debug::DebugCommand::new();

        match self {
            DebugCommand::DrawText { text, target, color } => {
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
                    None => ()
                }

                debug_text.mut_color().set_r(color.0 as u32);
                debug_text.mut_color().set_g(color.1 as u32);
                debug_text.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_text().push(debug_text);
            },
            DebugCommand::DrawLine { p1, p2, color } => {
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
            },
            DebugCommand::DrawBox { min, max, color } => {
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
            }
            DebugCommand::DrawSphere { center, radius, color } => {
                let mut debug_sphere = debug::DebugSphere::new();

                debug_sphere.mut_p().set_x(center.x);
                debug_sphere.mut_p().set_y(center.y);
                debug_sphere.mut_p().set_z(center.z);

                debug_sphere.set_r(radius);

                debug_sphere.mut_color().set_r(color.0 as u32);
                debug_sphere.mut_color().set_g(color.1 as u32);
                debug_sphere.mut_color().set_b(color.2 as u32);

                cmd.mut_draw().mut_spheres().push(debug_sphere);
            }
        }

        cmd
    }
}

/// debugging trait
pub trait Debugging {
    /// queue a debugging command
    fn command_debug(&mut self, cmd: DebugCommand);

    /// send all queued debug commands
    fn send_debug_commands(&mut self) -> Result<()>;
}

impl Debugging for Participant {
    fn command_debug(&mut self, cmd: DebugCommand) {
        self.debug_commands.push(cmd);
    }

    fn send_debug_commands(&mut self) -> Result<()> {
        if !self.debug_commands.is_empty() {
            let mut req = sc2api::Request::new();

            for cmd in self.debug_commands.drain(..) {
                req.mut_debug().mut_debug().push(cmd.into_proto());
            }

            self.debug_commands.clear();

            self.send(req)?;
            self.recv()?;
        }

        Ok(())
    }
}
