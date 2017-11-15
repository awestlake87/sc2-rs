
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

use sc2_proto::sc2api;

use super::super::{ Result, FromProto, IntoSc2 };
use data::{
    PowerSource,
    PlayerData,
    GameState,
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
};
use participant::{ Participant, AppState };

/// UNSTABLE observation trait
pub trait Observation {
    /// get the player id associated with the participant
    fn get_player_id(&self) -> Option<u32>;
    /// get the current game loop (or game step)
    fn get_game_loop(&self) -> u32;
    /// get a list of all known units at the moment
    fn get_units(&self) -> Vec<Rc<Unit>>;

    /// get a list of all known units that match the filter condition
    fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
        where F: Fn(&Unit) -> bool
    ;

    /// get the actions performed as abilities applied to units
    ///
    /// (for use with the raw option)
    fn get_actions(&self) -> &Vec<Action>;
    /// get the actions performed as abilities applied to the current selection
    ///
    /// (for use with the feature layer or rendered options)
    fn get_feature_layer_actions(&self) -> &Vec<SpatialAction>;
    /// get all power sources associated with the current player
    fn get_power_sources(&self) -> &Vec<PowerSource>;
    /// get all active effects in vision of the current player
    fn get_effects(&self) -> &Vec<Effect>;
    /// get all upgrades
    fn get_upgrades(&self) -> &Vec<Upgrade>;
    /// get detailed current set of scores
    fn get_score(&self) -> &Score;
    /// get ability data
    fn get_ability_data(&self) -> &HashMap<Ability, AbilityData>;
    /// get unit type data
    fn get_unit_type_data(&self) -> &HashMap<UnitType, UnitTypeData>;
    /// get upgrade data
    fn get_upgrade_data(&self) -> &HashMap<Upgrade, UpgradeData>;
    /// get buff data
    fn get_buff_data(&self) -> &HashMap<Buff, BuffData>;
    /// get terrain info
    fn get_terrain_info(&mut self) -> Result<&TerrainInfo>;

    /// get current mineral count
    fn get_minerals(&self) -> u32;
    /// get current vespene count
    fn get_vespene(&self) -> u32;
    /// get the total supply cap given the players max supply
    fn get_food_cap(&self) -> u32;
    /// the total supply used by the player
    fn get_food_used(&self) -> u32;
    /// the total supply consumed by army units alone
    fn get_food_army(&self) -> u32;
    /// the total supply consumed by workers alone
    fn get_food_workers(&self) -> u32;
    /// the number of workers that currently have no orders
    fn get_idle_worker_count(&self) -> u32;
    /// the number of army units
    fn get_army_count(&self) -> u32;
    /// the number of warp gates owned by the player
    fn get_warp_gate_count(&self) -> u32;
    /// position of the center of the camera
    fn get_camera_pos(&self) -> Point2;
    /// gets the initial start location of the player
    fn get_start_location(&self) -> Point3;
    /// gets the results of the game
    fn get_results(&self) -> Option<&Vec<PlayerResult>>;
    /// check if the given point contains creep
    fn has_creep(&self, point: Point2) -> bool;
    /// get the visibility of the given point for the current player
    fn get_visibility(&self, point: Point2) -> Visibility;
    /// whether the given point on the terrain is pathable
    ///
    /// this does not include pathing blockers like structures, for more
    /// accurate pathing results, use query interface
    fn is_pathable(&self, point: Point2) -> bool;
    /// whether the given point on the terrain is buildable
    ///
    /// this does not include blockers like other structures. for more
    /// accurate building placement results, use query interface
    fn is_placable(&self, point: Point2) -> bool;
    /// returns the terrain height of the given point
    fn get_terrain_height(&self, point: Point2) -> f32;

    /// request a data update
    fn update_data(&mut self) -> Result<()>;
    /// request an observation update
    fn update_observation(&mut self) -> Result<()>;
}

