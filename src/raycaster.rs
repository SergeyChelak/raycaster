use crate::{
    common::{DrawCommand, Float, Float2d},
    map::LevelMap,
};

const TOL: Float = 1e-3;

struct Ray {
    depth: Float,
    sine: Float,
    cosine: Float,
    position: Float2d,
}

#[derive(Default)]
pub struct RayCaster {
    half_fov: Float,
    rays: usize,
    delta_angle: Float,
    max_depth: usize,
    tile_size: Float,
    ray_buffer: Vec<Ray>,
}

impl RayCaster {
    pub fn new(fov: Float, rays: usize, max_depth: usize, tile_size: usize) -> Self {
        Self {
            half_fov: fov * 0.5,
            rays,
            delta_angle: fov / rays as Float,
            max_depth,
            tile_size: tile_size as Float,
            ray_buffer: Vec::with_capacity(rays),
        }
    }

    pub fn update(&mut self, pos: Float2d, angle: Float, map: &LevelMap) {
        self.ray_buffer.clear();
        let (tile_x, tile_y) = (pos.x.floor(), pos.y.floor());
        let mut ray_angle = angle - self.half_fov + TOL;
        for _ in 0..self.rays {
            let sin_a = ray_angle.sin();
            let cos_a = ray_angle.cos();
            // horizontals
            let (mut horizontal_y, dy) = if sin_a > 0.0 {
                (tile_y + 1.0, 1.0)
            } else {
                (tile_y - TOL, -1.0)
            };
            let mut horizontal_depth = (horizontal_y - pos.y) / sin_a;
            let mut horizontal_x = pos.x + horizontal_depth * cos_a;
            let depth_delta = dy / sin_a;
            let dx = depth_delta * cos_a;
            for _ in 0..self.max_depth {
                if map.has_collision(Float2d::new(horizontal_x, horizontal_y)) {
                    break;
                }
                horizontal_x += dx;
                horizontal_y += dy;
                horizontal_depth += depth_delta;
            }
            // verticals
            let (mut vertical_x, dx) = if cos_a > 0.0 {
                (tile_x + 1.0, 1.0)
            } else {
                (tile_x - TOL, -1.0)
            };
            let mut vertical_depth = (vertical_x - pos.x) / cos_a;
            let mut vertical_y = pos.y + vertical_depth * sin_a;
            let depth_delta = dx / cos_a;
            let dy = depth_delta * sin_a;
            for _ in 0..self.max_depth {
                if map.has_collision(Float2d::new(vertical_x, vertical_y)) {
                    break;
                }
                vertical_x += dx;
                vertical_y += dy;
                vertical_depth += depth_delta;
            }

            let depth = vertical_depth.min(horizontal_depth);
            self.ray_buffer.push(Ray {
                depth,
                sine: sin_a,
                cosine: cos_a,
                position: pos,
            });
            ray_angle += self.delta_angle;
        }
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        commands.push(DrawCommand::ColorRGB(0, 127, 127));
        for ray in &self.ray_buffer {
            let x = ray.position.x * self.tile_size;
            let y = ray.position.y * self.tile_size;
            let cmd = DrawCommand::Line(
                x as i32,
                y as i32,
                (x + ray.depth * self.tile_size * ray.cosine) as i32,
                (y + ray.depth * self.tile_size * ray.sine) as i32,
            );
            commands.push(cmd);
        }
    }
}
