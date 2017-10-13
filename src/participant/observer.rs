
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
    Action,
    SpatialAction
};

pub trait Observer {
    fn get_player_id(&self) -> Option<u32>;
    fn get_game_loop(&self) -> u32;
    fn get_units(&self) -> Vec<Unit>;

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

    fn update_observation(&mut self) -> Result<()> {
        if self.get_app_state() != AppState::Normal {
            unimplemented!("Err - app in bad state");
        }

        let mut req = sc2api::Request::new();
        req.mut_observation();

        self.send(req)?;
        let rsp = self.recv()?;

        {
            let observation = rsp.get_observation().get_observation();

            // TODO convert observation data to score

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
        }

        Ok(())
    }
}
