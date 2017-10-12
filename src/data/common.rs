
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
