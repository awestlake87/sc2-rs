
use std::collections::HashMap;
use std::mem;

use sc2_proto::sc2api;

use super::{ Participant, AppState };
use super::super::{ Result };
use super::super::game::{ PlayerData, GameState };
use super::super::unit::{ Unit };

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
            self.previous_units = mem::replace(&mut self.units, HashMap::new());
            self.units.clear();

            for unit in raw.get_units().iter() {
                let mut unit = Unit::from(unit.clone());
                let tag = unit.tag;

                unit.last_seen_game_loop = self.get_game_loop();

                self.units.insert(tag, unit);
            }

            // get camera data

            //TODO the rest
        }

        Ok(())
    }
}
