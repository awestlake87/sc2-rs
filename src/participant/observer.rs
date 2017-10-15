
use std::collections::HashMap;
use std::mem;

use sc2_proto::sc2api;

use super::{ Participant, AppState };
use super::super::{ Result, Error };
use super::super::data::{
    PowerSource,
    PlayerData,
    GameState,
    Unit,
    Upgrade,
    Point2,
    Point3,
    Action,
    SpatialAction,
    Score
};

pub trait Observer {
    fn get_player_id(&self) -> Option<u32>;
    fn get_game_loop(&self) -> u32;
    fn get_units(&self) -> Vec<Unit>;
    fn get_actions(&self) -> &Vec<Action>;
    fn get_feature_layer_actions(&self) -> &Vec<SpatialAction>;
    fn get_power_sources(&self) -> &Vec<PowerSource>;
    fn get_upgrades(&self) -> &Vec<Upgrade>;
    fn get_score(&self) -> &Score;
    //fn get_ability_data(&self) -> &HashMap<Ability, AbilityData>;
    //fn get_unit_type_data(&self)
    //fn get_upgrade_data(&self)
    //fn get_buff_data(&self)
    //fn get_game_info(&self)
    fn get_minerals(&self) -> u32;
    fn get_vespene(&self) -> u32;
    fn get_food_cap(&self) -> u32;
    fn get_food_used(&self) -> u32;
    fn get_food_army(&self) -> u32;
    fn get_food_workers(&self) -> u32;
    fn get_idle_worker_count(&self) -> u32;
    fn get_army_count(&self) -> u32;
    fn get_warp_gate_count(&self) -> u32;
    fn get_camera_pos(&self) -> Point2;
    fn get_start_location(&self) -> Point3;
    fn has_creep(&self, point: Point2) -> bool;
    //fn get_visibility(&self, point: Point2) -> Visibility;
    fn is_pathable(&self, point: Point2) -> bool;
    fn is_placable(&self, point: Point2) -> bool;
    fn get_terrain_height(&self, point: Point2) -> f32;

    fn update_observation(&mut self) -> Result<()>;
}

impl Observer for Participant {
    fn get_player_id(&self) -> Option<u32> {
        self.player_id
    }
    fn get_game_loop(&self) -> u32 {
        self.game_state.current_game_loop
    }
    fn get_units(&self) -> Vec<Unit> {
        unimplemented!("get units");
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
    fn get_upgrades(&self) -> &Vec<Upgrade> {
        &self.upgrades
    }
    fn get_score(&self) -> &Score {
        unimplemented!("get score");
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
    fn has_creep(&self, _: Point2) -> bool {
        unimplemented!("has creep");
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

        self.score = Some(Score::from_proto(observation.get_score()));

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

            self.actions.push(Action::from_proto(&cmd));
        }

        for action in rsp.get_observation().get_actions() {
            if !action.has_action_feature_layer() {
                continue;
            }

            let fl = action.get_action_feature_layer();

            if fl.has_unit_command() {
                self.feature_layer_actions.push(
                    SpatialAction::from_unit_command_proto(
                        fl.get_unit_command()
                    )
                );
            }
            else if fl.has_camera_move() {
                self.feature_layer_actions.push(
                    SpatialAction::from_camera_move_proto(
                        fl.get_camera_move()
                    )
                );
            }
            else if fl.has_unit_selection_point() {
                self.feature_layer_actions.push(
                    SpatialAction::from_selection_point_proto(
                        fl.get_unit_selection_point()
                    )
                );
            }
            else if fl.has_unit_selection_rect() {
                self.feature_layer_actions.push(
                    SpatialAction::from_selection_rect_proto(
                        fl.get_unit_selection_rect()
                    )
                )
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
            let mut unit = Unit::from(unit.clone());
            let tag = unit.tag;

            unit.last_seen_game_loop = self.get_game_loop();

            self.units.insert(tag, unit);
        }

        if !raw.has_player() {
            return Err(Error::Todo("no player data"))
        }

        let player_raw = raw.get_player();
        if !player_raw.has_camera() {
            return Err(Error::Todo("no camera data"))
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
            self.upgrades.push(Upgrade::from_id(*upgrade_id));
        }

        Ok(())
    }
}
