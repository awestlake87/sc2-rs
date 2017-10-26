
use std::rc::Rc;

use rand::random;

use sc2::data::{ Tag, Vector2, Point2, GameInfo, Alliance, UnitType, Ability };
use sc2::agent::{ Agent };
use sc2::participant::{ Participant, Observer, Actions };

const TARGET_SCV_COUNT: usize = 15;

pub struct TerranBot {
    game_info:          Option<GameInfo>
}

impl TerranBot {
    pub fn new() -> Self {
        Self {
            game_info: None
        }
    }

    fn find_enemy_structure(&self, p: &mut Participant) -> Option<Tag> {
        let units = p.filter_units(
            |u| u.alliance == Alliance::Enemy && (
                u.unit_type == UnitType::TerranCommandCenter ||
                u.unit_type == UnitType::TerranSupplyDepot ||
                u.unit_type == UnitType::TerranBarracks
            )
        );

        if !units.is_empty() {
            Some(units[0].tag)
        }
        else {
            None
        }
    }

    fn find_enemy_pos(&self, _: &mut Participant) -> Option<Point2> {
        match self.game_info {
            Some(ref game_info) => {
                if game_info.enemy_start_locations.is_empty() {
                    None
                }
                else {
                    //TODO: should be random I think
                    Some(game_info.enemy_start_locations[0])
                }
            },
            None => None
        }
    }

    fn scout_with_marines(&mut self, p: &mut Participant) {
        let units = p.filter_units(
            |u| u.alliance == Alliance::Domestic &&
                u.unit_type == UnitType::TerranMarine &&
                u.orders.is_empty()
        );

        for ref u in units {
            match self.find_enemy_structure(p) {
                Some(enemy_tag) => {
                    p.command_units_to_target(
                        &vec![ Rc::clone(u) ], Ability::Attack, enemy_tag
                    );
                },
                None => ()
            }

            match self.find_enemy_pos(p) {
                Some(target_pos) => {
                    p.command_units_to_location(
                        &vec![ Rc::clone(u) ], Ability::Smart, target_pos
                    );
                },
                None => ()
            }
        }
    }

    fn try_build_supply_depot(&mut self, p: &mut Participant) -> bool {
        // if we are not supply capped, don't build a supply depot
        if p.get_food_used() + 2 <= p.get_food_cap() {
            return false
        }

        // find a random SVC to build a depot
        self.try_build_structure(p, Ability::BuildSupplyDepot)
    }

    fn try_build_scv(&mut self, p: &mut Participant) -> bool {
        let scv_count = p.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();

        if scv_count < TARGET_SCV_COUNT {
            self.try_build_unit(
                p, Ability::TrainScv, UnitType::TerranCommandCenter
            )
        }
        else {
            false
        }
    }

    fn try_build_barracks(&mut self, p: &mut Participant) -> bool {
        let scv_count = p.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return false
        }

        let barracks_count = p.filter_units(
            |u| u.unit_type == UnitType::TerranBarracks
        ).len();

        if barracks_count > 0 {
            return false
        }

        self.try_build_structure(p, Ability::BuildBarracks)
    }

    fn try_build_marine(&mut self, p: &mut Participant) -> bool {
        self.try_build_unit(
            p, Ability::TrainMarine, UnitType::TerranBarracks
        )
    }

    fn try_build_unit(
        &mut self, p: &mut Participant, ability: Ability, unit_type: UnitType
    )
        -> bool
    {
        let units = p.filter_units(
            |u| u.unit_type == unit_type && u.orders.is_empty()
        );

        if units.is_empty() {
            false
        }
        else {
            p.command_units(&vec![ Rc::clone(&units[0]) ], ability);
            true
        }
    }

    fn try_build_structure(&mut self, p: &mut Participant, ability: Ability)
        -> bool
    {
        let units = p.filter_units(|u| u.alliance == Alliance::Domestic);

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in &u.orders {
                if o.ability == ability {
                    return false
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            p.command_units_to_location(
                &vec![ Rc::clone(&units[u]) ],
                ability,
                Point2::new(units[u].pos.x, units[u].pos.y) + r * 5.0
            );

            true
        }
        else {
            false
        }
    }
}

impl Agent for TerranBot {
    fn on_game_start(&mut self, p: &mut Participant) {
        self.game_info = match p.get_game_info() {
            Ok(info) => Some(info.clone()),
            Err(e) => {
                eprintln!("unable to fetch game info {}", e);
                return
            }
        };

        println!("game started");
    }

    fn on_step(&mut self, p: &mut Participant) {
        // if there are marines and the command center is not found, send them
        // scouting.
        self.scout_with_marines(p);

        // build supply depots if they are needed
        if self.try_build_supply_depot(p) {
            return
        }

        // build terran SCV's if they are needed
        if self.try_build_scv(p) {
            return
        }

        // build barracks if they are ready to be built
        if self.try_build_barracks(p) {
            return
        }

        // just keep building marines if possible
        if self.try_build_marine(p) {
            return
        }
    }

    fn on_game_end(&mut self, _: &mut Participant) {
        println!("game ended");
    }
}
