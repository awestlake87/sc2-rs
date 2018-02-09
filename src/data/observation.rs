use std::rc::Rc;

use sc2_proto::sc2api;

use super::super::{FromProto, IntoSc2, Result};
use data::{
    Color,
    Effect,
    ImageData,
    Point2,
    Point3,
    PowerSource,
    Rect2,
    Score,
    Unit,
    Upgrade,
    Visibility,
};

/// terrain info
#[derive(Debug, Clone)]
pub struct MapInfo {
    /// width of the terrain
    pub width: i32,
    /// height of the terrain
    pub height: i32,

    /// image that reveals pathable tiles
    pub pathing_grid: ImageData,
    /// image that reveals placable tiles
    pub placement_grid: ImageData,
    /// image that reveals terrain height
    pub terrain_height: ImageData,

    /// rectangle of the playable area
    pub playable_area: Rect2,
    /// starting locations of the enemy bases
    pub enemy_start_locations: Vec<Point2>,
    /* options */
    /* player_info */
}

impl FromProto<sc2api::ResponseGameInfo> for MapInfo {
    fn from_proto(mut info: sc2api::ResponseGameInfo) -> Result<Self> {
        let mut start_raw = info.take_start_raw();

        Ok(Self {
            width: start_raw.get_map_size().get_x(),
            height: start_raw.get_map_size().get_y(),

            pathing_grid: start_raw.take_pathing_grid().into_sc2()?,
            placement_grid: start_raw.take_placement_grid().into_sc2()?,
            terrain_height: start_raw.take_terrain_height().into_sc2()?,

            playable_area: {
                let area = start_raw.get_playable_area();

                Rect2 {
                    from: Point2::new(
                        area.get_p0().get_x() as f32,
                        area.get_p0().get_y() as f32,
                    ),
                    to: Point2::new(
                        area.get_p1().get_x() as f32,
                        area.get_p1().get_y() as f32,
                    ),
                }
            },

            enemy_start_locations: start_raw
                .take_start_locations()
                .into_iter()
                .map(|p| Point2::new(p.get_x() as f32, p.get_y() as f32))
                .collect(),
        })
    }
}

/// state of the game (changes every frame)
#[derive(Debug, Clone)]
pub struct Observation {
    /// the player id associated with the participant
    pub player_id: u32,
    /// the previous game step
    pub previous_step: u32,
    /// the current game step
    pub current_step: u32,
    /// position of the center of the camera
    pub camera_pos: Point2,

    /// a list of all known units at the moment
    pub units: Vec<Rc<Unit>>,

    /// all power sources associated with the current player
    pub power_sources: Vec<PowerSource>,
    /// all active effects in vision of the current player
    pub effects: Vec<Effect>,
    /// all upgrades
    pub upgrades: Vec<Upgrade>,

    /// current mineral count
    pub minerals: u32,
    /// current vespene count
    pub vespene: u32,
    /// the total supply cap given the players max supply
    pub food_cap: u32,
    /// the total supply used by the player
    pub food_used: u32,
    /// the total supply consumed by army units alone
    pub food_army: u32,
    /// the total supply consumed by workers alone
    pub food_workers: u32,
    /// the number of workers that currently have no orders
    pub idle_worker_count: u32,
    /// the number of army units
    pub army_count: u32,
    /// the number of warp gates owned by the player
    pub warp_gate_count: u32,
    /// the number of larva owned by the player
    pub larva_count: u32,

    /// creep image (sample pixels to find tiles with creep)
    pub creep: ImageData,
    /// visibility image (sample pixels to find visible tiles)
    pub visibility: ImageData,

    /// detailed current set of scores
    pub score: Score,
}

impl Observation {
    /// filter all units based on a custom condition
    pub fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
    where
        F: Fn(&Unit) -> bool,
    {
        let mut units = vec![];

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
