
use sc2_proto::score::{
     Score as ProtoScore,
     Score_ScoreType as ProtoScoreType,
     ScoreDetails as ProtoScoreDetails,
     CategoryScoreDetails as ProtoCategoryScoreDetails,
     VitalScoreDetails as ProtoVitalScoreDetails,
};

pub struct Score {
    pub score_type:                     ScoreType,
    pub score:                          f32,
    pub details:                        ScoreDetails
}

impl Score {
    pub fn from_proto(score: &ProtoScore) -> Self {
        Self {
            score_type: {
                if score.has_score_type() {
                    match score.get_score_type() {
                        ProtoScoreType::Curriculum => ScoreType::Curriculum,
                        ProtoScoreType::Melee => ScoreType::Melee,
                    }
                }
                else {
                    ScoreType::Melee
                }
            },
            score: score.get_score() as f32,
            details: ScoreDetails::from_proto(score.get_score_details()),
        }
    }
}

pub enum ScoreType {
    Curriculum,
    Melee
}

pub struct ScoreEntry {
    pub name:                           String,
    pub offset:                         i32,
    pub used:                           bool,
    pub nonzero:                        bool,
}

pub struct CategoryScoreDetails {
    pub none:                           f32,
    pub army:                           f32,
    pub economy:                        f32,
    pub technology:                     f32,
    pub upgrade:                        f32,
}

impl CategoryScoreDetails {
    pub fn from_proto(details: &ProtoCategoryScoreDetails) -> Self {
        Self {
            none: details.get_none(),
            army: details.get_army(),
            economy: details.get_economy(),
            technology: details.get_technology(),
            upgrade: details.get_upgrade(),
        }
    }
}

pub struct VitalScoreDetails {
    pub life:                           f32,
    pub shields:                        f32,
    pub energy:                         f32
}

impl VitalScoreDetails {
    pub fn from_proto(details: &ProtoVitalScoreDetails) -> Self {
        Self {
            life: details.get_life(),
            shields: details.get_shields(),
            energy: details.get_energy(),
        }
    }
}

pub struct ScoreDetails {
    pub idle_production_time:           f32,
    pub idle_worker_time:               f32,

    pub total_value_units:              f32,
    pub total_value_structures:         f32,

    pub killed_value_units:             f32,
    pub killed_value_structures:        f32,

    pub collected_minerals:             f32,
    pub collected_vespene:              f32,

    pub collection_rate_minerals:       f32,
    pub collection_rate_vespene:        f32,

    pub spent_minerals:                 f32,
    pub spent_vespene:                  f32,

    pub food_used:                      Option<CategoryScoreDetails>,

    pub killed_minerals:                Option<CategoryScoreDetails>,
    pub killed_vespene:                 Option<CategoryScoreDetails>,

    pub lost_minerals:                  Option<CategoryScoreDetails>,
    pub lost_vespene:                   Option<CategoryScoreDetails>,

    pub friendly_fire_minerals:         Option<CategoryScoreDetails>,
    pub friendly_fire_vespene:          Option<CategoryScoreDetails>,

    pub used_minerals:                  Option<CategoryScoreDetails>,
    pub used_vespene:                   Option<CategoryScoreDetails>,

    pub total_used_minerals:            Option<CategoryScoreDetails>,
    pub total_used_vespene:             Option<CategoryScoreDetails>,

    pub total_damage_dealt:             Option<VitalScoreDetails>,
    pub total_damage_taken:             Option<VitalScoreDetails>,
    pub total_healed:                   Option<VitalScoreDetails>,
}

impl ScoreDetails {
    pub fn from_proto(details: &ProtoScoreDetails) -> Self {
        Self {
            idle_production_time: details.get_idle_production_time(),
            idle_worker_time: details.get_idle_worker_time(),

            total_value_units: details.get_total_value_units(),
            total_value_structures: details.get_total_value_structures(),

            killed_value_units: details.get_killed_value_units(),
            killed_value_structures: details.get_killed_value_structures(),

            collected_minerals: details.get_collected_minerals(),
            collected_vespene: details.get_collected_vespene(),

            collection_rate_minerals: details.get_collection_rate_minerals(),
            collection_rate_vespene: details.get_collection_rate_vespene(),

            spent_minerals: details.get_spent_minerals(),
            spent_vespene: details.get_spent_vespene(),

            food_used: {
                if details.has_food_used() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_food_used()
                        )
                    )
                }
                else {
                    None
                }
            },

            killed_minerals: {
                if details.has_killed_minerals() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_killed_minerals()
                        )
                    )
                }
                else {
                    None
                }
            },
            killed_vespene: {
                if details.has_killed_vespene() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_killed_vespene()
                        )
                    )
                }
                else {
                    None
                }
            },

            lost_minerals: {
                if details.has_lost_minerals() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_lost_minerals()
                        )
                    )
                }
                else {
                    None
                }
            },
            lost_vespene: {
                if details.has_lost_vespene() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_lost_vespene()
                        )
                    )
                }
                else {
                    None
                }
            },

            friendly_fire_minerals: {
                if details.has_friendly_fire_minerals() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_friendly_fire_minerals()
                        )
                    )
                }
                else {
                    None
                }
            },
            friendly_fire_vespene: {
                if details.has_friendly_fire_vespene() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_friendly_fire_vespene()
                        )
                    )
                }
                else {
                    None
                }
            },

            used_minerals: {
                if details.has_used_minerals() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_used_minerals()
                        )
                    )
                }
                else {
                    None
                }
            },
            used_vespene: {
                if details.has_used_vespene() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_used_vespene()
                        )
                    )
                }
                else {
                    None
                }
            },

            total_used_minerals: {
                if details.has_total_used_minerals() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_total_used_minerals()
                        )
                    )
                }
                else {
                    None
                }
            },
            total_used_vespene: {
                if details.has_total_used_vespene() {
                    Some(
                        CategoryScoreDetails::from_proto(
                            details.get_total_used_vespene()
                        )
                    )
                }
                else {
                    None
                }
            },

            total_damage_dealt: {
                if details.has_total_damage_dealt() {
                    Some(
                        VitalScoreDetails::from_proto(
                            details.get_total_damage_dealt()
                        )
                    )
                }
                else {
                    None
                }
            },
            total_damage_taken: {
                if details.has_total_damage_taken() {
                    Some(
                        VitalScoreDetails::from_proto(
                            details.get_total_damage_taken()
                        )
                    )
                }
                else {
                    None
                }
            },
            total_healed: {
                if details.has_total_healed() {
                    Some(
                        VitalScoreDetails::from_proto(
                            details.get_total_healed()
                        )
                    )
                }
                else {
                    None
                }
            }
        }
    }
}
