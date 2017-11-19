
use std::collections::{ HashSet, HashMap };
use std::mem;
use std::rc::Rc;

use sc2_proto::sc2api;

use super::super::{ Result, FromProto, IntoSc2 };
use data::{
    PowerSource,
    TerrainInfo,
    Unit,
    Upgrade,
    Point2,
    Score,
    UnitType,
    UnitTypeData,
    Effect,
    Ability,
    AbilityData,
    UpgradeData,
    Buff,
    BuffData,
    Visibility,
    Alliance,
    DisplayType
};
use participant::{ Participant, AppState };

/// an event from the game
pub enum GameEvent {
    /// a unit was destroyed
    UnitDestroyed(Rc<Unit>),
    /// a unit was created
    UnitCreated(Rc<Unit>),
    /// a unit does not have any orders
    UnitIdle(Rc<Unit>),
    /// a unit was detected
    UnitDetected(Rc<Unit>),

    /// an upgrade completed
    UpgradeCompleted(Upgrade),
    /// a unit finished constructing a building
    BuildingCompleted(Rc<Unit>),

    /// number of nydus worms detected
    NydusWormsDetected(u32),
    /// number of nukes launched
    NukesDetected(u32),
}

/// game data (may vary depending on version and DLC)
pub struct GameData {
    /// data associated with abilities
    pub ability_data:               HashMap<Ability, AbilityData>,
    /// data associated with unit types
    pub unit_type_data:             HashMap<UnitType, UnitTypeData>,
    /// data associated with upgrades
    pub upgrade_data:               HashMap<Upgrade, UpgradeData>,
    /// data associated buffs
    pub buff_data:                  HashMap<Buff, BuffData>,

    /// playable area info
    pub terrain_info:               TerrainInfo,
}

/// state of the game (changes every frame)
pub struct GameState {
    /// the player id associated with the participant
    pub player_id:                  u32,
    /// the previous game step
    pub previous_step:              u32,
    /// the current game step
    pub current_step:               u32,
    /// position of the center of the camera
    pub camera_pos:                 Point2,

    /// a list of all known units at the moment
    pub units:                      Vec<Rc<Unit>>,

    /// all power sources associated with the current player
    pub power_sources:              Vec<PowerSource>,
    /// all active effects in vision of the current player
    pub effects:                    Vec<Effect>,
    /// all upgrades
    pub upgrades:                   Vec<Upgrade>,

    /// current mineral count
    pub minerals:                   u32,
    /// current vespene count
    pub vespene:                    u32,
    /// the total supply cap given the players max supply
    pub food_cap:                   u32,
    /// the total supply used by the player
    pub food_used:                  u32,
    /// the total supply consumed by army units alone
    pub food_army:                  u32,
    /// the total supply consumed by workers alone
    pub food_workers:               u32,
    /// the number of workers that currently have no orders
    pub idle_worker_count:          u32,
    /// the number of army units
    pub army_count:                 u32,
    /// the number of warp gates owned by the player
    pub warp_gate_count:            u32,
    /// the number of larva owned by the player
    pub larva_count:                u32,

    /// detailed current set of scores
    pub score:                      Score
}

impl GameState {
    pub fn filter_units<F>(&self, filter: F) -> Vec<Rc<Unit>>
        where F: Fn(&Unit) -> bool
    {
        let mut units = vec![ ];

        for unit in &self.units {
            if filter(&unit) {
                units.push(Rc::clone(&unit));
            }
        }

        units
    }
    /// check if the given point contains creep
    pub fn has_creep(&self, _: Point2) -> bool {
        unimplemented!("has creep")
    }
    /// get the visibility of the given point for the current player
    pub fn get_visibility(&self, _: Point2) -> Visibility {
        unimplemented!("get visibility")
    }
    /// whether the given point on the terrain is pathable
    ///
    /// this does not include pathing blockers like structures, for more
    /// accurate pathing results, use query interface
    pub fn is_pathable(&self, _: Point2) -> bool {
        unimplemented!("is pathable")
    }
    /// whether the given point on the terrain is buildable
    ///
    /// this does not include blockers like other structures. for more
    /// accurate building placement results, use query interface
    pub fn is_placable(&self, _: Point2) -> bool {
        unimplemented!("is placable")
    }
    /// returns the terrain height of the given point
    pub fn get_terrain_height(&self, _: Point2) -> f32 {
        unimplemented!("get terrain height")
    }
}

/// all game data passed to agents and observers
pub struct FrameData {
    /// state that updates every frame
    pub state: GameState,
    /// data that can change on a per game basis
    pub data: Rc<GameData>,
    /// events that have happened since the last update
    pub events: Vec<GameEvent>
}

