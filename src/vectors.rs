pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Int = i32;
pub type Int2d = Vec2d<Int>;

pub struct Size2d<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size2d<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}
