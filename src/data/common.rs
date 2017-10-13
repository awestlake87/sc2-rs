
use na::{ Vector2, Vector3 };

#[derive(Copy, Clone)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T
}

pub type Point2 = Vector2<f32>;
pub type Point3 = Vector3<f32>;

#[derive(Copy, Clone)]
pub struct Rect2 {
    from:       Point2,
    to:         Point2,
}

pub type Point2I = Vector2<i32>;
pub type Point3I = Vector3<i32>;

#[derive(Copy, Clone)]
pub struct Rect2I {
    from:       Point2I,
    to:         Point2I,
}
