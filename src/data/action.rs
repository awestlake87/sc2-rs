
use sc2_proto::raw;
use sc2_proto::spatial::{
    ActionSpatialUnitCommand,
    ActionSpatialCameraMove,
    ActionSpatialUnitSelectionPoint,
    ActionSpatialUnitSelectionRect,
    ActionSpatialUnitSelectionPoint_Type as ProtoPointSelectionType
};

use super::{ Tag, Ability, Point2, Point2I, Rect2I };

pub enum ActionTarget {
    UnitTag(Tag),
    Position(Point2),
}

pub struct Action {
    pub ability:            Ability,
    pub unit_tags:          Vec<Tag>,
    pub target:             Option<ActionTarget>,
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

pub enum PointSelectType {
    Select,
    Toggle,
    All,
    AddAll
}

impl From<ProtoPointSelectionType> for PointSelectType {
    fn from(select_type: ProtoPointSelectionType) -> Self {
        match select_type {
            ProtoPointSelectionType::Select => PointSelectType::Select,
            ProtoPointSelectionType::Toggle => PointSelectType::Toggle,
            ProtoPointSelectionType::AllType => PointSelectType::All,
            ProtoPointSelectionType::AddAllType => PointSelectType::AddAll,
        }
    }
}

pub enum SpatialAction {
    UnitCommand {
        ability:            Ability,
        target:             Option<SpatialUnitCommandTarget>,
        queued:             bool
    },
    CameraMove {
        center_minimap:     Point2I
    },
    SelectPoint {
        select_screen:      Point2I,
        select_type:        PointSelectType
    },
    SelectRect {
        select_screen:      Vec<Rect2I>,
        select_add:         bool
    }
}

impl SpatialAction {
    pub fn from_unit_command_proto(cmd: &ActionSpatialUnitCommand) -> Self {
        SpatialAction::UnitCommand {
            ability: Ability::from_id(cmd.get_ability_id() as u32),
            queued: cmd.get_queue_command(),
            target: {
                if cmd.has_target_screen_coord() {
                    let pos = cmd.get_target_screen_coord();
                    Some(
                        SpatialUnitCommandTarget::Screen(
                            Point2I::new(pos.get_x(), pos.get_y())
                        )
                    )
                }
                else if cmd.has_target_minimap_coord() {
                    let pos = cmd.get_target_minimap_coord();
                    Some(
                        SpatialUnitCommandTarget::Minimap(
                            Point2I::new(pos.get_x(), pos.get_y())
                        )
                    )
                }
                else {
                    None
                }
            }
        }
    }

    pub fn from_camera_move_proto(cmd: &ActionSpatialCameraMove) -> Self {
        SpatialAction::CameraMove {
            center_minimap: {
                let pos = cmd.get_center_minimap();
                Point2I::new(pos.get_x(), pos.get_y())
            }
        }
    }

    pub fn from_selection_point_proto(cmd: &ActionSpatialUnitSelectionPoint)
        -> Self
    {
        SpatialAction::SelectPoint {
            select_screen: {
                let pos = cmd.get_selection_screen_coord();
                Point2I::new(pos.get_x(), pos.get_y())
            },
            select_type: PointSelectType::from(cmd.get_field_type())
        }
    }

    pub fn from_selection_rect_proto(cmd: &ActionSpatialUnitSelectionRect)
        -> Self
    {
        SpatialAction::SelectRect {
            select_screen: {
                let mut rects = vec![ ];

                for r in cmd.get_selection_screen_coord() {
                    rects.push(
                        Rect2I {
                            from: {
                                let p = r.get_p0();
                                Point2I::new(p.get_x(), p.get_y())
                            },
                            to: {
                                let p = r.get_p1();
                                Point2I::new(p.get_x(), p.get_y())
                            }
                        }
                    )
                }

                rects
            },
            select_add: cmd.get_selection_add()
        }
    }
}
