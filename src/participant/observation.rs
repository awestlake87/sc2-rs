
use std::collections::{ HashSet, HashMap };
use std::mem;
use std::rc::Rc;

use sc2_proto::sc2api;

use super::super::{ Result, FromProto, IntoSc2 };
use data::{
    PowerSource,
    PlayerData,
    TerrainInfo,
    Unit,
    Upgrade,
    Point2,
    Point3,
    Action,
    SpatialAction,
    Score,
    UnitType,
    UnitTypeData,
    Effect,
    Ability,
    AbilityData,
    UpgradeData,
    Buff,
    BuffData,
    PlayerResult,
    Visibility,
    Tag,
    Alliance,
    DisplayType
};
use participant::{ Participant, AppState };

pub enum GameEvent {
    UnitDestroyed(Rc<Unit>),
    UnitCreated(Rc<Unit>),
    UnitIdle(Rc<Unit>),
    UnitDetected(Rc<Unit>),

    UpgradeCompleted(Upgrade),
    BuildingCompleted(Rc<Unit>),

    NydusWormsDetected(u32),
    NukesDetected(u32),
}

pub struct GameData {
    ability_data:               HashMap<Ability, AbilityData>,
    unit_type_data:             HashMap<UnitType, UnitTypeData>,
    upgrade_data:               HashMap<Upgrade, UpgradeData>,
    buff_data:                  HashMap<Buff, BuffData>,
}

impl GameData {
    /// get ability data
    pub fn get_ability_data(&self) -> &HashMap<Ability, AbilityData> {
        &self.ability_data
    }
    /// get unit type data
    pub fn get_unit_type_data(&self) -> &HashMap<UnitType, UnitTypeData> {
        &self.unit_type_data
    }
    /// get upgrade data
    pub fn get_upgrade_data(&self) -> &HashMap<Upgrade, UpgradeData> {
        &self.upgrade_data
    }
    /// get buff data
    pub fn get_buff_data(&self) -> &HashMap<Buff, BuffData> {
        &self.buff_data
    }
}

pub struct GameState {
    pub player_id:                  u32,
    pub previous_step:              u32,
    pub current_step:               u32,
    pub camera_pos:                 Point2,

    pub units:                      HashMap<Tag, Rc<Unit>>,

    pub power_sources:              Vec<PowerSource>,
    pub effects:                    Vec<Effect>,
    pub upgrades:                   HashSet<Upgrade>,

    pub minerals:                   u32,
    pub vespene:                    u32,
    pub food_cap:                   u32,
    pub food_used:                  u32,
    pub food_army:                  u32,
    pub food_workers:               u32,
    pub idle_worker_count:          u32,
    pub army_count:                 u32,
    pub warp_gate_count:            u32,
    pub larva_count:                u32,

    pub score:                      Score
}

impl GameState {
    pub fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
        where F: Fn(&Unit) -> bool
    {
        let mut units = vec![ ];

        for unit in self.units.values() {
            if filter(&unit) {
                units.push(Rc::clone(&unit));
            }
        }

        units
    }
    /// check if the given point contains creep
    pub fn has_creep(&self, point: Point2) -> bool {
        unimplemented!("has creep")
    }
    /// get the visibility of the given point for the current player
    pub fn get_visibility(&self, point: Point2) -> Visibility {
        unimplemented!("get visibility")
    }
    /// whether the given point on the terrain is pathable
    ///
    /// this does not include pathing blockers like structures, for more
    /// accurate pathing results, use query interface
    pub fn is_pathable(&self, point: Point2) -> bool {
        unimplemented!("is pathable")
    }
    /// whether the given point on the terrain is buildable
    ///
    /// this does not include blockers like other structures. for more
    /// accurate building placement results, use query interface
    pub fn is_placable(&self, point: Point2) -> bool {
        unimplemented!("is placable")
    }
    /// returns the terrain height of the given point
    pub fn get_terrain_height(&self, point: Point2) -> f32 {
        unimplemented!("get terrain height")
    }
}

/// UNSTABLE observation trait
pub trait Observation {

    fn get_terrain_info(&mut self) -> Result<&TerrainInfo>;

    /// request a data update
    fn update_data(&mut self) -> Result<()>;
    /// request an observation update
    fn update_observation(&mut self) -> Result<()>;
}

impl Observation for Participant {
    fn get_terrain_info(&mut self) -> Result<&TerrainInfo> {
        let mut req = sc2api::Request::new();
        req.mut_game_info();

        self.send(req)?;
        let mut rsp = self.recv()?;

        if rsp.has_game_info() {
            self.terrain_info = rsp.take_game_info().into();
        }

        Ok(&self.terrain_info)
    }

    fn update_data(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();
        req.mut_data().set_unit_type_id(true);

        self.send(req)?;
        let mut rsp = self.recv()?;

        self.unit_type_data.clear();

        for data in rsp.mut_data().take_units().into_iter() {
            match UnitTypeData::from_proto(data) {
                Ok(u) => {
                    self.unit_type_data.insert(u.unit_type, u);
                },
                Err(e) => ()
            }
        }

        Ok(())
    }

