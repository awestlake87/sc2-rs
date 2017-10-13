
mod ability;
mod action;
mod buff;
mod game;
mod player;
mod unit;
mod upgrade;

use na::{ Vector2, Vector3 };

use sc2_proto::raw;

pub use self::ability::*;
pub use self::action::*;
pub use self::buff::*;
pub use self::game::*;
pub use self::player::*;
pub use self::unit::*;
pub use self::upgrade::*;

#[derive(Copy, Clone)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T
}

pub type Point2 = Vector2<f32>;
pub type Point3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Rect2 {
    pub from:               Point2,
    pub to:                 Point2,
}

pub type Point2I = Vector2<i32>;
pub type Point3I = Vector3<i32>;

#[derive(Copy, Clone)]
pub struct Rect2I {
    pub from:               Point2I,
    pub to:                 Point2I,
}

pub struct AvailableAbility {
    ability:                Ability,
    requires_point:         bool,
}

pub enum AbilityTarget {
    Point,
    Unit,
    PointOrUnit,
    PointOrNone,
}

pub struct AbilityData {
    available:              bool,
    ability:                Ability,
    link_name:              String,
    link_index:             u32,
    button_name:            String,
    friendly_name:          String,
    hotkey:                 String,
    remaps_to_ability:      Option<Ability>,
    remaps_from_ability:    Vec<Ability>,
    target:                 Option<AbilityTarget>,
    allow_minimap:          bool,
    allow_autocast:         bool,
    is_building:            bool,
    footprint_radius:       f32,
    is_instant_placement:   bool,
    cast_range:             f32,
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
    abilities:              Vec<AvailableAbilities>,
    unit_tag:               Tag,
    unit_type:              UnitType,
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
    attribute:              Attribute,
    bonus:                  f32,
}

pub enum WeaponTargetType {
    Ground,
    Air,
    Any,
    Invalid,
}

pub struct Weapon {
    target_type:            WeaponTargetType,
    damage:                 f32,
    damage_bonus:           Vec<DamageBonus>,
    attacks:                u32,
    range:                  f32,
    speed:                  f32,
}

pub struct UnitTypeData {
    unit_type:              UnitType,
    name:                   String,
    available:              bool,
    cargo_size:             u32,
    mineral_cost:           i32,
    vespene_cost:           i32,
    attributes:             Vec<Attribute>,
    movement_speed:         f32,
    armor:                  f32,
    weapons:                Vec<Weapon>,
    food_required:          f32,
    food_provided:          f32,
    ability:                Ability,
    race:                   Race,
    build_time:             f32,
    has_minerals:           bool,
    has_vespene:            bool,
    tech_alias:             Vec<UnitType>,
    tech_requirement:       UnitType,
    require_attached:       bool,
}

pub struct UpgradeData {
    upgrade:                Upgrade,
    name:                   String,
    mineral_cost:           u32,
    vespene_cost:           u32,
    ability:                Ability,
    research_time:          f32,
}

pub struct BuffData {
    buff:                   Buff,
    name:                   String,
}

pub struct PowerSource {
    tag:                    Tag,
    pos:                    Point2,
    radius:                 f32,
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
