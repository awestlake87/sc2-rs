
use super::{ Result };
use data::{ Tag, Point2, Ability, Unit, AvailableUnitAbilities };
use participant::{ Participant };

/// pathing query starting point
pub enum PathStart {
    /// start from a unit
    UnitTag(Tag),
    /// start from a location
    Position(Point2),
}

/// query for determining the pathing distance between two points
pub struct PathingQuery {
    /// starting point (either a unit or a location)
    pub start:              PathStart,
    /// end point
    pub end:                Point2
}

/// query for determining whether a building can be placed at a location
pub struct PlacementQuery {
    /// ability for building or moving the structure
    pub ability:            Ability,
    /// position to attempt placement on
    pub target_pos:         Point2,
    /// the unit that is performing the placement
    pub placing_unit_tag:   Option<Tag>,
}

/// the query interface
pub trait Query {
    /// returns the available abilities for each unit
    ///
    /// ignore_resource_requirements - ignores food, mineral, and gas costs
    ///     as well as cooldowns
    fn query_unit_abilities(
        &mut self, units: &Vec<Unit>, ignore_resource_requirements: bool
    )
        -> Result<Vec<AvailableUnitAbilities>>
    ;

    /// returns the pathing distance between two locations
    ///
    /// takes into account unit movement properties (eg. flying)
    fn query_pathing(&mut self, queries: &Vec<PathingQuery>)
        -> Result<Vec<f32>>
    ;

    /// returns whether these buildings can be placed at these locations
    ///
    /// the placing unit tag is only used for cases where placing the unit
    /// plays a role in the placement grid test (eg. a flying barracks
    /// building an addon requires room for both the barracks and addon).
    fn query_placement(&mut self, queries: &Vec<PlacementQuery>)
        -> Result<Vec<bool>>
    ;
}

impl Query for Participant {
    fn query_unit_abilities(
        &mut self, _: &Vec<Unit>, _: bool
    )
        -> Result<Vec<AvailableUnitAbilities>>
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
