use std::f32::{consts::PI, EPSILON};

use crate::{
    common::{DrawCommand, Float, Float2d},
    settings::SceneSettings,
};

#[derive(Default)]
pub struct Sprite {
    // --
    texture_id: i32,
    position: Float2d,
    sprite_scale: Float,
    elevation: Float,
    // -- state
    delta_angle: Float,
    screen_scale: Float,
    half_rays: Float,
    screen_distance: Float,
    distance: Float,
    center_x: Float,
}

impl Sprite {
    // TODO: get rid of copy-paste
    pub fn new(opts: &SceneSettings) -> Self {
        let rays = opts.screen_width >> 1;
        let delta_angle = opts.fov / rays as Float;
        let scale = opts.screen_width as Float / rays as Float;

        let half_fov = 0.5 * opts.fov;
        let screen_distance = opts.screen_width as Float * 0.5 * half_fov.tan();

        Self {
            delta_angle,
            screen_scale: scale,
            half_rays: (rays >> 1) as Float,
            screen_distance,
            sprite_scale: 1.0,
            ..Default::default()
        }
    }

    pub fn prepare(
        &mut self,
        texture_id: i32,
        position: Float2d,
        sprite_scale: Float,
        elevation: Float,
    ) {
        self.texture_id = texture_id;
        self.position = position;
        self.sprite_scale = sprite_scale;
        self.elevation = elevation;
    }

    pub fn update(&mut self, player_pos: Float2d, player_angle: Float) {
        let Float2d { x: mut dx, y: dy } = self.position - player_pos;
        if dx.abs() < EPSILON {
            dx += 1e-5;
        }
        let angle = (dy / dx).atan();

        let mut delta = angle - player_angle;
        if dx > 0.0 && player_angle > PI || dx < 0.0 && dy < 0.0 {
            delta += 2.0 * PI;
        }
        let delta_rays = delta / self.delta_angle;

        self.center_x = (self.half_rays + delta_rays) * self.screen_scale;
        self.distance = (dx * dx + dy * dy).sqrt() * delta.cos();
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        let projection = self.screen_distance / self.distance;
        commands.push(DrawCommand::Sprite(
            self.texture_id,
            self.center_x,
            projection * self.sprite_scale,
            self.elevation,
        ));
    }
}
