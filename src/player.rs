use std::f32::consts::PI;

use crate::{
    common::{DrawCommand, Float, Float2d},
    control::ControllerState,
    map::LevelMap,
};

#[derive(Default)]
pub struct Player {
    position: Float2d,
    angle: Float,
    movement_speed: Float,
    rotation_speed: Float,
    tile_size: Float,
}

impl Player {
    pub fn new(movement_speed: Float, rotation_speed: Float, tile_size: usize) -> Self {
        Self {
            movement_speed,
            rotation_speed,
            tile_size: tile_size as Float,
            ..Self::default()
        }
    }

    pub fn setup(&mut self, position: Float2d, angle: Float) {
        self.position = position;
        self.angle = angle;
    }

    pub fn update(
        &mut self,
        delta_time: Float,
        controller_state: &ControllerState,
        map: &LevelMap,
    ) {
        let sin_a = self.angle.sin();
        let cos_a = self.angle.cos();
        let (mut dx, mut dy) = (0.0, 0.0);
        let dist = self.movement_speed * delta_time;
        let dist_cos = dist * cos_a;
        let dist_sin = dist * sin_a;

        if controller_state.forward_pressed {
            dx = dist_cos;
            dy = dist_sin;
        }
        if controller_state.backward_pressed {
            dx = -dist_cos;
            dy = -dist_sin;
        }
        if controller_state.left_pressed {
            dx = dist_sin;
            dy = -dist_cos;
        }
        if controller_state.right_pressed {
            dx = -dist_sin;
            dy = dist_cos;
        }
        let position = self.position + Float2d::new(dx, dy);
        if !map.has_collision(position) {
            self.position = position;
        }

        if controller_state.rotate_left_pressed {
            self.angle -= self.rotation_speed * delta_time;
        }
        if controller_state.rotate_right_pressed {
            self.angle += self.rotation_speed * delta_time;
        }
        self.angle %= 2.0 * PI;
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        commands.push(DrawCommand::ColorRGB(255, 128, 128));
        let size = 10;
        let (x, y) = (
            (self.position.x * self.tile_size) as i32,
            (self.position.y * self.tile_size) as i32,
        );
        let rect = DrawCommand::Rectangle(x - size / 2, y - size / 2, size as u32, size as u32);
        commands.push(rect);

        let length = 3.0 * self.tile_size;
        let line = DrawCommand::Line(
            x,
            y,
            x + (length * self.angle.cos()) as i32,
            y + (length * self.angle.sin()) as i32,
        );
        commands.push(line);
    }

    pub fn pos(&self) -> Float2d {
        self.position
    }

    pub fn angle(&self) -> Float {
        self.angle
    }
}
