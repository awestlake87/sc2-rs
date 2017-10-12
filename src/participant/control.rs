
use std::path::PathBuf;

use sc2_proto::common;
use sc2_proto::sc2api;

use super::{ Participant };
use super::super::{ Result, Error };
use super::super::game::{ GameSettings, Map, Tag };
use super::super::player::{ Player, PlayerKind, Race, Difficulty };

pub trait Control {
    fn save_map(&mut self, data: Vec<u8>, remote_path: PathBuf) -> Result<()>;
    fn create_game(&mut self, settings: &GameSettings, players: &Vec<Player>)
        -> Result<()>
    ;
    fn join_game(&mut self) -> Result<()>;
    fn leave_game(&mut self) -> Result<()>;

    fn step(&mut self, count: usize) -> Result<()>;

    fn save_replay(&mut self, path: PathBuf) -> Result<()>;

    fn issue_events(&mut self, commands: Vec<Tag>) -> Result<()>;

    fn quit(&mut self) -> Result<()>;
}

impl Control for Participant {
    fn save_map(&mut self, data: Vec<u8>, remote_path: PathBuf) -> Result<()> {
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

    fn step(&mut self, count: usize) -> Result<()> {
        unimplemented!("step");
    }

    fn save_replay(&mut self, path: PathBuf) -> Result<()> {
        unimplemented!("save replay");
    }

    fn issue_events(&mut self, commands: Vec<Tag>) -> Result<()> {
        unimplemented!("issue events");
    }

    fn quit(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_quit();

        self.send(req)
    }
}
