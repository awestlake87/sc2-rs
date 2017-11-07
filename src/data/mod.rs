
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

/// generic structure to represent a 2D rectangle
#[derive(Copy, Clone)]
pub struct Rect<T> {
    /// x position of lefthand corner
    pub x: T,
    /// y position of lefthand corner
    pub y: T,
    /// width of the rectangle
    pub w: T,
    /// height of the rectangle
    pub h: T
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
#[derive(Copy, Clone)]
pub struct Rect2 {
    pub from:               Point2,
    pub to:                 Point2,
}

/// 2D integer point used to specify a location
pub type Point2I = na::Vector2<i32>;
/// 3D integer point used to specify a location
pub type Point3I = na::Vector3<i32>;

/// 2D integer rectangle represented by two points
#[derive(Copy, Clone)]
pub struct Rect2I {
    pub from:               Point2I,
    pub to:                 Point2I,
}

/// data for an ability that is currently available
pub struct AvailableAbility {
    /// the ability that is available
    pub ability:                Ability,
    /// indicates whether the ability requires a point to invoke
    pub requires_point:         bool,
}

/// target type of the ability
pub enum AbilityTarget {
    /// ability targets a location
    Point,
    /// ability targets another unit
    Unit,
    /// ability can target either a location or a unit
    PointOrUnit,
    /// ability can target either a location or nothing
    PointOrNone,
}

/// data about an ability
pub struct AbilityData {
    /// indicates whether the ability is available to the current mods/map
    pub available:              bool,
    /// stable ID for the ability
    pub ability:                Ability,
    /// catalog (game data xml) name of the ability
    pub link_name:              String,
    /// catalog (game data xml) index of the ability
    pub link_index:             u32,
    /// name of the button for the command card
    pub button_name:            String,
    /// in case the button name is not descriptive
    pub friendly_name:          String,
    /// UI hotkey
    pub hotkey:                 String,
    /// this ability may be represented by this more generic ability
    pub remaps_to_ability:      Option<Ability>,
    /// other abilities that can remap to this generic ability
    pub remaps_from_ability:    Vec<Ability>,
    /// type of target that this ability uses
    pub target:                 Option<AbilityTarget>,
    /// can be cast in the minimap (unimplemented)
    pub allow_minimap:          bool,
    /// autocast can be set
    pub allow_autocast:         bool,
    /// requires placement to construct a building
    pub is_building:            bool,
    /// if the ability is placing a building, give the radius of the footprint
    pub footprint_radius:       f32,
    /// placement next to an existing structure (an addon like a Tech Lab)
    pub is_instant_placement:   bool,
    /// range unit can cast ability without needing to approach target
    pub cast_range:             f32,
}

impl AbilityData {
    /// get the most generalized id of the ability
    pub fn get_generalized_ability(&self) -> Ability {
        match self.remaps_to_ability {
            Some(remap) => remap,
            None  => self.ability
        }
    }
}

/// all abilities available to a unit
pub struct AvailableUnitAbilities {
    /// the available abilities
    pub abilities:              Vec<AvailableAbility>,
    /// the tag of the unit
    pub unit_tag:               Tag,
    /// the type of the unit
    pub unit_type:              UnitType,
}

/// category of unit
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

/// damage bonus of a unit
pub struct DamageBonus {
    /// affected attribute
    pub attribute:              Attribute,
    /// damage bonus
    pub bonus:                  f32,
}

impl DamageBonus {
    /// convert the protobuf data
    pub fn from_proto(b: &data::DamageBonus) -> Self {
        Self {
            attribute: Attribute::from(b.get_attribute()),
            bonus: b.get_bonus()
        }
    }
}

/// target type of a weapon
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

/// unit weapon
pub struct Weapon {
    /// weapon's target type
    pub target_type:            WeaponTargetType,
    /// weapon damage
    pub damage:                 f32,
    /// any damage bonuses that apply to the weapon
    pub damage_bonus:           Vec<DamageBonus>,
    /// number of hits per attack (eg. Colossus has 2 beams)
    pub attacks:                u32,
    /// attack range
    pub range:                  f32,
    /// time between attacks
    pub speed:                  f32,
}

impl Weapon {
    /// convert from protobuf data
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

/// data about a unit type
///
/// this data is derived from the catalog (xml) data of the game and upgrades
pub struct UnitTypeData {
    /// stable unit ID
    pub unit_type:              UnitType,
    /// unit type name (corresponds to the game's catalog)
    pub name:                   String,
    /// whether this unit is available to the current mods/map
    pub available:              bool,
    /// number of cargo slots this unit occupies in a transport
    pub cargo_size:             u32,
    /// cost in minerals to build this unit
    pub mineral_cost:           u32,
    /// cost in vespene to build this unit
    pub vespene_cost:           u32,

    /// unit attributes (may change based on upgrades)
    pub attributes:             Vec<Attribute>,
    /// movement speed of this unit
    pub movement_speed:         f32,
    /// armor of this unit
    pub armor:                  f32,
    /// weapons on this unit
    pub weapons:                Vec<Weapon>,
    /// how much food this unit requires
    pub food_required:          f32,
    /// how much food this unit provides
    pub food_provided:          f32,
    /// which ability id creates this unit
    pub ability:                Ability,
    /// the race this unit belongs to
    pub race:                   Option<Race>,
    /// how long a unit takes to build
    pub build_time:             f32,
    /// whether this unit can have minerals (mineral patches)
    pub has_minerals:           bool,
    /// whether this unit can have vespene (vespene geysers)
    pub has_vespene:            bool,

    /// units this is equivalent to in terms of satisfying tech requirements
    pub tech_alias:             Vec<UnitType>,
    /// units that are morphed variants of the same unit
    pub unit_alias:             UnitType,
    /// structure required to build this unit (or any with same tech alias)
    pub tech_requirement:       UnitType,
    /// whether tech requirement is an addon
    pub require_attached:       bool,
}

impl UnitTypeData {
    /// convert from protobuf data
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

/// upgrade data
pub struct UpgradeData {
    /// stable upgrade ID
    pub upgrade:                Upgrade,
    /// upgrade name (corresponds to the game's catalog)
    pub name:                   String,
    /// mineral cost of researching this upgrade
    pub mineral_cost:           u32,
    /// vespene cost of researching this upgrade
    pub vespene_cost:           u32,
    /// ability that researches this upgrade
    pub ability:                Ability,
    /// time in game steps to research this upgrade
    pub research_time:          f32,
}

/// buff data
pub struct BuffData {
    /// stable buff ID
    pub buff:                   Buff,
    /// buff name (corresponds to the game's catalog)
    pub name:                   String,
}

/// effect data
pub struct EffectData {
    /// stable effect ID
    effect:                     Ability,
    /// effect name (corresponds to game's catalog)
    name:                       String,
    /// a more recognizable name of the effect
    friendly_name:              String,
    /// size of the circle the effect impacts
    radius:                     f32,
}

/// power source information for Protoss
pub struct PowerSource {
    /// unit tag of the power source
    pub tag:                    Tag,
    /// position of the power source
    pub pos:                    Point2,
    /// radius of the power source
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