    fn update_observation(&mut self) -> Result<()> {
        if self.get_app_state() != AppState::Normal {
            unimplemented!("Err - app in bad state");
        }

        let mut req = sc2api::Request::new();
        req.mut_observation();

        self.send(req)?;
        let mut rsp = self.recv()?;

        let mut observation = rsp.take_observation().take_observation();

        let mut state = mem::replace(&mut self.game_state, None);

        let previous_step = match state {
            Some(ref mut state) => {
                self.previous_units = mem::replace(
                    &mut state.units, HashMap::new()
                );
                self.previous_upgrades = mem::replace(
                    &mut state.upgrades, HashSet::new()
                );

                state.current_step
            },
            None => 0
        };
        let next_step = observation.get_game_loop();
        let is_new_frame = next_step != previous_step;

        let player_common = observation.take_player_common();
        let mut raw = observation.take_raw_data();
        let mut player_raw = raw.take_player();

        let new_state = GameState {
            player_id: player_common.get_player_id(),
            previous_step: previous_step,
            current_step: next_step,
            camera_pos: {
                let camera = player_raw.get_camera();

                Point2::new(camera.get_x(), camera.get_y())
            },

            units: {
                let mut units = HashMap::new();

                for unit in raw.take_units().into_iter() {
                    match Unit::from_proto(unit) {
                        Ok(mut unit) => {
                            let tag = unit.tag;

                            unit.last_seen_game_loop = next_step;

                            units.insert(tag, Rc::from(unit));
                        },
                        _ => ()
                    }
                }

                units
            },
            power_sources: {
                let mut power_sources = vec![ ];

                for p in player_raw.take_power_sources().into_iter() {
                    power_sources.push(p.into());
                }

                power_sources
            },
            upgrades: {
                let mut upgrades = HashSet::new();

                for u in player_raw.take_upgrade_ids().into_iter() {
                    upgrades.insert(Upgrade::from_proto(u)?);
                }

                upgrades
            },
            effects: vec![ ],

            minerals: player_common.get_minerals(),
            vespene: player_common.get_vespene(),
            food_used: player_common.get_food_used(),
            food_cap: player_common.get_food_cap(),
            food_army: player_common.get_food_army(),
            food_workers: player_common.get_food_workers(),
            idle_worker_count: player_common.get_idle_worker_count(),
            army_count: player_common.get_army_count(),
            warp_gate_count: player_common.get_warp_gate_count(),
            larva_count: player_common.get_larva_count(),

            score: observation.take_score().into_sc2()?,
        };

        self.game_state = Some(new_state);

        if is_new_frame {
            self.actions.clear();
            self.feature_layer_actions.clear();
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_raw() {
                continue;
            }

            let raw = action.get_action_raw();
            if !raw.has_unit_command() {
                continue;
            }

            let cmd = raw.get_unit_command();
            if !cmd.has_ability_id() {
                continue;
            }

            self.actions.push(cmd.clone().into_sc2()?);
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_feature_layer() {
                continue;
            }

            let fl = action.get_action_feature_layer();

            if fl.has_unit_command() {
                self.feature_layer_actions.push(
                    fl.get_unit_command().clone().into_sc2()?
                );
            }
            else if fl.has_camera_move() {
                self.feature_layer_actions.push(
                    fl.get_camera_move().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_point() {
                self.feature_layer_actions.push(
                    fl.get_unit_selection_point().clone().into_sc2()?
                );
            }
            else if fl.has_unit_selection_rect() {
                self.feature_layer_actions.push(
                    fl.get_unit_selection_rect().clone().into_sc2()?
                );
            }
        }

        // remap ability ids
        if self.use_generalized_ability {
            for action in &mut self.actions {
                action.ability = match self.ability_data.get(
                    &action.ability
                ) {
                    Some(ref ability_data) => {
                        ability_data.get_generalized_ability()
                    },
                    None => action.ability
                };
            }
            for action in &mut self.feature_layer_actions {
                match action {
                    &mut SpatialAction::UnitCommand {
                        ref mut ability, ..
                    } => {
                        *ability = match self.ability_data.get(ability) {
                            Some(ref ability_data) => {
                                ability_data.get_generalized_ability()
                            },
                            None => *ability
                        };
                    },
                    _ => ()
                };
            }
        }

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.previous_units.get_mut(tag) {
                    Some(ref mut unit) => {
                        Rc::get_mut(unit).unwrap().mark_dead();
                        self.events.push(
                            GameEvent::UnitDestroyed(Rc::clone(unit))
                        );
                    },
                    None => ()
                }
            }
        }

        for ref unit in self.units.values() {
            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if unit.orders.is_empty() && !prev_unit.orders.is_empty() {
                        self.events.push(
                            GameEvent::UnitIdle(Rc::clone(unit))
                        );
                    }
                    else if unit.build_progress >= 1.0
                        && prev_unit.build_progress < 1.0
                    {
                        self.events.push(
                            GameEvent::BuildingCompleted(Rc::clone(unit))
                        );
                    }
                },
                None => {
                    if unit.alliance == Alliance::Enemy &&
                        unit.display_type == DisplayType::Visible
                    {
                        self.events.push(
                            GameEvent::UnitDetected(Rc::clone(unit))
                        );
                    }
                    else {
                        self.events.push(
                            GameEvent::UnitCreated(Rc::clone(unit))
                        );
                    }

                    self.events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                }
            }
        }

        let prev_upgrades = mem::replace(
            &mut self.previous_upgrades, HashSet::new()
        );

        for upgrade in &self.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    self.events.push(GameEvent::UpgradeCompleted(*upgrade));
                }
            }
        }

        self.previous_upgrades = prev_upgrades;

        let mut nukes = 0;
        let mut nydus_worms = 0;

        for alert in observation.get_alerts() {
            match *alert {
                sc2api::Alert::NuclearLaunchDetected => nukes += 1,
                sc2api::Alert::NydusWormDetected => nydus_worms += 1
            }
        }

        if nukes > 0 {
            self.events.push(GameEvent::NukesDetected(nukes));
        }

        if nydus_worms > 0 {
            self.events.push(GameEvent::NydusWormsDetected(nydus_worms));
        }

        Ok(())
    }
}
