
use std::rc::Rc;

use super::{ Result };
use participant::{ FrameData };
use colors::Color;
use data::{ Unit, Point2, Point3, Ability, ActionTarget };

/// target for debugging text
#[derive(Debug, Copy, Clone)]
pub enum DebugTextTarget {
    /// screen coordinates for debug text
    Screen(Point2),
    /// world coordinates for debug text
    World(Point3)
}

/// a command to issue to the game instance
#[derive(Debug, Clone)]
pub enum Command {
    /// command a set of units
    Action {
        /// units to command
        units: Vec<Rc<Unit>>,
        /// ability to trigger
        ability: Ability,
        /// ability target
        target: Option<ActionTarget>
    },
    // ToggleAutocast {
    //     units: Vec<Rc<Unit>>,
    //     ability: Ability
    // },

    /// shows debug text in the game instance
    DebugText {
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
    DebugLine {
        /// starting point of the line
        p1: Point3,
        /// ending point of the line
        p2: Point3,
        /// color of the line
        color: Color,
    },

    /// shows a debug box in the game defined by corners min and max
    DebugBox {
        /// minimum corner of the box
        min: Point3,
        /// maximum corner of the box
        max: Point3,
        /// color of the box
        color: Color,
    },

    /// shows a debug sphere in the game
    DebugSphere {
        /// center of the sphere
        center: Point3,
        /// radius of the sphere
        radius: f32,
        /// color of the sphere
        color: Color,
    }
}

/// trait for all entities that can handle game events
pub trait Agent {
    /// called at the beginning of a match with the inital frame data
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }
    /// called throughout the game to step the bot
    fn update(&mut self, _: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }
    /// called at the end of a match with the final frame data
    fn end(&mut self, _: FrameData) -> Result<()> {
        Ok(())
    }
}
