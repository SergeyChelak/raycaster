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

pub type ScreenSize = Size2d<u32>;

pub enum DrawCommand {
    ColorRGB(u8, u8, u8),
    Rectangle(i32, i32, u32, u32),
    FillRectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
    Texture(i32, i32, Float, u32, u32, Float, i32), // x, y, offset, width, projected_height, depth, id
    SkyTexture(i32, Float),                         // id, offset
}