impl Observation for Participant {
    fn get_player_id(&self) -> Option<u32> {
        self.player_id
    }
    fn get_game_loop(&self) -> u32 {
        self.game_state.current_game_loop
    }
    fn get_units(&self) -> Vec<Rc<Unit>> {
        unimplemented!("get units");
    }
    fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
        where F: Fn(&Unit) -> bool
    {
        let mut units = vec![ ];

        for unit in self.units.values() {
            if filter(unit.as_ref()) {
                units.push(unit.clone());
            }
        }

        units
    }
    fn get_actions(&self) -> &Vec<Action> {
        &self.actions
    }
    fn get_feature_layer_actions(&self) -> &Vec<SpatialAction> {
        &self.feature_layer_actions
    }
    fn get_power_sources(&self) -> &Vec<PowerSource> {
        &self.power_sources
    }
    fn get_effects(&self) -> &Vec<Effect> {
        unimplemented!("get effects");
    }
    fn get_upgrades(&self) -> &Vec<Upgrade> {
        &self.upgrades
    }
    fn get_score(&self) -> &Score {
        unimplemented!("get score");
    }
    fn get_ability_data(&self) -> &HashMap<Ability, AbilityData> {
        unimplemented!("get ability data");
    }
    fn get_upgrade_data(&self) -> &HashMap<Upgrade, UpgradeData> {
        unimplemented!("get upgrade data");
    }
    fn get_buff_data(&self) -> &HashMap<Buff, BuffData> {
        unimplemented!("get buff data");
    }
    //fn get_ability_data(&self) -> &HashMap<Ability, AbilityData>;
    fn get_unit_type_data(&self) -> &HashMap<UnitType, UnitTypeData> {
        &self.unit_type_data
    }
    //fn get_upgrade_data(&self)
    //fn get_buff_data(&self)

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
    fn get_minerals(&self) -> u32 {
        self.player_data.minerals
    }
    fn get_vespene(&self) -> u32 {
        self.player_data.vespene
    }
    fn get_food_cap(&self) -> u32 {
        self.player_data.food_cap
    }
    fn get_food_used(&self) -> u32 {
        self.player_data.food_used
    }
    fn get_food_army(&self) -> u32 {
        self.player_data.food_army
    }
    fn get_food_workers(&self) -> u32 {
        self.player_data.food_workers
    }
    fn get_idle_worker_count(&self) -> u32 {
        self.player_data.idle_worker_count
    }
    fn get_army_count(&self) -> u32 {
        self.player_data.army_count
    }
    fn get_warp_gate_count(&self) -> u32 {
        self.player_data.warp_gate_count
    }
    fn get_camera_pos(&self) -> Point2 {
        unimplemented!("get camera pos");
    }
    fn get_start_location(&self) -> Point3 {
        unimplemented!("get start location");
    }
    fn get_results(&self) -> Option<&Vec<PlayerResult>> {
        unimplemented!("get results");
    }
    fn has_creep(&self, _: Point2) -> bool {
        unimplemented!("has creep");
    }
    fn get_visibility(&self, _: Point2) -> Visibility {
        unimplemented!("get visibility");
    }
    fn is_pathable(&self, _: Point2) -> bool {
        unimplemented!("is pathable");
    }
    fn is_placable(&self, _: Point2) -> bool {
        unimplemented!("is placable");
    }
    fn get_terrain_height(&self, _: Point2) -> f32 {
        unimplemented!("get terrain height");
    }

    fn update_data(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();
        req.mut_data().set_unit_type_id(true);

        self.send(req)?;
        let mut rsp = self.recv()?;

        self.unit_type_data.clear();

        for data in rsp.mut_data().take_units().into_iter() {
            let u = UnitTypeData::from_proto(data)?;

            self.unit_type_data.insert(u.unit_type, u);
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

        self.observation = rsp.take_observation();
        let observation = self.observation.get_observation();

        self.score = Some(observation.get_score().clone().into_sc2()?);

        let next_game_loop = observation.get_game_loop();
        let is_new_frame = next_game_loop != self.get_game_loop();

        self.game_state = GameState {
            previous_game_loop: self.get_game_loop(),
            current_game_loop: next_game_loop
        };

        let player_common = observation.get_player_common();
        if player_common.has_player_id() {
            self.player_id = Some(player_common.get_player_id());
        }

        self.player_data = PlayerData::from(player_common.clone());

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

        let raw = observation.get_raw_data();
        self.previous_units = mem::replace(
            &mut self.units, HashMap::new()
        );

        for unit in raw.get_units().iter() {
            let mut unit = Unit::from_proto(unit.clone())?;
            let tag = unit.tag;

            unit.last_seen_game_loop = self.get_game_loop();

            self.units.insert(tag, Rc::from(unit));
        }

        if !raw.has_player() {
            bail!("no player data")
        }

        let player_raw = raw.get_player();
        if !player_raw.has_camera() {
            bail!("no camera data")
        }

        self.camera_pos = {
            let camera = player_raw.get_camera();
            Some(Point2::new(camera.get_x(), camera.get_y()))
        };

        self.power_sources.clear();
        for power_source in player_raw.get_power_sources() {
            self.power_sources.push(
                PowerSource::from(power_source.clone())
            );
        }

        self.previous_upgrades = mem::replace(&mut self.upgrades, vec![ ]);
        for upgrade_id in player_raw.get_upgrade_ids() {
            self.upgrades.push(Upgrade::from_proto(*upgrade_id)?);
        }

        Ok(())
    }
}
