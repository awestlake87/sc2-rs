
mod ability;
mod action;
mod buff;
mod game;
mod player;
mod score;
mod unit;
mod upgrade;

use na;
use na::geometry;

use sc2_proto::raw;

pub use self::ability::*;
pub use self::action::*;
pub use self::buff::*;
pub use self::game::*;
pub use self::player::*;
pub use self::score::*;
pub use self::unit::*;
pub use self::upgrade::*;

#[derive(Copy, Clone)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T
}

pub type Vector2 = na::Vector2<f32>;
pub type Vector3 = na::Vector3<f32>;
pub type Point2 = geometry::Point2<f32>;
pub type Point3 = geometry::Point3<f32>;

#[derive(Copy, Clone)]
pub struct Rect2 {
    pub from:               Point2,
    pub to:                 Point2,
}

pub type Point2I = na::Vector2<i32>;
pub type Point3I = na::Vector3<i32>;

#[derive(Copy, Clone)]
pub struct Rect2I {
    pub from:               Point2I,
    pub to:                 Point2I,
}

pub struct AvailableAbility {
    pub ability:                Ability,
    pub requires_point:         bool,
}

pub enum AbilityTarget {
    Point,
    Unit,
    PointOrUnit,
    PointOrNone,
}

pub struct AbilityData {
    pub available:              bool,
    pub ability:                Ability,
    pub link_name:              String,
    pub link_index:             u32,
    pub button_name:            String,
    pub friendly_name:          String,
    pub hotkey:                 String,
    pub remaps_to_ability:      Option<Ability>,
    pub remaps_from_ability:    Vec<Ability>,
    pub target:                 Option<AbilityTarget>,
    pub allow_minimap:          bool,
    pub allow_autocast:         bool,
    pub is_building:            bool,
    pub footprint_radius:       f32,
    pub is_instant_placement:   bool,
    pub cast_range:             f32,
}

impl AbilityData {
    pub fn get_generalized_ability(&self) -> Ability {
        match self.remaps_to_ability {
            Some(remap) => remap,
            None  => self.ability
        }
    }
}

pub struct AvailableAbilities {
    pub abilities:              Vec<AvailableAbilities>,
    pub unit_tag:               Tag,
    pub unit_type:              UnitType,
}

pub enum Attribute {
    Light,
    Armored,
    Biological,
    Mechanical,
    Robotic,
    Psionic,
    Massive,
    Structure,
    Hover,
    Heroic,
    Summoned,
    Invalid
}

pub struct DamageBonus {
    pub attribute:              Attribute,
    pub bonus:                  f32,
}

pub enum WeaponTargetType {
    Ground,
    Air,
    Any,
    Invalid,
}

pub struct Weapon {
    pub target_type:            WeaponTargetType,
    pub damage:                 f32,
    pub damage_bonus:           Vec<DamageBonus>,
    pub attacks:                u32,
    pub range:                  f32,
    pub speed:                  f32,
}

pub struct UnitTypeData {
    pub unit_type:              UnitType,
    pub name:                   String,
    pub available:              bool,
    pub cargo_size:             u32,
    pub mineral_cost:           i32,
    pub vespene_cost:           i32,
    pub attributes:             Vec<Attribute>,
    pub movement_speed:         f32,
    pub armor:                  f32,
    pub weapons:                Vec<Weapon>,
    pub food_required:          f32,
    pub food_provided:          f32,
    pub ability:                Ability,
    pub race:                   Race,
    pub build_time:             f32,
    pub has_minerals:           bool,
    pub has_vespene:            bool,
    pub tech_alias:             Vec<UnitType>,
    pub tech_requirement:       UnitType,
    pub require_attached:       bool,
}

pub struct UpgradeData {
    pub upgrade:                Upgrade,
    pub name:                   String,
    pub mineral_cost:           u32,
    pub vespene_cost:           u32,
    pub ability:                Ability,
    pub research_time:          f32,
}

pub struct BuffData {
    pub buff:                   Buff,
    pub name:                   String,
}

pub struct PowerSource {
    pub tag:                    Tag,
    pub pos:                    Point2,
    pub radius:                 f32,
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
