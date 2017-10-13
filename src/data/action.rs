
use sc2_proto::raw;
use sc2_proto::spatial::{
    ActionSpatialUnitCommand,
    ActionSpatialCameraMove,
    ActionSpatialUnitSelectionPoint,
    ActionSpatialUnitSelectionRect,
};

use super::{ Tag, Ability, Point2, Point2I, Rect2I };

pub enum ActionTarget {
    UnitTag(Tag),
    Position(Point2),
}

pub struct Action {
    ability:            Ability,
    unit_tags:          Vec<Tag>,
    target:             Option<ActionTarget>,
}

impl Action {
    pub fn from_proto(action: &raw::ActionRawUnitCommand) -> Self {
        Self {
            ability: Ability::from_id(action.get_ability_id() as u32),
            unit_tags: {
                let mut unit_tags = vec![ ];

                for tag in action.get_unit_tags() {
                    unit_tags.push(*tag);
                }

                unit_tags
            },
            target: {
                if action.has_target_unit_tag() {
                    Some(ActionTarget::UnitTag(action.get_target_unit_tag()))
                }
                else if action.has_target_world_space_pos() {
                    let pos = action.get_target_world_space_pos();
                    Some(
                        ActionTarget::Position(
                            Point2::new(pos.get_x(), pos.get_y())
                        )
                    )
                }
                else {
                    None
                }
            },
        }
    }
}

pub enum SpatialUnitCommandTarget {
    Screen(Point2I),
    Minimap(Point2I),
}

pub enum PointSelectionType {
    Select,
    Toggle,
    All,
    AddAll
}

pub enum SpatialAction {
    UnitCommand {
        ability:            Ability,
        target:             SpatialUnitCommandTarget,
        queued:             bool
    },
    CameraMove {
        center_minimap:     Point2I
    },
    SelectPoint {
        select_screen:      Point2I,
        select_type:        PointSelectionType
    },
    SelectRect {
        select_screen:      Vec<Rect2I>,
        select_add:         bool
    }
}

impl SpatialAction {
    pub fn from_unit_command_proto(cmd: &ActionSpatialUnitCommand) -> Self {
        unimplemented!("from unit command proto");
    }

    pub fn from_camera_move_proto(cmd: &ActionSpatialCameraMove) -> Self {
        unimplemented!("from camera move proto");
    }

    pub fn from_selection_point_proto(cmd: &ActionSpatialUnitSelectionPoint)
        -> Self
    {
        unimplemented!("from selection point proto");
    }

    pub fn from_selection_rect_proto(cmd: &ActionSpatialUnitSelectionRect)
        -> Self
    {
        unimplemented!("from selection rect proto");
    }
}
