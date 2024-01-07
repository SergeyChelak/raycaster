use std::{f32::consts::PI, time::Instant};

use crate::{
    common::{Float, Float2d, Size2d},
    pbm::PBMImage,
    settings::Settings,
};

pub enum State {
    Initial,
    Running,
    Terminated,
}

impl Default for State {
    fn default() -> Self {
        Self::Initial
    }
}

pub enum ControlEvent {
    Keyboard(i32, bool), // key code | is pressed
}

#[derive(Default)]
struct ControllerState {
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    rotate_left_pressed: bool,
    rotate_right_pressed: bool,
}

impl ControllerState {
    const KEYCODE_W: i32 = 119;
    const KEYCODE_S: i32 = 115;
    const KEYCODE_A: i32 = 97;
    const KEYCODE_D: i32 = 100;
    const KEYCODE_LEFT: i32 = 1073741904;
    const KEYCODE_RIGHT: i32 = 1073741903;
    // up = 1073741906
    // down = 1073741905

    fn on_key_event(&mut self, key_code: i32, is_pressed: bool) {
        match key_code {
            Self::KEYCODE_W => self.up_pressed = is_pressed,
            Self::KEYCODE_S => self.down_pressed = is_pressed,
            Self::KEYCODE_A => self.left_pressed = is_pressed,
            Self::KEYCODE_D => self.right_pressed = is_pressed,
            Self::KEYCODE_LEFT => self.rotate_left_pressed = is_pressed,
            Self::KEYCODE_RIGHT => self.rotate_right_pressed = is_pressed,
            _ => {
                // don't care
                // println!("Code {key_code}")
            }
        }
    }
}

#[derive(Default)]
struct Player {
    prev_position: Float2d,
    position: Float2d,
    angle: Float,
    movement_speed: Float,
    rotation_speed: Float,
}

impl Player {
    fn do_movement(&mut self, delta_time: Float, controller_state: &ControllerState) {
        let sin_a = self.angle.sin();
        let cos_a = self.angle.cos();
        let (mut dx, mut dy) = (0.0, 0.0);
        let dist = self.movement_speed * delta_time;
        let dist_cos = dist * cos_a;
        let dist_sin = dist * sin_a;

        if controller_state.up_pressed {
            dx = dist_cos;
            dy = dist_sin;
        }
        if controller_state.down_pressed {
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

    fn undo_movement(&mut self) {
        self.position = self.prev_position;
    }

    fn draw(&self, commands: &mut Vec<DrawCommand>) {
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
}

type LevelMap = Vec<Vec<i32>>;

pub enum DrawCommand {
    ColorRGB(u8, u8, u8),
    Rectangle(i32, i32, u32, u32),
    Line(i32, i32, i32, i32),
}

pub struct Scene {
    settings: Settings,
    map: LevelMap,
    state: State,
    player: Player,
    controller_state: ControllerState,
    time: Instant,
}

impl Scene {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            map: Vec::default(),
            state: State::default(),
            player: Player::default(),
            controller_state: ControllerState::default(),
            time: Instant::now(),
        }
    }

    pub fn prepare(&mut self) {
        let level_info = &self.settings.level;
        // TODO: refactor this method to return Result<...>
        if let Ok(pbm_image) = PBMImage::with_file(&level_info.map) {
            self.map = pbm_image.transform_to_array(|x| x as i32);
            println!("Level map was loaded");
        }
        self.player.position = Float2d::new(level_info.player_x, level_info.player_y);
        self.player.movement_speed = level_info.player_movement_speed;
        self.player.rotation_speed = level_info.player_rotation_speed;
        self.state = State::Running;
    }

    pub fn process_events(&mut self, events: &[ControlEvent]) {
        for event in events {
            match event {
                ControlEvent::Keyboard(code, is_pressed) => {
                    self.controller_state.on_key_event(*code, *is_pressed)
                }
            }
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.time.elapsed().as_secs_f32();
        self.player.do_movement(elapsed, &self.controller_state);
        if self.has_collisions() {
            self.player.undo_movement();
        }
        self.time = Instant::now();
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        // map
        commands.push(DrawCommand::ColorRGB(255, 255, 255));
        let tile_size = self.settings.scene.tile_size;
        for (r, row) in self.map.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 0 {
                    continue;
                }
                let obj = DrawCommand::Rectangle(
                    (c * tile_size) as i32,
                    (r * tile_size) as i32,
                    tile_size as u32,
                    tile_size as u32,
                );
                commands.push(obj);
            }
        }
        // player
        self.player.draw(commands);
        // other objects
        // ...
    }

    fn has_collisions(&self) -> bool {
        // there is no real collider
        // check if player collides with wall to make implementation simpler as possible
        let Float2d { x, y } = self.player.position;
        if x < 0.0 || y < 0.0 {
            return false;
        }
        let size = self.settings.scene.tile_size;
        let (col, row) = (x as usize / size, y as usize / size);
        if row > self.map.len() || col > self.map[0].len() {
            false
        } else {
            self.map[row][col] > 0
        }
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.state, State::Terminated)
    }

    pub fn on_terminate(&mut self) {
        self.state = State::Terminated;
    }

    pub fn window_size(&self) -> Size2d<u32> {
        Size2d {
            width: self.settings.scene.screen_width as u32,
            height: self.settings.scene.screen_height as u32,
        }
    }

    pub fn target_fps(&self) -> usize {
        self.settings.scene.fps
    }
}
