use crate::vectors::Vec2d;

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
    Rectangle {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        fill: bool,
    },
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    },
    SkyTexture {
        id: i32,
        offset: Float,
    },
    Texture {
        depth: Float,
        x: i32,
        y: i32,
        offset: Float,
        width: u32,
        projected_height: u32,
        texture_id: i32,
    },
}
