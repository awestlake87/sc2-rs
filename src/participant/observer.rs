
use std::collections::HashMap;
use std::mem;

use sc2_proto::sc2api;

use super::{ Participant, AppState };
use super::super::{ Result, Error };
use super::super::data::{
    PowerSource, PlayerData, GameState, Unit, Upgrade, Point2
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
            // convert observation data to score
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

            self.player_data = PlayerData {
                minerals:           player_common.get_minerals(),
                vespene:            player_common.get_vespene(),
                food_used:          player_common.get_food_used(),
                food_cap:           player_common.get_food_cap(),
                food_army:          player_common.get_food_army(),
                food_workers:       player_common.get_food_workers(),
                idle_worker_count:  player_common.get_idle_worker_count(),
                army_count:         player_common.get_army_count(),
                warp_gate_count:    player_common.get_warp_gate_count(),
                larva_count:        player_common.get_larva_count()
            };

            if is_new_frame {
                //raw_actions.clear()
                //feature_layer_actions = SpatialActions()
            }

            /*
            if self.use_generalized_ability {
                //TODO this
            }
            */

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

            // get camera data
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
