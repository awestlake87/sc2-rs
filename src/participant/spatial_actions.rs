
use super::{ Result };
use data::{ Ability, Point2I, Rect2I, PointSelectType };
use participant::{ Participant };

/// UNSTABLE feature layer trait
pub trait FeatureLayerActions {
    fn command_unit_spatial(&mut self, ability: Ability) -> Result<()>;
    fn command_unit_screen(&mut self, ability: Ability, point: Point2I)
        -> Result<()>
    ;
    fn command_unit_minimap(&mut self, ability: Ability, point: Point2I)
        -> Result<()>
    ;

    fn select(&mut self, center: Point2I, select: PointSelectType)
        -> Result<()>
    ;
    fn select_rect(&mut self, rect: Rect2I, additive: bool) -> Result<()>;

    fn send_spatial_actions(&mut self) -> Result<()>;
}

impl FeatureLayerActions for Participant {
    fn command_unit_spatial(&mut self, _: Ability) -> Result<()> {
        unimplemented!("command unit spatial");
    }
    fn command_unit_screen(&mut self, _: Ability, _: Point2I) -> Result<()> {
        unimplemented!("command unit screen");
    }
    fn command_unit_minimap(&mut self, _: Ability, _: Point2I) -> Result<()> {
        unimplemented!("command unit minimap");
    }

    fn select(&mut self, _: Point2I, _: PointSelectType) -> Result<()> {
        unimplemented!("select");
    }
    fn select_rect(&mut self, _: Rect2I, _: bool) -> Result<()> {
        unimplemented!("select rect");
    }

    fn send_spatial_actions(&mut self) -> Result<()> {
        unimplemented!("send spatial actions");
    }
}
