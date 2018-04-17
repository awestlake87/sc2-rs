//! Contains useful data exposed through interfaces to the game instance.

mod ability;
mod buff;
mod game;
mod image;
mod map_info;
mod player;
mod score;
mod unit;
mod upgrade;

use na;
use na::geometry;

use sc2_proto::{common, raw, sc2api};

use {FromProto, IntoSc2, Result};

pub use self::ability::{Ability, AbilityData};
pub use self::buff::{Buff, BuffData};
pub use self::game::{GameResult, Map, PlayerResult};
pub use self::image::ImageData;
pub use self::map_info::MapInfo;
pub use self::player::{Difficulty, PlayerSetup, Race};
pub use self::score::Score;
pub use self::unit::{
    Alliance,
    DisplayType,
    Tag,
    Unit,
    UnitType,
    UnitTypeData,
};
pub use self::upgrade::{Upgrade, UpgradeData};

/// Color type for debug commands.
pub type Color = (u8, u8, u8);

/// Generic structure to represent a 2D rectangle.
#[derive(Debug, Copy, Clone)]
pub struct Rect<T> {
    /// X position of lefthand corner.
    pub x: T,
    /// Y position of lefthand corner.
    pub y: T,
    /// Width of the rectangle.
    pub w: T,
    /// Height of the rectangle.
    pub h: T,
}

/// 2D vector used to specify direction.
pub type Vector2 = na::Vector2<f32>;
/// 3D vector used to specify direction.
pub type Vector3 = na::Vector3<f32>;
/// 2D point used to specify location.
pub type Point2 = geometry::Point2<f32>;
/// 3D point used to specify location.
pub type Point3 = geometry::Point3<f32>;

/// 2D rectangle represented by two points.
#[derive(Debug, Copy, Clone)]
pub struct Rect2 {
    /// Upper left-hand corner.
    pub from: Point2,
    /// Lower right-hand corner.
    pub to: Point2,
}

impl Rect2 {
    /// Returns the width and height of the rectangle.
    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.to.x - self.from.x, self.to.y - self.from.y)
    }
}

/// 2D integer point used to specify a location.
pub type Point2I = na::Vector2<i32>;

/// 2D integer rectangle represented by two points.
#[derive(Debug, Copy, Clone)]
pub struct Rect2I {
    /// Upper left-hand corner.
    pub from: Point2I,
    /// Lower right-hand corner.
    pub to: Point2I,
}

/// Visibility of a point on the terrain.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Hidden,
    Fogged,
    Visible,
    FullHidden,
}

/// Effect data.
#[derive(Debug, Clone)]
pub struct EffectData {
    /// Stable effect ID.
    effect: Ability,
    /// Effect name (corresponds to game's catalog).
    name: String,
    /// A more recognizable name of the effect.
    friendly_name: String,
    /// Size of the circle the effect impacts.
    radius: f32,
}

/// Visuals of a persistent ability on the map (eg. PsiStorm).
#[derive(Debug, Clone)]
pub struct Effect {
    /// Stable effect ID.
    effect: Ability,
    /// All the positions that this effect is impacting on the map.
    positions: Vec<Point2>,
}

/// Power source information for Protoss.
#[derive(Debug, Copy, Clone)]
pub struct PowerSource {
    /// Unit tag of the power source.
    tag: Tag,
    /// Position of the power source.
    pos: Point2,
    /// Radius of the power source.
    radius: f32,
}

impl From<raw::PowerSource> for PowerSource {
    fn from(source: raw::PowerSource) -> Self {
        Self {
            tag: source.get_tag(),
            pos: {
                let pos = source.get_pos();
                Point2::new(pos.get_x(), pos.get_y())
            },
            radius: source.get_radius(),
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
