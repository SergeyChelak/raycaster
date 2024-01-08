use crate::common::{DrawCommand, Float, Float2d};

#[derive(Default)]
pub struct RayCaster {
    half_fov: Float,
    rays: usize,
    delta_angle: Float,
    max_depth: usize,
    tile_size: usize,
}

impl RayCaster {
    pub fn new(fov: Float, rays: usize, max_depth: usize, tile_size: usize) -> Self {
        Self {
            half_fov: fov * 0.5,
            rays,
            delta_angle: fov / rays as Float,
            max_depth,
            tile_size,
        }
    }

    pub fn update(&mut self, pos: Float2d, angle: Float) {
        //
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        //
    }
}
