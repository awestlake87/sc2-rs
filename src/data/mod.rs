
mod ability;
mod buff;
mod common;
mod game;
mod player;
mod unit;
mod upgrade;

use sc2_proto::raw;

pub use self::ability::*;
pub use self::buff::*;
pub use self::common::*;
pub use self::game::*;
pub use self::player::*;
pub use self::unit::*;
pub use self::upgrade::*;

pub struct PowerSource {
    tag:            Tag,
    pos:            Point2,
    radius:         f32,
}

impl From<raw::PowerSource> for PowerSource {
    fn from(source: raw::PowerSource) -> Self {
        Self {
            tag: source.get_tag(),
            pos: {
                let pos = source.get_pos();
                Point2::new(pos.get_x(), pos.get_y())
            },
            radius: source.get_radius(),
        }
    }
}
