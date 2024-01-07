use crate::vectors::Vec2d;

pub type Int = i32;
pub type Int2d = Vec2d<Int>;

pub type Float = f32;
pub type Float2d = Vec2d<Float>;

pub struct Size2d<T> {
    pub width: T,
    pub height: T,
}

// impl<T> Size2d<T> {
//     pub fn new(width: T, height: T) -> Self {
//         Self { width, height }
//     }
// }
