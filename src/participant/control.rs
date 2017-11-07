
use std::collections::HashSet;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use sc2_proto::common;
use sc2_proto::sc2api;

use super::super::{ Result, Error };
use super::super::agent::Agent;
use super::super::data::{
    GameSettings,
    GamePorts,
    Map,
    PlayerSetup,
    Alliance,
    DisplayType
};
use super::{ Participant, AppState, Observer };

pub trait Control {
    fn save_map(&mut self, data: Vec<u8>, remote_path: PathBuf) -> Result<()>;
    fn create_game(
        &mut self,
        settings: &GameSettings,
        players: &Vec<PlayerSetup>,
        is_realtime: bool
    )
        -> Result<()>
    ;
    fn req_join_game(&mut self, ports: &Option<GamePorts>) -> Result<()>;
    fn await_join_game(&mut self) -> Result<()>;

    fn leave_game(&mut self) -> Result<()>;

    fn req_step(&mut self, count: usize) -> Result<()>;
    fn await_step(&mut self) -> Result<()>;

    fn save_replay(&mut self, path: PathBuf) -> Result<()>;

    fn issue_events(&mut self) -> Result<()>;

    fn quit(&mut self) -> Result<()>;
}

trait InnerControl {
    fn issue_unit_destroyed_events(&mut self) -> Result<()>;
    fn issue_unit_added_events(&mut self) -> Result<()>;
    fn issue_idle_events(&mut self) -> Result<()>;
    fn issue_building_completed_events(&mut self) -> Result<()>;
    fn issue_upgrade_events(&mut self) -> Result<()>;
    fn issue_alert_events(&mut self) -> Result<()>;
}

