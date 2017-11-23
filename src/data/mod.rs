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

use na;
use na::geometry;

use sc2_proto::common;
use sc2_proto::data;
use sc2_proto::raw;
use sc2_proto::sc2api;

use super::{ Result, FromProto, IntoSc2 };

pub use self::ability::*;
pub use self::action::*;
pub use self::buff::*;
pub use self::game::*;
pub use self::image::*;
pub use self::player::*;
pub use self::score::*;
pub use self::unit::*;
pub use self::upgrade::*;

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
#[derive(Debug, Copy, Clone)]
pub struct Rect2 {
    /// upper left-hand corner
    pub from:               Point2,
    /// lower right-hand corner
    pub to:                 Point2,
}

/// 2D integer point used to specify a location
pub type Point2I = na::Vector2<i32>;
/// 3D integer point used to specify a location
pub type Point3I = na::Vector3<i32>;

/// 2D integer rectangle represented by two points
#[derive(Debug, Copy, Clone)]
pub struct Rect2I {
    /// upper left-hand corner
    pub from:               Point2I,
    /// lower right-hand corner
    pub to:                 Point2I,
}

/// visibility of a point on the terrain
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Hidden,
    Fogged,
    Visible,
    FullHidden
}

/// data for an ability that is currently available
#[derive(Debug, Copy, Clone)]
pub struct AvailableAbility {
    /// the ability that is available
    pub ability:                Ability,
    /// indicates whether the ability requires a point to invoke
    pub requires_point:         bool,
}

/// target type of the ability
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl FromProto<data::Attribute> for Attribute {
    fn from_proto(a: data::Attribute) -> Result<Self> {
        Ok(
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
        )
    }
}

/// damage bonus of a unit
#[derive(Debug, Copy, Clone)]
pub struct DamageBonus {
    /// affected attribute
    pub attribute:              Attribute,
    /// damage bonus
    pub bonus:                  f32,
}

impl FromProto<data::DamageBonus> for DamageBonus {
    fn from_proto(b: data::DamageBonus) -> Result<Self> {
        Ok(
            Self {
                attribute: b.get_attribute().into_sc2()?,
                bonus: b.get_bonus()
            }
        )
    }
}

/// target type of a weapon
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WeaponTargetType {
    Ground,
    Air,
    Any,
}

impl FromProto<data::Weapon_TargetType> for WeaponTargetType {
    fn from_proto(target: data::Weapon_TargetType) -> Result<Self> {
        Ok(
            match target {
                data::Weapon_TargetType::Ground => WeaponTargetType::Ground,
                data::Weapon_TargetType::Air => WeaponTargetType::Air,
                data::Weapon_TargetType::Any => WeaponTargetType::Any,
            }
        )
    }
}

/// unit weapon
#[derive(Debug, Clone)]
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

impl FromProto<data::Weapon> for Weapon {
    fn from_proto(mut w: data::Weapon) -> Result<Self> {
        Ok(
            Self {
                target_type: w.get_field_type().into_sc2()?,
                damage: w.get_damage(),
                damage_bonus: {
                    let mut bonuses = vec![ ];

                    for b in w.take_damage_bonus().into_iter() {
                        bonuses.push(b.into_sc2()?);
                    }

                    bonuses
                },
                attacks: w.get_attacks(),
                range: w.get_range(),
                speed: w.get_speed(),
            }
        )
    }
}

/// data about a unit type
///
/// this data is derived from the catalog (xml) data of the game and upgrades
#[derive(Debug, Clone)]
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

impl FromProto<data::UnitTypeData> for UnitTypeData {
    fn from_proto(mut data: data::UnitTypeData) -> Result<Self> {
        Ok(
            Self {
                unit_type: data.get_unit_id().into_sc2()?,
                name: data.get_name().to_string(),
                available: data.get_available(),
                cargo_size: data.get_cargo_size(),
                mineral_cost: data.get_mineral_cost(),
                vespene_cost: data.get_vespene_cost(),

                attributes: {
                    let mut attributes = vec![ ];

                    for a in data.take_attributes().into_iter() {
                        attributes.push(a.into_sc2()?);
                    }

                    attributes
                },

                movement_speed: data.get_movement_speed(),
                armor: data.get_armor(),
                weapons: {
                    let mut weapons = vec![ ];

                    for w in data.take_weapons().into_iter() {
                        weapons.push(w.into_sc2()?);
                    }

                    weapons
                },
                food_required: data.get_food_required(),
                food_provided: data.get_food_provided(),

                ability: data.get_ability_id().into_sc2()?,
                race: {
                    if data.has_race()
                        && data.get_race() != common::Race::NoRace
                    {
                        Some(data.get_race().into_sc2()?)
                    }
                    else {
                        None
                    }
                },
                build_time: data.get_build_time(),
                has_minerals: data.get_has_minerals(),
                has_vespene: data.get_has_vespene(),

                tech_alias: {
                    let mut aliases = vec![ ];

                    for a in data.get_tech_alias() {
                        aliases.push(UnitType::from_proto(*a)?);
                    }

                    aliases
                },
                unit_alias: UnitType::from_proto(data.get_unit_alias())?,
                tech_requirement: UnitType::from_proto(
                    data.get_tech_requirement()
                )?,
                require_attached: data.get_require_attached()
            }
        )
    }
}

