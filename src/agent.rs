
use std::rc::Rc;

use super::{ Result };
use participant::{ GameState, GameData, GameEvent, Participant, FrameData };
use colors::Color;
use data::{ Unit, Upgrade, Point2, Point3, Ability, Action, ActionTarget };

/// target for debugging text
#[derive(Debug, Copy, Clone)]
pub enum DebugTextTarget {
    /// screen coordinates for debug text
    Screen(Point2),
    /// world coordinates for debug text
    World(Point3)
}

#[derive(Debug, Clone)]
pub enum Command {
    Action {
        units: Vec<Rc<Unit>>,
        ability: Ability,
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
    fn start(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }

    fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        Ok(vec![ ])
    }

    fn end(&mut self, frame: FrameData) -> Result<()> {
        Ok(())
    }
}
