
use num::Float;

use sc2::data::{ Tag, Point2, UnitType, Alliance, Ability, Unit };
use sc2::{ Agent, Participant, Observer, Actions };

use na::{ distance, distance_squared, normalize };

pub struct MarineMicroBot {
    targeted_zergling:      Option<Tag>,
    move_back:              bool,
    backup_target:          Option<Point2>,
    backup_start:           Option<Point2>,
}

impl MarineMicroBot {
    pub fn new() -> Self {
        Self {
            targeted_zergling: None,
            move_back: false,
            backup_target: None,
            backup_start: None,
        }
    }
}

impl Agent for MarineMicroBot {
    fn on_game_start(&mut self, _: &mut Participant) {
        self.move_back = false;
        self.targeted_zergling = None;
    }

    fn on_step(&mut self, game: &mut Participant) {
        let mp = match get_position(
            game, UnitType::TerranMarine, Alliance::Domestic
        ) {
            Some(pos) => pos,
            None => return
        };

        self.targeted_zergling = get_nearest_zergling(game, mp);

        let units = game.filter_units(
            |unit| match unit.alliance {
                Alliance::Domestic => match unit.unit_type {
                    UnitType::TerranMarine => true,
                    _ => false
                },
                _ => false
            }
        );

        if !self.move_back {
            match self.targeted_zergling {
                Some(tag) => game.command_units_to_target(
                    &units, Ability::Attack, tag
                ),
                None => ()
            }
        }
        else {
            let target = match self.backup_target {
                Some(target) => target,
                None => return
            };

            if distance(&mp, &target) < 1.5 {
                self.move_back = false;
            }

            game.command_units_to_location(&units, Ability::Smart, target);
        }
    }

    fn on_unit_destroyed(&mut self, game: &mut Participant, unit: &Unit) {
        match self.targeted_zergling {
            Some(tag) => {
                if unit.tag == tag {
                    let mp = match get_position(
                        game, UnitType::TerranMarine, Alliance::Domestic
                    ) {
                        Some(pos) => pos,
                        None => return
                    };
                    let zp = match get_position(
                        game, UnitType::ZergZergling, Alliance::Enemy
                    ) {
                        Some(pos) => pos,
                        None => return
                    };

                    let direction = normalize(&(mp - zp));

                    self.targeted_zergling = None;
                    self.move_back = true;
                    self.backup_start = Some(mp);
                    self.backup_target = Some(mp + direction * 3.0);
                }
            },
            None => ()
        }
    }
}

fn get_position(
    game: &mut Participant, unit_type: UnitType, alliance: Alliance
)
    -> Option<Point2>
{
    let units = game.filter_units(
        |u| u.alliance == alliance && u.unit_type == unit_type
    );

    let mut pos = Point2::new(0.0, 0.0);

    for u in &units {
        pos = Point2::new(pos.x + u.pos.x, pos.y + u.pos.y);
    }

    if units.len() > 0 {
        Some(pos / (units.len() as f32))
    }
    else {
        None
    }
}

fn get_nearest_zergling(game: &mut Participant, from: Point2)
    -> Option<Tag>
{
    let units = game.filter_units(
        |u| u.alliance == Alliance::Enemy &&
            u.unit_type == UnitType::ZergZergling
    );

    let mut tag = None;
    let mut distance = f32::max_value();
    for u in &units {
        let d = distance_squared(&Point2::new(u.pos.x, u.pos.y), &from);
        if d < distance {
            distance = d;
            tag = Some(u.tag);
        }
    }

    tag
}
