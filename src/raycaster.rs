use crate::{
    common::{DrawCommand, Float, Float2d},
    map::LevelMap,
    settings::SceneSettings,
};

const TOL: Float = 1e-3;
const DEFAULT_TEXTURE_ID: i32 = 1;

struct Rect {
    projected_height: Float,
    texture_id: i32,
    texture_offset: Float,
    depth: Float,
}

#[derive(Default)]
pub struct RayCaster {
    half_fov: Float,
    screen_distance: Float,
    scale: Float,
    height: Float,
    rays: usize,
    delta_angle: Float,
    max_depth: usize,
    rect_buffer: Vec<Rect>,
}

impl RayCaster {
    pub fn new(opts: &SceneSettings) -> Self {
        let half_fov = 0.5 * opts.fov;
        let rays = opts.screen_width >> 1;
        let delta_angle = opts.fov / rays as Float;

        let screen_distance = opts.screen_width as Float * 0.5 * half_fov.tan();
        let scale = opts.screen_width as Float / rays as Float;

        Self {
            half_fov,
            screen_distance,
            scale,
            height: opts.screen_height as Float,
            rays,
            delta_angle,
            max_depth: opts.max_depth,
            rect_buffer: Vec::with_capacity(rays),
        }
    }

    pub fn update(&mut self, pos: Float2d, angle: Float, map: &LevelMap) {
        self.rect_buffer.clear();
        // default texture ids
        let (mut texture_id_vertical, mut texture_id_horizontal) =
            (DEFAULT_TEXTURE_ID, DEFAULT_TEXTURE_ID);
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
                let point = Float2d::new(horizontal_x, horizontal_y);
                if map.has_collision(point) {
                    texture_id_horizontal = map.texture_id(point);
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
                let point = Float2d::new(vertical_x, vertical_y);
                if map.has_collision(point) {
                    texture_id_vertical = map.texture_id(point);
                    break;
                }
                vertical_x += dx;
                vertical_y += dy;
                vertical_depth += depth_delta;
            }

            let (mut depth, texture_id, offset) = if vertical_depth < horizontal_depth {
                vertical_y %= 1.0;
                let offset = if cos_a > 0.0 {
                    vertical_y
                } else {
                    1.0 - vertical_y
                };
                (vertical_depth, texture_id_vertical, offset)
            } else {
                horizontal_x %= 1.0;
                let offset = if sin_a > 0.0 {
                    1.0 - horizontal_x
                } else {
                    horizontal_x
                };
                (horizontal_depth, texture_id_horizontal, offset)
            };
            // get rid of fishbowl effect
            depth *= (angle - ray_angle).cos();

            let projected_height = self.screen_distance / (depth + TOL);
            self.rect_buffer.push(Rect {
                projected_height,
                texture_id,
                texture_offset: offset,
                depth,
            });

            ray_angle += self.delta_angle;
        }
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        for (ray, rect) in self.rect_buffer.iter().enumerate() {
            let cmd = DrawCommand::Texture(
                (ray as Float * self.scale) as i32,
                (0.5 * (self.height - rect.projected_height)) as i32,
                rect.texture_offset,
                self.scale as u32,
                rect.projected_height as u32,
                rect.depth,
                rect.texture_id,
            );
            commands.push(cmd);
        }
    }
}