impl Control for Participant {
    fn save_map(&mut self, _: Vec<u8>, _: PathBuf) -> Result<()> {
        unimplemented!("save map");
    }
    fn create_game(
        &mut self,
        settings: &GameSettings,
        players: &Vec<PlayerSetup>,
        is_realtime: bool
    )
        -> Result<()>
    {
        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(ref path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.clone().into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => return Err(
                            Error::Todo("invalid path string")
                        )
                    }
                );
            },
            Map::BlizzardMap(ref map) => {
                req.mut_create_game().set_battlenet_map_name(map.clone());
            }
        };

        for player in players {
            let mut setup = sc2api::PlayerSetup::new();

            match player {
                &PlayerSetup::Computer { ref difficulty, ref race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(difficulty.to_proto());
                    setup.set_race(race.to_proto());
                },
                &PlayerSetup::Player { ref race, .. } => {
                    setup.set_field_type(sc2api::PlayerType::Participant);

                    setup.set_race(race.to_proto());
                },
                &PlayerSetup::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(is_realtime);

        self.send(req)?;
        let rsp = self.recv()?;

        println!("create game rsp: {:#?}", rsp);

        Ok(())
    }

    fn req_join_game(&mut self, ports: &Option<GamePorts>) -> Result<()> {
        let mut req = sc2api::Request::new();

        match self.player {
            PlayerSetup::Computer { race, .. } => {
                req.mut_join_game().set_race(race.to_proto());
            },
            PlayerSetup::Player { race, .. } => {
                req.mut_join_game().set_race(race.to_proto());
            },
            _ => req.mut_join_game().set_race(common::Race::NoRace)
        };

        match ports {
            &Some(ref ports) => {
                req.mut_join_game().set_shared_port(ports.shared_port as i32);

                {
                    let s = req.mut_join_game().mut_server_ports();

                    s.set_game_port(ports.server_ports.game_port as i32);
                    s.set_base_port(ports.server_ports.base_port as i32);
                }

                {
                    let client_ports = req.mut_join_game().mut_client_ports();

                    for c in &ports.client_ports {
                        let mut p = sc2api::PortSet::new();

                        p.set_game_port(c.game_port as i32);
                        p.set_base_port(c.base_port as i32);

                        client_ports.push(p);
                    }
                }
            },
            &None => (),
        }

        {
            let options = req.mut_join_game().mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        self.send(req)?;

        Ok(())
    }

    fn await_join_game(&mut self) -> Result<()> {
        let rsp = self.recv()?;

        println!("recv: {:#?}", rsp);

        self.player_id = Some(rsp.get_join_game().get_player_id());

        Ok(())
    }

    fn leave_game(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_leave_game();

        self.send(req)?;

        let rsp = self.recv()?;

        println!("recv: {:#?}", rsp);

        Ok(())
    }

    fn req_step(&mut self, count: usize) -> Result<()> {
        if self.get_app_state() != AppState::Normal {
            return Err(Error::Todo("app is in bad state"))
        }

        let mut req = sc2api::Request::new();

        req.mut_step().set_count(count as u32);

        self.send(req)?;

        Ok(())
    }

    fn await_step(&mut self) -> Result<()> {
        let rsp = self.recv()?;

        if !rsp.has_step() || rsp.get_error().len() > 0 {
            return Err(Error::Todo("step error"))
        }

        self.update_observation()?;

        Ok(())
    }

    fn save_replay(&mut self, _: PathBuf) -> Result<()> {
        unimplemented!("save replay");
    }

    fn issue_events(&mut self) -> Result<()> {
        if self.get_game_loop() == self.game_state.previous_game_loop {
            return Ok(())
        }

        self.issue_unit_destroyed_events()?;
        self.issue_unit_added_events()?;

        self.issue_idle_events()?;
        self.issue_building_completed_events()?;

        self.issue_upgrade_events()?;
        self.issue_alert_events()?;

        self.on_step();

        Ok(())
    }

    fn quit(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_quit();

        self.send(req)
    }
}

impl InnerControl for Participant {
    fn issue_unit_destroyed_events(&mut self) -> Result<()> {
        let mut destroyed_units = vec![ ];

        if !self.observation.get_observation().has_raw_data() {
            return Ok(())
        }

        {
            let raw = self.observation.get_observation().get_raw_data();

            if raw.has_event() {
                let event = raw.get_event();

                for tag in event.get_dead_units() {
                    match self.previous_units.get_mut(tag) {
                        Some(ref mut unit) => {
                            Rc::get_mut(unit).unwrap().mark_dead();
                            destroyed_units.push(Rc::clone(unit));
                        },
                        None => ()
                    }
                }
            }
        }

        for ref u in destroyed_units {
            self.on_unit_destroyed(u);
        }

        Ok(())
    }
    fn issue_unit_added_events(&mut self) -> Result<()> {
        let mut created_units = vec![ ];
        let mut detected_units = vec![ ];

        for ref mut unit in self.units.values_mut() {
            match self.previous_units.get(&unit.tag) {
                Some(_) => continue,
                None => {
                    if unit.alliance == Alliance::Enemy &&
                        unit.display_type == DisplayType::Visible
                    {
                        detected_units.push(Rc::clone(unit));
                    }
                    else {
                        created_units.push(Rc::clone(unit));
                    }
                }
            }
        }

        for ref u in detected_units {
            self.on_unit_detected(u);
        }

        for ref u in created_units {
            self.on_unit_created(u);
        }

        Ok(())
    }
    fn issue_idle_events(&mut self) -> Result<()> {
        let mut idle_units = vec![ ];

        for unit in self.units.values() {
            if !unit.orders.is_empty() || unit.build_progress < 1.0 {
                continue;
            }

            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if !prev_unit.orders.is_empty() {
                        idle_units.push(Rc::clone(unit));
                        continue;
                    }

                    if prev_unit.build_progress < 1.0 {
                        idle_units.push(Rc::clone(unit));
                        continue;
                    }

                    for tag in &self.commands {
                        if *tag == unit.tag {
                            idle_units.push(Rc::clone(unit));
                        }
                    }
                },
                None => {
                    idle_units.push(Rc::clone(unit));
                    continue;
                }
            }
        }

        for ref u in idle_units {
            self.on_unit_idle(u);
        }

        Ok(())
    }
    fn issue_building_completed_events(&mut self) -> Result<()> {
        let mut building_completed_units = vec![ ];

        for unit in self.units.values() {
            if unit.build_progress < 1.0 {
                continue;
            }

            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if prev_unit.build_progress < 1.0 {
                        building_completed_units.push(Rc::clone(unit));
                    }
                },
                None => ()
            }
        }

        for ref u in building_completed_units {
            self.on_building_complete(u);
        }

        Ok(())
    }
    fn issue_upgrade_events(&mut self) -> Result<()> {
        let mut prev_upgrades = HashSet::new();
        let mut new_upgrades = vec![ ];

        for upgrade in &self.previous_upgrades {
            prev_upgrades.insert(*upgrade);
        }

        for upgrade in &self.upgrades {
            match prev_upgrades.get(upgrade) {
                Some(_) => (),
                None => {
                    new_upgrades.push(*upgrade);
                }
            }
        }

        for u in new_upgrades {
            self.on_upgrade_complete(u);
        }

        Ok(())
    }
    fn issue_alert_events(&mut self) -> Result<()> {
        let mut nukes = 0;
        let mut nydus_worms = 0;

        for alert in self.observation.get_observation().get_alerts() {
            match *alert {
                sc2api::Alert::NuclearLaunchDetected => nukes += 1,
                sc2api::Alert::NydusWormDetected => nydus_worms += 1
            }
        }

        for _ in 0..nukes {
            self.on_nuke_detected();
        }
        for _ in 0..nydus_worms {
            self.on_nydus_detected();
        }

        Ok(())
    }
}
