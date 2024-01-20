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

// TODO: it's a bad design, need to improve
pub enum DrawCommand {
    ColorRGB(u8, u8, u8),
    Rectangle(i32, i32, u32, u32),
    FillRectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
    SkyTexture { id: i32, offset: Float }, // id, offset
    Texture(Float, i32, i32, Float, u32, u32, i32), // distance, x, y, offset, width, projected_height, id
}
