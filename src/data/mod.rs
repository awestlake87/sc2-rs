//! Contains useful data exposed through interfaces to the game instance.

mod ability;
mod action;
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

use super::{FromProto, IntoSc2, Result};

pub use self::ability::{Ability, AbilityData};
pub use self::action::{
    DebugAabb,
    DebugCommand,
    DebugLine,
    DebugSphere,
    DebugText,
    DebugTextTarget,
};
pub use self::buff::{Buff, BuffData};
pub use self::game::{GameResult, GameSetup, Map, PlayerResult};
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

/// Information about a player in a replay.
#[derive(Debug, Copy, Clone)]
pub struct ReplayPlayerInfo {
    /// Id of the player.
    player_id: u32,
    /// Player ranking.
    mmr: i32,
    /// Player actions per minute.
    apm: i32,

    /// Actual player race.
    race: Race,
    /// Selected player race (if Random or None, race will be different).
    race_selected: Option<Race>,
    /// If the player won or lost.
    game_result: Option<GameResult>,
}

impl FromProto<sc2api::PlayerInfoExtra> for ReplayPlayerInfo {
    fn from_proto(info: sc2api::PlayerInfoExtra) -> Result<Self> {
        Ok(Self {
            player_id: info.get_player_info().get_player_id(),

            race: info.get_player_info()
                .get_race_actual()
                .into_sc2()?,
            race_selected: {
                if info.get_player_info().has_race_requested() {
                    let proto_race =
                        info.get_player_info().get_race_requested();

                    if proto_race != common::Race::NoRace {
                        Some(proto_race.into_sc2()?)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },

            mmr: info.get_player_mmr(),
            apm: info.get_player_apm(),

            game_result: {
                if info.has_player_result() {
                    Some(info.get_player_result()
                        .get_result()
                        .into_sc2()?)
                } else {
                    None
                }
            },
        })
    }
}

/// Information about a replay file.
#[derive(Debug, Clone)]
pub struct ReplayInfo {
    /// Name of the map.
    map_name: String,
    /// Path to the map.
    map_path: String,
    /// Version of the game.
    game_version: String,
    /// Data version of the game.
    data_version: String,

    /// Duration in seconds.
    duration: f32,
    /// Duration in game steps.
    duration_steps: u32,

    /// Data build of the game.
    data_build: u32,
    /// Required base build of the game.
    base_build: u32,

    /// Information about specific players.
    players: Vec<ReplayPlayerInfo>,
}

impl FromProto<sc2api::ResponseReplayInfo> for ReplayInfo {
    fn from_proto(mut info: sc2api::ResponseReplayInfo) -> Result<Self> {
        Ok(Self {
            map_name: info.take_map_name(),
            map_path: info.take_local_map_path(),
            game_version: info.take_game_version(),
            data_version: info.take_data_version(),

            duration: info.get_game_duration_seconds(),
            duration_steps: info.get_game_duration_loops(),

            data_build: info.get_data_build(),
            base_build: info.get_base_build(),

            players: {
                let mut player_info = vec![];

                for p in info.take_player_info().into_iter() {
                    player_info.push(p.into_sc2()?);
                }

                player_info
            },
        })
    }
}
