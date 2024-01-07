use std::f32::consts::PI;

use crate::{
    common::{DrawCommand, Float, Float2d},
    control::ControllerState,
    settings::LevelInfo,
};

#[derive(Default)]
pub struct Player {
    prev_position: Float2d,
    position: Float2d,
    angle: Float,
    movement_speed: Float,
    rotation_speed: Float,
}

impl Player {
    pub fn setup(&mut self, level_info: &LevelInfo) {
        self.position = Float2d::new(level_info.player_x, level_info.player_y);
        self.movement_speed = level_info.player_movement_speed;
        self.rotation_speed = level_info.player_rotation_speed;
    }

    pub fn do_movement(&mut self, delta_time: Float, controller_state: &ControllerState) {
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
        self.prev_position = self.position;
        self.position += Float2d::new(dx, dy);

        if controller_state.rotate_left_pressed {
            self.angle -= self.rotation_speed * delta_time;
        }
        if controller_state.rotate_right_pressed {
            self.angle += self.rotation_speed * delta_time;
        }
        self.angle %= 2.0 * PI;
    }

    pub fn undo_movement(&mut self) {
        self.position = self.prev_position;
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        commands.push(DrawCommand::ColorRGB(255, 128, 128));
        let size = 10;
        let (x, y) = (self.position.x as i32, self.position.y as i32);
        let rect = DrawCommand::Rectangle(x - size / 2, y - size / 2, size as u32, size as u32);
        commands.push(rect);
        commands.push(DrawCommand::ColorRGB(255, 255, 0)); // yellow

        let length = 250.0;
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
}