/// UNSTABLE observation trait
pub trait Observation {
    /// request a data update
    fn update_data(&mut self) -> Result<()>;
    /// request an observation update
    fn update_observation(&mut self) -> Result<FrameData>;
}

impl Observation for Participant {
    fn update_data(&mut self) -> Result<()> {
        let mut req_data = sc2api::Request::new();
        req_data.mut_data().set_unit_type_id(true);

        self.send(req_data)?;
        let mut rsp_data = self.recv()?;

        let mut req_terrain_info = sc2api::Request::new();
        req_terrain_info.mut_game_info();

        self.send(req_terrain_info)?;
        let mut rsp_terrain_info = self.recv()?;

        let mut game_data = GameData {
            ability_data: HashMap::new(),
            unit_type_data: HashMap::new(),
            upgrade_data: HashMap::new(),
            buff_data: HashMap::new(),

            terrain_info: rsp_terrain_info.take_game_info().into()
        };

        for data in rsp_data.mut_data().take_units().into_iter() {
            let u = UnitTypeData::from_proto(data)?;

            game_data.unit_type_data.insert(u.unit_type, u);
        }

        self.game_data = Some(Rc::from(game_data));

        Ok(())
    }

    fn update_observation(&mut self) -> Result<FrameData> {
        if self.get_app_state() != AppState::Normal {
            unimplemented!("Err - app in bad state");
        }

        let mut req = sc2api::Request::new();
        req.mut_observation();

        self.send(req)?;
        let mut rsp = self.recv()?;

        let mut observation = rsp.take_observation().take_observation();



        self.previous_step = self.current_step;
        self.current_step = observation.get_game_loop();
        let is_new_frame = self.current_step != self.previous_step;

        let player_common = observation.take_player_common();
        let mut raw = observation.take_raw_data();
        let mut player_raw = raw.take_player();

        self.previous_units = mem::replace(&mut self.units, HashMap::new());
        for unit in raw.take_units().into_iter() {
            match Unit::from_proto(unit) {
                Ok(mut unit) => {
                    let tag = unit.tag;

                    unit.last_seen_game_loop = self.current_step;

                    self.units.insert(tag, Rc::from(unit));
                },
                _ => ()
            }
        }

        self.previous_upgrades = mem::replace(
            &mut self.upgrades, HashSet::new()
        );

        for u in player_raw.take_upgrade_ids().into_iter() {
            self.upgrades.insert(Upgrade::from_proto(u)?);
        }

        let new_state = GameState {
            player_id: player_common.get_player_id(),
            previous_step: self.previous_step,
            current_step: self.current_step,
            camera_pos: {
                let camera = player_raw.get_camera();

                Point2::new(camera.get_x(), camera.get_y())
            },

            units: self.units.values().map(|u| Rc::clone(u)).collect(),
            power_sources: {
                let mut power_sources = vec![ ];

                for p in player_raw.take_power_sources().into_iter() {
                    power_sources.push(p.into());
                }

                power_sources
            },
            upgrades: self.upgrades.iter().map(|u| *u).collect(),
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

        let mut events = vec![ ];

        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.previous_units.get(tag) {
                    Some(ref mut unit) => {
                        events.push(GameEvent::UnitDestroyed(Rc::clone(unit)));
                    },
                    None => ()
                }
            }
        }

        for ref unit in self.units.values() {
            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if unit.orders.is_empty() && !prev_unit.orders.is_empty() {
                        events.push(GameEvent::UnitIdle(Rc::clone(unit)));
                    }
                    else if unit.build_progress >= 1.0
                        && prev_unit.build_progress < 1.0
                    {
                        events.push(
                            GameEvent::BuildingCompleted(Rc::clone(unit))
                        );
                    }
                },
                None => {
                    if unit.alliance == Alliance::Enemy &&
                        unit.display_type == DisplayType::Visible
                    {
                        events.push(GameEvent::UnitDetected(Rc::clone(unit)));
                    }
                    else {
                        events.push(GameEvent::UnitCreated(Rc::clone(unit)));
                    }

                    events.push(GameEvent::UnitIdle(Rc::clone(unit)));
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
                    events.push(GameEvent::UpgradeCompleted(*upgrade));
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
            events.push(GameEvent::NukesDetected(nukes));
        }

        if nydus_worms > 0 {
            events.push(GameEvent::NydusWormsDetected(nydus_worms));
        }

        Ok(
            FrameData {
                state: new_state,
                data: self.get_game_data()?,
                events: events
            }
        )
    }
}
