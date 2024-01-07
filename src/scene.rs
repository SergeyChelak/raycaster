use std::time::Instant;

use crate::{
    common::{DrawCommand, Float2d, Size2d},
    control::{ControlEvent, ControllerState},
    pbm::PBMImage,
    player::Player,
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

type LevelMap = Vec<Vec<i32>>;

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
        self.player.setup(level_info);
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
        let Float2d { x, y } = self.player.pos();
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
