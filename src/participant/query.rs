
use super::{ Participant };
use super::super::{ Result };
use super::super::data::{ Tag, Point2, Ability, Unit, AvailableAbility };

pub enum PathStart {
    UnitTag(Tag),
    Position(Point2),
}

pub struct PathingQuery {
    pub start:              PathStart,
    pub end:                Point2
}

pub struct PlacementQuery {
    pub ability:            Ability,
    pub target_pos:         Point2,
    pub placing_unit_tag:   Option<Tag>,
}

pub trait Query {
    fn query_abilities(
        &mut self, units: &Vec<Unit>, ignore_resource_requirements: bool
    )
        -> Result<Vec<Vec<AvailableAbility>>>
    ;

    fn query_pathing(&mut self, queries: &Vec<PathingQuery>)
        -> Result<Vec<f32>>
    ;

    fn query_placement(&mut self, queries: &Vec<PlacementQuery>)
        -> Result<Vec<bool>>
    ;
}

impl Query for Participant {
    fn query_abilities(
        &mut self, _: &Vec<Unit>, _: bool
    )
        -> Result<Vec<Vec<AvailableAbility>>>
    {
        unimplemented!("query abilities");
    }

    fn query_pathing(&mut self, _: &Vec<PathingQuery>)
        -> Result<Vec<f32>>
    {
        unimplemented!("query pathing");
    }

    fn query_placement(&mut self, _: &Vec<PlacementQuery>)
        -> Result<Vec<bool>>
    {
        unimplemented!("query placement");
    }
}
