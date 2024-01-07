use crate::vectors::Vec2d;

#[allow(dead_code)]
pub type Int = i32;
#[allow(dead_code)]
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

pub enum DrawCommand {
    ColorRGB(u8, u8, u8),
    Rectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
}
