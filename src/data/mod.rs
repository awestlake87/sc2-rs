//! contains useful data exposed through interfaces to the game instance

mod ability;
mod action;
mod buff;
mod game;
mod image;
mod player;
mod score;
mod unit;
mod upgrade;
mod observation;

use na;
use na::geometry;

use sc2_proto::{common, raw, sc2api};

use super::{FromProto, IntoSc2, Result};

pub use self::ability::{Ability, AbilityData};
pub use self::action::{Action, ActionTarget, DebugCommand, DebugTextTarget};
pub use self::buff::{Buff, BuffData};
pub use self::game::{GameResult, GameSettings, Map, PlayerResult};
pub use self::image::ImageData;
pub use self::observation::{MapInfo, Observation};
pub use self::player::{Difficulty, PlayerSetup, Race};
pub use self::score::Score;
pub use self::unit::{Alliance, DisplayType, Tag, Unit, UnitType, UnitTypeData};
pub use self::upgrade::{Upgrade, UpgradeData};

/// color type for debug commands
pub type Color = (u8, u8, u8);

/// generic structure to represent a 2D rectangle
#[derive(Debug, Copy, Clone)]
pub struct Rect<T> {
    /// x position of lefthand corner
    pub x: T,
    /// y position of lefthand corner
    pub y: T,
    /// width of the rectangle
    pub w: T,
    /// height of the rectangle
    pub h: T,
}

/// 2D vector used to specify direction
pub type Vector2 = na::Vector2<f32>;
/// 3D vector used to specify direction
pub type Vector3 = na::Vector3<f32>;
/// 2D point used to specify location
pub type Point2 = geometry::Point2<f32>;
/// 3D point used to specify location
pub type Point3 = geometry::Point3<f32>;

/// 2D rectangle represented by two points
#[derive(Debug, Copy, Clone)]
pub struct Rect2 {
    /// upper left-hand corner
    pub from: Point2,
    /// lower right-hand corner
    pub to: Point2,
}

impl Rect2 {
    /// returns the width and height of the rectangle
    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.to.x - self.from.x, self.to.y - self.from.y)
    }
}

/// 2D integer point used to specify a location
pub type Point2I = na::Vector2<i32>;
/// 3D integer point used to specify a location
//pub type Point3I = na::Vector3<i32>;

/// 2D integer rectangle represented by two points
#[derive(Debug, Copy, Clone)]
pub struct Rect2I {
    /// upper left-hand corner
    pub from: Point2I,
    /// lower right-hand corner
    pub to: Point2I,
}

/// visibility of a point on the terrain
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Hidden,
    Fogged,
    Visible,
    FullHidden,
}

/// effect data
#[derive(Debug, Clone)]
pub struct EffectData {
    /// stable effect ID
    pub effect: Ability,
    /// effect name (corresponds to game's catalog)
    pub name: String,
    /// a more recognizable name of the effect
    pub friendly_name: String,
    /// size of the circle the effect impacts
    pub radius: f32,
}

/// visuals of a persistent ability on the map (eg. PsiStorm)
#[derive(Debug, Clone)]
pub struct Effect {
    /// stable effect ID
    pub effect: Ability,
    /// all the positions that this effect is impacting on the map
    pub positions: Vec<Point2>,
}

/// power source information for Protoss
#[derive(Debug, Copy, Clone)]
pub struct PowerSource {
    /// unit tag of the power source
    pub tag: Tag,
    /// position of the power source
    pub pos: Point2,
    /// radius of the power source
    pub radius: f32,
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

/// information about a player in a replay
#[derive(Debug, Copy, Clone)]
pub struct ReplayPlayerInfo {
    /// id of the player
    pub player_id: u32,
    /// player ranking
    pub mmr: i32,
    /// player actions per minute
    pub apm: i32,

    /// actual player race
    pub race: Race,
    /// selected player race (if Random or None, race will be different)
    pub race_selected: Option<Race>,
    /// if the player won or lost
    pub game_result: Option<GameResult>,
}

impl FromProto<sc2api::PlayerInfoExtra> for ReplayPlayerInfo {
    fn from_proto(info: sc2api::PlayerInfoExtra) -> Result<Self> {
        Ok(Self {
            player_id: info.get_player_info().get_player_id(),

            race: info.get_player_info().get_race_actual().into_sc2()?,
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
                    Some(info.get_player_result().get_result().into_sc2()?)
                } else {
                    None
                }
            },
        })
    }
}

/// information about a replay file
#[derive(Debug, Clone)]
pub struct ReplayInfo {
    /// name of the map
    pub map_name: String,
    /// path to the map
    pub map_path: String,
    /// version of the game
    pub game_version: String,
    /// data version of the game
    pub data_version: String,

    /// duration in seconds
    pub duration: f32,
    /// duration in game steps
    pub duration_steps: u32,

    /// data build of the game
    pub data_build: u32,
    /// required base build of the game
    pub base_build: u32,

    /// information about specific players
    pub players: Vec<ReplayPlayerInfo>,
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
