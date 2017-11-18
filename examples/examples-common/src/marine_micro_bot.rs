
use std::rc::Rc;

use na::{ distance, distance_squared, normalize };
use num::Float;
use sc2::data::{
    Tag, Point2, UnitType, Alliance, Ability, Unit, ActionTarget
};
use sc2::{ Agent, Participant, Actions, Result };

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
    fn on_game_start(&mut self, _: &mut Participant) -> Result<()> {
        self.move_back = false;
        self.targeted_zergling = None;

        Ok(())
    }

    fn on_step(&mut self, p: &mut Participant) -> Result<()> {
        let mp = match get_position(
            p, UnitType::TerranMarine, Alliance::Domestic
        )? {
            Some(pos) => pos,
            None => return Ok(())
        };

        self.targeted_zergling = get_nearest_zergling(p, mp)?;

        let units = p.get_game_state()?.filter_units(
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
                Some(tag) => p.command_units(
                    &units, Ability::Attack, Some(ActionTarget::UnitTag(tag))
                ),
                None => ()
            }
        }
        else {
            let target = match self.backup_target {
                Some(target) => target,
                None => return Ok(())
            };

            if distance(&mp, &target) < 1.5 {
                self.move_back = false;
            }

            p.command_units(
                &units, Ability::Smart, Some(ActionTarget::Location(target))
            );
        }

        Ok(())
    }

    fn on_unit_destroyed(&mut self, game: &mut Participant, unit: &Rc<Unit>)
        -> Result<()>
    {
        match self.targeted_zergling {
            Some(tag) => {
                if unit.tag == tag {
                    let mp = match get_position(
                        game, UnitType::TerranMarine, Alliance::Domestic
                    )? {
                        Some(pos) => pos,
                        None => return Ok(())
                    };
                    let zp = match get_position(
                        game, UnitType::ZergZergling, Alliance::Enemy
                    )? {
                        Some(pos) => pos,
                        None => return Ok(())
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

        Ok(())
    }
}

fn get_position(
    p: &mut Participant, unit_type: UnitType, alliance: Alliance
)
    -> Result<Option<Point2>>
{
    let units = p.get_game_state()?.filter_units(
        |u| u.alliance == alliance && u.unit_type == unit_type
    );

    let mut pos = Point2::new(0.0, 0.0);

    for u in &units {
        pos = Point2::new(pos.x + u.pos.x, pos.y + u.pos.y);
    }

    if units.len() > 0 {
        Ok(Some(pos / (units.len() as f32)))
    }
    else {
        Ok(None)
    }
}

fn get_nearest_zergling(p: &mut Participant, from: Point2)
    -> Result<Option<Tag>>
{
    let units = p.get_game_state()?.filter_units(
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

    Ok(tag)
}
