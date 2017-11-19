
use std::rc::Rc;

use rand::random;

use sc2::data::{
    Tag, Vector2, Point2, Alliance, UnitType, Ability, ActionTarget
};
use sc2::{ Agent, Result, FrameData, Command };

const TARGET_SCV_COUNT: usize = 15;

pub struct TerranBot {
}

impl TerranBot {
    pub fn new() -> Self {
        Self { }
    }

    fn find_enemy_structure(&self, frame: &FrameData) -> Option<Tag> {
        let units = frame.state.filter_units(
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

    fn find_enemy_pos(&self, frame: &FrameData) -> Option<Point2> {
        if frame.data.terrain_info.enemy_start_locations.is_empty() {
            None
        }
        else {
            //TODO: should be random I think
            Some(frame.data.terrain_info.enemy_start_locations[0])
        }
    }

    fn scout_with_marines(&mut self, frame: &FrameData) -> Option<Command> {
        let units = frame.state.filter_units(
            |u| u.alliance == Alliance::Domestic &&
                u.unit_type == UnitType::TerranMarine &&
                u.orders.is_empty()
        );

        for ref u in units {
            match self.find_enemy_structure(frame) {
                Some(enemy_tag) => {
                    return Some(
                        Command::Action {
                            units: vec![ Rc::clone(u) ],
                            ability: Ability::Attack,
                            target: Some(ActionTarget::UnitTag(enemy_tag))
                        }
                    )
                },
                None => ()
            }

            match self.find_enemy_pos(frame) {
                Some(target_pos) => {
                    return Some(
                        Command::Action {
                            units: vec![ Rc::clone(u) ],
                            ability: Ability::Smart,
                            target: Some(ActionTarget::Location(target_pos))
                        }
                    )
                },
                None => ()
            }
        }

        None
    }

    fn try_build_supply_depot(&mut self, frame: &FrameData) -> Option<Command>
    {
        // if we are not supply capped, don't build a supply depot
        if frame.state.food_used + 2 <= frame.state.food_cap {
            return None
        }

        // find a random SVC to build a depot
        self.try_build_structure(frame, Ability::BuildSupplyDepot)
    }

    fn try_build_scv(&mut self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();

        if scv_count < TARGET_SCV_COUNT {
            self.try_build_unit(
                frame, Ability::TrainScv, UnitType::TerranCommandCenter
            )
        }
        else {
            None
        }
    }

    fn try_build_barracks(&mut self, frame: &FrameData) -> Option<Command> {
        let scv_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranScv
        ).len();
        // wait until we have our quota of SCVs
        if scv_count < TARGET_SCV_COUNT {
            return None
        }

        let barracks_count = frame.state.filter_units(
            |u| u.unit_type == UnitType::TerranBarracks
        ).len();

        if barracks_count > 0 {
            return None
        }

        self.try_build_structure(frame, Ability::BuildBarracks)
    }

    fn try_build_marine(&mut self, frame: &FrameData) -> Option<Command> {
        self.try_build_unit(
            frame, Ability::TrainMarine, UnitType::TerranBarracks
        )
    }

    fn try_build_unit(
        &mut self, frame: &FrameData, ability: Ability, unit_type: UnitType
    )
        -> Option<Command>
    {
        let units = frame.state.filter_units(
            |u| u.unit_type == unit_type && u.orders.is_empty()
        );

        if units.is_empty() {
            None
        }
        else {
            Some(
                Command::Action {
                    units: vec![ Rc::clone(&units[0]) ],
                    ability: ability,
                    target: None
                }
            )
        }
    }

    fn try_build_structure(&mut self, frame: &FrameData, ability: Ability)
        -> Option<Command>
    {
        let units = frame.state.filter_units(
            |u| u.alliance == Alliance::Domestic
        );

        // if a unit is already building this structure, do nothing
        for u in &units {
            for o in &u.orders {
                if o.ability == ability {
                    return None
                }
            }
        }

        if !units.is_empty() {
            let r = Vector2::new(random(), random());

            let u = random::<usize>() % units.len();

            Some(
                Command::Action {
                    units: vec![ Rc::clone(&units[u]) ],
                    ability: ability,
                    target: Some(
                        ActionTarget::Location(
                            Point2::new(units[u].pos.x, units[u].pos.y)
                            + r * 5.0
                        )
                    )
                }
            )
        }
        else {
            None
        }
    }
}

impl Agent for TerranBot {
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        println!("game started");

        Ok(vec![ ])
    }

    fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        let mut commands = vec![ ];
        // if there are marines and the command center is not found, send them
        // scouting.
        if let Some(cmd) = self.scout_with_marines(&frame) {
            commands.push(cmd);
        }

        // build supply depots if they are needed
        if let Some(cmd) = self.try_build_supply_depot(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // build terran SCV's if they are needed
        if let Some(cmd) = self.try_build_scv(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // build barracks if they are ready to be built
        if let Some(cmd) = self.try_build_barracks(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        // just keep building marines if possible
        if let Some(cmd) = self.try_build_marine(&frame) {
            commands.push(cmd);
            return Ok(commands)
        }

        Ok(commands)
    }

    fn stop(&mut self, _: FrameData) -> Result<()> {
        println!("game ended");

        Ok(())
    }
}
