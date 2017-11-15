
use sc2_proto::raw;
use sc2_proto::spatial::{
    ActionSpatialUnitCommand,
    ActionSpatialCameraMove,
    ActionSpatialUnitSelectionPoint,
    ActionSpatialUnitSelectionRect,
    ActionSpatialUnitSelectionPoint_Type as ProtoPointSelectionType
};

use super::super::{ Result, FromProto, IntoSc2 };
use super::{ Tag, Ability, Point2, Point2I, Rect2I };

/// action target
pub enum ActionTarget {
    /// target a unit with this action
    UnitTag(Tag),
    /// target a location with this action
    Location(Point2),
}

/// an action (command or ability) applied to a unit or set of units
pub struct Action {
    /// the ability to invoke
    pub ability:            Ability,
    /// units that this action applies to
    pub unit_tags:          Vec<Tag>,
    /// target of the action
    pub target:             Option<ActionTarget>,
}

impl FromProto<raw::ActionRawUnitCommand> for Action {
    /// convert from protobuf data
    fn from_proto(action: raw::ActionRawUnitCommand) -> Result<Self> {
        Ok(
            Self {
                ability: Ability::from_proto(action.get_ability_id()as u32)?,
                unit_tags: {
                    let mut unit_tags = vec![ ];

                    for tag in action.get_unit_tags() {
                        unit_tags.push(*tag);
                    }

                    unit_tags
                },
                target: {
                    if action.has_target_unit_tag() {
                        Some(
                            ActionTarget::UnitTag(action.get_target_unit_tag())
                        )
                    }
                    else if action.has_target_world_space_pos() {
                        let pos = action.get_target_world_space_pos();
                        Some(
                            ActionTarget::Location(
                                Point2::new(pos.get_x(), pos.get_y())
                            )
                        )
                    }
                    else {
                        None
                    }
                },
            }
        )
    }
}

/// target of a feature layer command
pub enum SpatialUnitCommandTarget {
    /// screen coordinate target
    Screen(Point2I),
    /// minimap coordinate target
    Minimap(Point2I),
}

/// type of point selection
pub enum PointSelectType {
    /// changes selection to unit (equal to normal click)
    Select,
    /// toggle selection of unit (equal to shift+click)
    Toggle,
    /// select all units of a given type (equal to ctrl+click)
    All,
    /// select all units of a given type additively (equal to shift+ctrl+click)
    AddAll
}

impl FromProto<ProtoPointSelectionType> for PointSelectType {
    fn from_proto(select_type: ProtoPointSelectionType) -> Result<Self> {
        Ok(
            match select_type {
                ProtoPointSelectionType::Select => PointSelectType::Select,
                ProtoPointSelectionType::Toggle => PointSelectType::Toggle,
                ProtoPointSelectionType::AllType => PointSelectType::All,
                ProtoPointSelectionType::AddAllType => PointSelectType::AddAll,
            }
        )
    }
}

/// feature layer action
pub enum SpatialAction {
    /// issue a feature layer unit command
    UnitCommand {
        /// ability to invoke
        ability:            Ability,
        /// target of command
        target:             Option<SpatialUnitCommandTarget>,
        /// whether this action should replace or queue behind other actions
        queued:             bool
    },
    /// move the camera to a new location
    CameraMove {
        /// minimap location
        center_minimap:     Point2I
    },
    /// select a point on the screen
    SelectPoint {
        /// point in screen coordinates
        select_screen:      Point2I,
        /// point selection type
        select_type:        PointSelectType
    },
    /// select a rectangle on the screen
    SelectRect {
        /// rectangle in screen coordinates
        select_screen:      Vec<Rect2I>,
        /// whether selection is additive
        select_add:         bool
    }
}

impl FromProto<ActionSpatialUnitCommand> for SpatialAction {
    fn from_proto(cmd: ActionSpatialUnitCommand) -> Result<Self> {
        Ok(
            SpatialAction::UnitCommand {
                ability: Ability::from_proto(cmd.get_ability_id() as u32)?,
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
        )
    }
}

impl FromProto<ActionSpatialCameraMove> for SpatialAction {
    fn from_proto(cmd: ActionSpatialCameraMove) -> Result<Self> {
        Ok(
            SpatialAction::CameraMove {
                center_minimap: {
                    let pos = cmd.get_center_minimap();
                    Point2I::new(pos.get_x(), pos.get_y())
                }
            }
        )
    }
}

impl FromProto<ActionSpatialUnitSelectionPoint> for SpatialAction {
    fn from_proto(cmd: ActionSpatialUnitSelectionPoint) -> Result<Self> {
        Ok(
            SpatialAction::SelectPoint {
                select_screen: {
                    let pos = cmd.get_selection_screen_coord();
                    Point2I::new(pos.get_x(), pos.get_y())
                },
                select_type: cmd.get_field_type().into_sc2()?
            }
        )
    }
}

impl FromProto<ActionSpatialUnitSelectionRect> for SpatialAction {
    fn from_proto(cmd: ActionSpatialUnitSelectionRect) -> Result<Self> {
        Ok(
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
        )
    }
}
