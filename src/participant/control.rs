
use std::collections::HashSet;
use std::path::PathBuf;

use sc2_proto::common;
use sc2_proto::sc2api;

use super::{ Participant };
use super::super::{ Result, Error };
use super::super::data::{
    GameSettings,
    Map,
    Player,
    PlayerKind,
    Race,
    Difficulty,
    Alliance,
    DisplayType
};

pub trait Control {
    fn save_map(&mut self, data: Vec<u8>, remote_path: PathBuf) -> Result<()>;
    fn create_game(&mut self, settings: &GameSettings, players: &Vec<Player>)
        -> Result<()>
    ;
    fn join_game(&mut self) -> Result<()>;
    fn leave_game(&mut self) -> Result<()>;

    fn step(&mut self, count: usize) -> Result<()>;

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
        &mut self, settings: &GameSettings, players: &Vec<Player>
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

            match player.kind {
                PlayerKind::Computer => {
                    let difficulty = match player.difficulty {
                        Some(difficulty) => difficulty,
                        None => return Err(
                            Error::Todo("computer must have difficulty")
                        )
                    };

                    use sc2_proto::sc2api::Difficulty as Diff;

                    setup.set_field_type(sc2api::PlayerType::Computer);

                    setup.set_difficulty(
                        match difficulty {
                            Difficulty::VeryEasy        => Diff::VeryEasy,
                            Difficulty::Easy            => Diff::Easy,
                            Difficulty::Medium          => Diff::Medium,
                            Difficulty::MediumHard      => Diff::MediumHard,
                            Difficulty::Hard            => Diff::Hard,
                            Difficulty::Harder          => Diff::Harder,
                            Difficulty::VeryHard        => Diff::VeryHard,
                            Difficulty::CheatVision     => Diff::CheatVision,
                            Difficulty::CheatMoney      => Diff::CheatMoney,
                            Difficulty::CheatInsane     => Diff::CheatInsane
                        }
                    );
                },
                PlayerKind::Participant => {
                    setup.set_field_type(sc2api::PlayerType::Participant);
                },
                PlayerKind::Observer => {
                    setup.set_field_type(sc2api::PlayerType::Observer);
                }
            }

            match player.race {
                Some(race) => setup.set_race(
                    match race {
                        Race::Zerg      => common::Race::Zerg,
                        Race::Terran    => common::Race::Terran,
                        Race::Protoss   => common::Race::Protoss
                    }
                ),
                None => ()
            };

            req.mut_create_game().mut_player_setup().push(setup);
        }

        req.mut_create_game().set_realtime(true);

        self.send(req)?;
        let rsp = self.recv()?;

        println!("create game rsp: {:#?}", rsp);

        Ok(())
    }

    fn join_game(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        {
            let join_game = &mut req.mut_join_game();

            match self.player.race {
                Some(race) => join_game.set_race(
                    match race {
                        Race::Zerg      => common::Race::Zerg,
                        Race::Terran    => common::Race::Terran,
                        Race::Protoss   => common::Race::Protoss
                    }
                ),
                None => join_game.set_race(common::Race::NoRace)
            };

            let options = &mut join_game.mut_options();

            options.set_raw(true);
            options.set_score(true);
        }

        self.send(req)?;
        let rsp = self.recv()?;

        self.player_id = Some(rsp.get_join_game().get_player_id());

        Ok(())
    }

    fn leave_game(&mut self) -> Result<()> {
        unimplemented!("leave game");
    }

    fn step(&mut self, _: usize) -> Result<()> {
        unimplemented!("step");
    }

    fn save_replay(&mut self, _: PathBuf) -> Result<()> {
        unimplemented!("save replay");
    }

    fn issue_events(&mut self) -> Result<()> {
        if
            self.game_state.current_game_loop ==
            self.game_state.previous_game_loop
        {
            return Ok(())
        }

        self.issue_unit_destroyed_events()?;
        self.issue_unit_added_events()?;

        self.issue_idle_events()?;
        self.issue_building_completed_events()?;

        self.issue_upgrade_events()?;
        self.issue_alert_events()?;

        self.agent.on_step();

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
        if !self.observation.get_observation().has_raw_data() {
            return Ok(())
        }

        let raw = self.observation.get_observation().get_raw_data();
        if raw.has_event() {
            let event = raw.get_event();

            for tag in event.get_dead_units() {
                match self.units.get_mut(tag) {
                    Some(ref mut unit) => {
                        unit.mark_dead();
                        self.agent.on_unit_destroyed(unit);
                    },
                    None => ()
                }
            }
        }

        Ok(())
    }
    fn issue_unit_added_events(&mut self) -> Result<()> {
        for ref mut unit in self.units.values_mut() {
            match self.previous_units.get(&unit.tag) {
                Some(_) => continue,
                None => {
                    if unit.alliance == Alliance::Enemy &&
                        unit.display_type == DisplayType::Visible
                    {
                        self.agent.on_unit_detected(unit);
                    }
                    else {
                        self.agent.on_unit_created(unit);
                    }
                }
            }
        }

        Ok(())
    }
    fn issue_idle_events(&mut self) -> Result<()> {
        for unit in self.units.values() {
            if !unit.orders.is_empty() || unit.build_progress < 1.0 {
                continue;
            }

            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if !prev_unit.orders.is_empty() {
                        self.agent.on_unit_idle(&unit);
                        continue;
                    }

                    if prev_unit.build_progress < 1.0 {
                        self.agent.on_unit_idle(&unit);
                        continue;
                    }

                    for tag in &self.commands {
                        if *tag == unit.tag {
                            self.agent.on_unit_idle(&unit);
                        }
                    }
                },
                None => {
                    self.agent.on_unit_idle(&unit);
                    continue;
                }
            }
        }

        Ok(())
    }
    fn issue_building_completed_events(&mut self) -> Result<()> {
        for unit in self.units.values() {
            if unit.build_progress < 1.0 {
                continue;
            }

            match self.previous_units.get(&unit.tag) {
                Some(ref prev_unit) => {
                    if prev_unit.build_progress < 1.0 {
                        self.agent.on_building_complete(&unit);
                    }
                },
                None => ()
            }
        }

        Ok(())
    }
    fn issue_upgrade_events(&mut self) -> Result<()> {
        let mut prev_upgrades = HashSet::new();

        for upgrade in &self.previous_upgrades {
            prev_upgrades.insert(upgrade);
        }

        for upgrade in &self.upgrades {
            match prev_upgrades.get(&upgrade) {
                Some(_) => (),
                None => {
                    self.agent.on_upgrade_complete(*upgrade);
                }
            }
        }

        Ok(())
    }
    fn issue_alert_events(&mut self) -> Result<()> {
        for alert in self.observation.get_observation().get_alerts() {
            match *alert {
                sc2api::Alert::NuclearLaunchDetected => {
                    self.agent.on_nuke_detected();
                },
                sc2api::Alert::NydusWormDetected => {
                    self.agent.on_nydus_detected();
                },
                _ => continue
            }
        }

        Ok(())
    }
}