/// upgrade data
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct BuffData {
    /// stable buff ID
    pub buff:                   Buff,
    /// buff name (corresponds to the game's catalog)
    pub name:                   String,
}

/// effect data
#[derive(Debug, Clone)]
pub struct EffectData {
    /// stable effect ID
    pub effect:                 Ability,
    /// effect name (corresponds to game's catalog)
    pub name:                   String,
    /// a more recognizable name of the effect
    pub friendly_name:          String,
    /// size of the circle the effect impacts
    pub radius:                 f32,
}

/// visuals of a persistent ability on the map (eg. PsiStorm)
#[derive(Debug, Clone)]
pub struct Effect {
    /// stable effect ID
    pub effect:                 Ability,
    /// all the positions that this effect is impacting on the map
    pub positions:              Vec<Point2>,
}

/// power source information for Protoss
#[derive(Debug, Copy, Clone)]
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

/// information about a player in a replay
#[derive(Debug, Copy, Clone)]
pub struct ReplayPlayerInfo {
    /// id of the player
    pub player_id:              u32,
    /// player ranking
    pub mmr:                    i32,
    /// player actions per minute
    pub apm:                    i32,

    /// actual player race
    pub race:                   Race,
    /// selected player race (if Random or None, race will be different)
    pub race_selected:          Option<Race>,
    /// if the player won or lost
    pub game_result:            Option<GameResult>,
}

impl FromProto<sc2api::PlayerInfoExtra> for ReplayPlayerInfo {
    fn from_proto(info: sc2api::PlayerInfoExtra) -> Result<Self> {
        Ok(
            Self {
                player_id: info.get_player_info().get_player_id(),

                race: info.get_player_info().get_race_actual().into_sc2()?,
                race_selected: {
                    if info.get_player_info().has_race_requested() {
                        let proto_race = info.get_player_info()
                            .get_race_requested()
                        ;

                        if proto_race != common::Race::NoRace {
                            Some(proto_race.into_sc2()?)
                        }
                        else {
                            None
                        }
                    }
                    else {
                        None
                    }
                },

                mmr: info.get_player_mmr(),
                apm: info.get_player_apm(),

                game_result: {
                    if info.has_player_result() {
                        Some(info.get_player_result().get_result().into_sc2()?)
                    }
                    else {
                        None
                    }
                }
            }
        )
    }
}

/// information about a replay file
#[derive(Debug, Clone)]
pub struct ReplayInfo {
    /// name of the map
    pub map_name:               String,
    /// path to the map
    pub map_path:               String,
    /// version of the game
    pub game_version:           String,
    /// data version of the game
    pub data_version:           String,

    /// duration in seconds
    pub duration:               f32,
    /// duration in game steps
    pub duration_steps:         u32,

    /// data build of the game
    pub data_build:             u32,
    /// required base build of the game
    pub base_build:             u32,

    /// information about specific players
    pub players:                Vec<ReplayPlayerInfo>
}

impl FromProto<sc2api::ResponseReplayInfo> for ReplayInfo {
    fn from_proto(mut info: sc2api::ResponseReplayInfo) -> Result<Self> {
        Ok(
            Self {
                map_name: info.take_map_name(),
                map_path: info.take_local_map_path(),
                game_version: info.take_game_version(),
                data_version: info.take_data_version(),

                duration: info.get_game_duration_seconds(),
                duration_steps: info.get_game_duration_loops(),

                data_build: info.get_data_build(),
                base_build: info.get_base_build(),

                players: {
                    let mut player_info = vec![ ];

                    for p in info.take_player_info().into_iter() {
                        player_info.push(p.into_sc2()?);
                    }

                    player_info
                }
            }
        )
    }
}
