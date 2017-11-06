
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

use sc2_proto::data;
use sc2_proto::raw;
use sc2_proto::sc2api;

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
    Summoned
}

impl From<data::Attribute> for Attribute {
    fn from(a: data::Attribute) -> Self {
        match a {
            data::Attribute::Light => Attribute::Light,
            data::Attribute::Armored => Attribute::Armored,
            data::Attribute::Biological => Attribute::Biological,
            data::Attribute::Mechanical => Attribute::Mechanical,
            data::Attribute::Robotic => Attribute::Robotic,
            data::Attribute::Psionic => Attribute::Psionic,
            data::Attribute::Massive => Attribute::Massive,
            data::Attribute::Structure => Attribute::Structure,
            data::Attribute::Hover => Attribute::Hover,
            data::Attribute::Heroic => Attribute::Heroic,
            data::Attribute::Summoned => Attribute::Summoned,
        }
    }
}

pub struct DamageBonus {
    pub attribute:              Attribute,
    pub bonus:                  f32,
}

impl DamageBonus {
    pub fn from_proto(b: &data::DamageBonus) -> Self {
        Self {
            attribute: Attribute::from(b.get_attribute()),
            bonus: b.get_bonus()
        }
    }
}

pub enum WeaponTargetType {
    Ground,
    Air,
    Any,
}

impl From<data::Weapon_TargetType> for WeaponTargetType {
    fn from(target: data::Weapon_TargetType) -> Self {
        match target {
            data::Weapon_TargetType::Ground => WeaponTargetType::Ground,
            data::Weapon_TargetType::Air => WeaponTargetType::Air,
            data::Weapon_TargetType::Any => WeaponTargetType::Any,
        }
    }
}

pub struct Weapon {
    pub target_type:            WeaponTargetType,
    pub damage:                 f32,
    pub damage_bonus:           Vec<DamageBonus>,
    pub attacks:                u32,
    pub range:                  f32,
    pub speed:                  f32,
}

impl Weapon {
    pub fn from_proto(w: &data::Weapon) -> Self {
        Self {
            target_type: WeaponTargetType::from(w.get_field_type()),
            damage: w.get_damage(),
            damage_bonus: w.get_damage_bonus().iter().map(
                |b| DamageBonus::from_proto(b)
            ).collect(),
            attacks: w.get_attacks(),
            range: w.get_range(),
            speed: w.get_speed(),
        }
    }
}

pub struct UnitTypeData {
    pub unit_type:              UnitType,
    pub name:                   String,
    pub available:              bool,
    pub cargo_size:             u32,
    pub mineral_cost:           u32,
    pub vespene_cost:           u32,
    pub attributes:             Vec<Attribute>,
    pub movement_speed:         f32,
    pub armor:                  f32,
    pub weapons:                Vec<Weapon>,
    pub food_required:          f32,
    pub food_provided:          f32,
    pub ability:                Ability,
    pub race:                   Option<Race>,
    pub build_time:             f32,
    pub has_minerals:           bool,
    pub has_vespene:            bool,
    pub tech_alias:             Vec<UnitType>,
    pub unit_alias:             UnitType,
    pub tech_requirement:       UnitType,
    pub require_attached:       bool,
}

impl UnitTypeData {
    pub fn from_proto(data: &data::UnitTypeData) -> Self {
        Self {
            unit_type: UnitType::from_id(data.get_unit_id()),
            name: data.get_name().to_string(),
            available: data.get_available(),
            cargo_size: data.get_cargo_size(),
            mineral_cost: data.get_mineral_cost(),
            vespene_cost: data.get_vespene_cost(),

            attributes: {
                let mut attributes = vec![ ];

                for a in data.get_attributes() {
                    attributes.push(Attribute::from(*a));
                }

                attributes
            },

            movement_speed: data.get_movement_speed(),
            armor: data.get_armor(),
            weapons: data.get_weapons().iter().map(
                |w| Weapon::from_proto(w)
            ).collect(),
            food_required: data.get_food_required(),
            food_provided: data.get_food_provided(),

            ability: Ability::from_id(data.get_ability_id()),
            race: Race::from_proto(data.get_race()),
            build_time: data.get_build_time(),
            has_minerals: data.get_has_minerals(),
            has_vespene: data.get_has_vespene(),

            tech_alias: data.get_tech_alias().iter().map(
                |a| UnitType::from_id(*a)
            ).collect(),
            unit_alias: UnitType::from_id(data.get_unit_alias()),
            tech_requirement: UnitType::from_id(
                data.get_tech_requirement()
            ),
            require_attached: data.get_require_attached()
        }
    }
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


pub struct ReplayPlayerInfo {
    pub player_id:              u32,
    pub mmr:                    i32,
    pub apm:                    i32,

    pub race:                   Race,
    pub race_selected:          Option<Race>, // if player selected Random
    pub game_result:            Option<GameResult>,
}

pub struct ReplayInfo {
    pub map_name:               String,
    pub map_path:               String,
    pub game_version:           String,
    pub data_version:           String,

    pub duration:               f32,
    pub num_steps:              u32,

    pub data_build:             u32,
    pub base_build:             u32,

    pub players:                Vec<ReplayPlayerInfo>
}

impl ReplayInfo {
    pub fn from_proto(info: &sc2api::ResponseReplayInfo) -> Self {
        Self {
            map_name: info.get_map_name().to_string(),
            map_path: info.get_local_map_path().to_string(),
            game_version: info.get_game_version().to_string(),
            data_version: info.get_data_version().to_string(),

            duration: info.get_game_duration_seconds(),
            num_steps: info.get_game_duration_loops(),

            data_build: info.get_data_build(),
            base_build: info.get_base_build(),

            players: info.get_player_info().iter().map(
                |p| ReplayPlayerInfo {
                    player_id: p.get_player_info().get_player_id(),

                    race: Race::from_proto(
                        p.get_player_info().get_race_actual()
                    ).unwrap(),
                    race_selected: Race::from_proto(
                        p.get_player_info().get_race_requested()
                    ),

                    mmr: p.get_player_mmr(),
                    apm: p.get_player_apm(),

                    game_result: {
                        if p.has_player_result() {
                            Some(p.get_player_result().get_result().into())
                        }
                        else {
                            None
                        }
                    }
                }
            ).collect()
        }
    }
}
