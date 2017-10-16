
use rand::random;

use data::{ Rect2, Point2, GameInfo };

pub fn find_random_location_in_rect(r: Rect2) -> Point2 {
    let w = r.to.x - r.from.x;
    let h = r.to.y - r.from.y;

    Point2::new(
        w * random::<f32>() + r.from.x, h * random::<f32>() + r.from.y
    )
}

pub fn find_random_location(game_info: &GameInfo) -> Point2 {
    find_random_location_in_rect(
        Rect2 {
            from: game_info.playable_min,
            to: game_info.playable_max
        }
    )
}
