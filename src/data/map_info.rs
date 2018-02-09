use sc2_proto::sc2api;

use super::super::{FromProto, IntoSc2, Result};
use data::{ImageData, Point2, Rect2};

/// terrain info
#[derive(Debug, Clone)]
pub struct MapInfo {
    dimensions: (u32, u32),

    pathing_grid: ImageData,
    placement_grid: ImageData,
    terrain_height: ImageData,

    playable_area: Rect2,
    enemy_start_locations: Vec<Point2>,
}

impl MapInfo {
    /// dimensions of the map
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    /// image that reveals pathable tiles
    pub fn get_pathing_grid(&self) -> &ImageData {
        &self.pathing_grid
    }
    /// image that reveals placable tiles
    pub fn get_placement_grid(&self) -> &ImageData {
        &self.placement_grid
    }
    /// image that reveals terrain height
    pub fn get_terrain_height(&self) -> &ImageData {
        &self.terrain_height
    }

    /// rectangle of the playable area
    pub fn get_playable_area(&self) -> Rect2 {
        self.playable_area
    }
    /// starting locations of the enemy bases
    pub fn get_enemy_start_locations(&self) -> &[Point2] {
        &self.enemy_start_locations
    }
}

impl FromProto<sc2api::ResponseGameInfo> for MapInfo {
    fn from_proto(mut info: sc2api::ResponseGameInfo) -> Result<Self> {
        let mut start_raw = info.take_start_raw();

        Ok(Self {
            dimensions: (
                start_raw.get_map_size().get_x() as u32,
                start_raw.get_map_size().get_y() as u32,
            ),

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
