
use sc2_proto::common;
use sc2_proto::sc2api;
use sc2_proto::sc2api::{ Response };

use super::{ Client };
use super::super::{ Result, Error };
use super::super::game::{ GameSettings, Map };
use super::super::player::{ Player, PlayerKind, Race, Difficulty };

pub trait Control {
    fn quit(&mut self) -> Result<()>;
    fn create_game(&mut self, settings: GameSettings, players: Vec<Player>)
        -> Result<Response>
    ;
}

impl Control for Client {
    fn quit(&mut self) -> Result<()> {
        let mut req = sc2api::Request::new();

        req.mut_quit();

        self.send(req)
    }

    fn create_game(
        &mut self, settings: GameSettings, players: Vec<Player>
    )
        -> Result<Response>
    {
        let mut req = sc2api::Request::new();

        match settings.map {
            Map::LocalMap(path) => {
                req.mut_create_game().mut_local_map().set_map_path(
                    match path.into_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => return Err(
                            Error::Todo("invalid path string")
                        )
                    }
                );
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

        self.call(req)
    }
}
