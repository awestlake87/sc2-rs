
use std::collections::{ HashMap };
use std::rc::Rc;

use colors::Color;
use data::{
    PowerSource,
    TerrainInfo,
    Unit,
    Upgrade,
    Point2,
    Point3,
    Score,
    UnitType,
    UnitTypeData,
    Effect,
    Ability,
    AbilityData,
    UpgradeData,
    Buff,
    BuffData,
    Visibility,
    ActionTarget
};

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

/// an event from the game
pub enum GameEvent {
    /// a unit was destroyed
    UnitDestroyed(Rc<Unit>),
    /// a unit was created
    UnitCreated(Rc<Unit>),
    /// a unit does not have any orders
    UnitIdle(Rc<Unit>),
    /// a unit was detected
    UnitDetected(Rc<Unit>),

    /// an upgrade completed
    UpgradeCompleted(Upgrade),
    /// a unit finished constructing a building
    BuildingCompleted(Rc<Unit>),

    /// number of nydus worms detected
    NydusWormsDetected(u32),
    /// number of nukes launched
    NukesDetected(u32),
}

/// game data (may vary depending on version and DLC)
pub struct GameData {
    /// data associated with abilities
    pub ability_data:               HashMap<Ability, AbilityData>,
    /// data associated with unit types
    pub unit_type_data:             HashMap<UnitType, UnitTypeData>,
    /// data associated with upgrades
    pub upgrade_data:               HashMap<Upgrade, UpgradeData>,
    /// data associated buffs
    pub buff_data:                  HashMap<Buff, BuffData>,

    /// playable area info
    pub terrain_info:               TerrainInfo,
}

/// state of the game (changes every frame)
pub struct GameState {
    /// the player id associated with the participant
    pub player_id:                  u32,
    /// the previous game step
    pub previous_step:              u32,
    /// the current game step
    pub current_step:               u32,
    /// position of the center of the camera
    pub camera_pos:                 Point2,

    /// a list of all known units at the moment
    pub units:                      Vec<Rc<Unit>>,

    /// all power sources associated with the current player
    pub power_sources:              Vec<PowerSource>,
    /// all active effects in vision of the current player
    pub effects:                    Vec<Effect>,
    /// all upgrades
    pub upgrades:                   Vec<Upgrade>,

    /// current mineral count
    pub minerals:                   u32,
    /// current vespene count
    pub vespene:                    u32,
    /// the total supply cap given the players max supply
    pub food_cap:                   u32,
    /// the total supply used by the player
    pub food_used:                  u32,
    /// the total supply consumed by army units alone
    pub food_army:                  u32,
    /// the total supply consumed by workers alone
    pub food_workers:               u32,
    /// the number of workers that currently have no orders
    pub idle_worker_count:          u32,
    /// the number of army units
    pub army_count:                 u32,
    /// the number of warp gates owned by the player
    pub warp_gate_count:            u32,
    /// the number of larva owned by the player
    pub larva_count:                u32,

    /// detailed current set of scores
    pub score:                      Score
}

impl GameState {
    pub fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
        where F: Fn(&Unit) -> bool
    {
        let mut units = vec![ ];

        for unit in &self.units {
            if filter(&unit) {
                units.push(Rc::clone(&unit));
            }
        }

        units
    }
    /// check if the given point contains creep
    pub fn has_creep(&self, _: Point2) -> bool {
        unimplemented!("has creep")
    }
    /// get the visibility of the given point for the current player
    pub fn get_visibility(&self, _: Point2) -> Visibility {
        unimplemented!("get visibility")
    }
    /// whether the given point on the terrain is pathable
    ///
    /// this does not include pathing blockers like structures, for more
    /// accurate pathing results, use query interface
    pub fn is_pathable(&self, _: Point2) -> bool {
        unimplemented!("is pathable")
    }
    /// whether the given point on the terrain is buildable
    ///
    /// this does not include blockers like other structures. for more
    /// accurate building placement results, use query interface
    pub fn is_placable(&self, _: Point2) -> bool {
        unimplemented!("is placable")
    }
    /// returns the terrain height of the given point
    pub fn get_terrain_height(&self, _: Point2) -> f32 {
        unimplemented!("get terrain height")
    }
}

/// all game data passed to agents and observers
pub struct FrameData {
    /// state that updates every frame
    pub state: GameState,
    /// data that can change on a per game basis
    pub data: Rc<GameData>,
    /// events that have happened since the last update
    pub events: Vec<GameEvent>
}
