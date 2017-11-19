
use na::{ distance, distance_squared, normalize };
use num::Float;
use sc2::data::{
    Tag, Point2, UnitType, Alliance, Ability, ActionTarget
};
use sc2::{ Agent, Result, FrameData, Command, GameEvent };

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
    fn start(&mut self, _: FrameData) -> Result<Vec<Command>> {
        self.move_back = false;
        self.targeted_zergling = None;

        Ok(vec![ ])
    }

    fn update(&mut self, frame: FrameData) -> Result<Vec<Command>> {
        for e in &frame.events {
            if let (&GameEvent::UnitDestroyed(ref unit), Some(tag)) = (
                e, self.targeted_zergling
            ) {
                if unit.tag == tag {
                    let mp = match get_position(
                        &frame, UnitType::TerranMarine, Alliance::Domestic
                    ) {
                        Some(pos) => pos,
                        None => return Ok(vec![ ])
                    };
                    let zp = match get_position(
                        &frame, UnitType::ZergZergling, Alliance::Enemy
                    ) {
                        Some(pos) => pos,
                        None => return Ok(vec![ ])
                    };

                    let direction = normalize(&(mp - zp));

                    self.targeted_zergling = None;
                    self.move_back = true;
                    self.backup_start = Some(mp);
                    self.backup_target = Some(mp + direction * 3.0);
                }
            }
        }

        let mp = match get_position(
            &frame, UnitType::TerranMarine, Alliance::Domestic
        ) {
            Some(pos) => pos,
            None => return Ok(vec![ ])
        };

        self.targeted_zergling = get_nearest_zergling(&frame, mp);

        let units = frame.state.filter_units(
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
                Some(tag) => Ok(
                    vec![
                        Command::Action {
                            units: units,
                            ability: Ability::Attack,
                            target: Some(ActionTarget::UnitTag(tag))
                        }
                    ]
                ),
                None => Ok(vec![ ])
            }
        }
        else {
            let target = match self.backup_target {
                Some(target) => target,
                None => return Ok(vec![ ])
            };

            if distance(&mp, &target) < 1.5 {
                self.move_back = false;
            }

            Ok(
                vec![
                    Command::Action {
                        units: units,
                        ability: Ability::Smart,
                        target: Some(ActionTarget::Location(target))
                    }
                ]
            )
        }
    }
}

fn get_position(frame: &FrameData, unit_type: UnitType, alliance: Alliance)
    -> Option<Point2>
{
    let units = frame.state.filter_units(
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

fn get_nearest_zergling(frame: &FrameData, from: Point2) -> Option<Tag> {
    let units = frame.state.filter_units(
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
