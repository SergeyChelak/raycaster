use crate::{
    pbm::PBMImage,
    settings::Settings,
    vectors::{Float2d, Size2d},
};

/// DoomLike Project, 2024
///

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
}

pub trait Scene {
    /// could be considered as reset
    fn prepare(&mut self);

    fn process_events(&mut self, events: &[ControlEvent]);
    fn update(&mut self);
    fn draw(&self) -> Vec<DrawCommand>;

    /// system callbacks
    fn on_terminate(&mut self);

    /// state properties
    fn is_running(&self) -> bool;
    fn window_size(&self) -> Size2d<u32>;
}

type LevelMap = Vec<Vec<i32>>;

pub enum DrawCommand {
    ColorRGB(u8, u8, u8),
    Rectangle(i32, i32, u32, u32),
}

pub struct Raycaster {
    settings: Settings,
    map: LevelMap,
    state: State,
    player_pos: Float2d,
    controller_state: ControllerState,
}

impl Raycaster {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            map: Vec::default(),
            state: State::default(),
            player_pos: Float2d::default(),
            controller_state: ControllerState::default(),
        }
    }
}

impl Scene for Raycaster {
    fn update(&mut self) {
        let step = 5.0;
        if self.controller_state.up_pressed {
            self.player_pos.y -= step;
        }
        if self.controller_state.down_pressed {
            self.player_pos.y += step;
        }
        if self.controller_state.left_pressed {
            self.player_pos.x -= step;
        }
        if self.controller_state.right_pressed {
            self.player_pos.x += step;
        }
    }

    fn draw(&self) -> Vec<DrawCommand> {
        let mut objects = Vec::new();
        // map
        objects.push(DrawCommand::ColorRGB(255, 255, 255));
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
                objects.push(obj);
            }
        }
        // player
        {
            objects.push(DrawCommand::ColorRGB(255, 128, 128));
            let rect =
                DrawCommand::Rectangle(self.player_pos.x as i32, self.player_pos.y as i32, 10, 10);
            objects.push(rect);
        }
        // other objects
        // ...
        objects
    }

    fn is_running(&self) -> bool {
        !matches!(self.state, State::Terminated)
    }

    fn on_terminate(&mut self) {
        self.state = State::Terminated;
    }

    fn prepare(&mut self) {
        let level_info = &self.settings.level;
        // TODO: refactor this method to return Result<...>
        if let Ok(pbm_image) = PBMImage::with_file(&level_info.map) {
            self.map = pbm_image.transform_to_array(|x| x as i32);
            println!("Level map was loaded");
        }
        self.player_pos = Float2d::new(level_info.player_x, level_info.player_y);
        self.state = State::Running;
    }

    fn window_size(&self) -> Size2d<u32> {
        Size2d {
            width: self.settings.scene.screen_width as u32,
            height: self.settings.scene.screen_height as u32,
        }
    }

    fn process_events(&mut self, events: &[ControlEvent]) {
        for event in events {
            match event {
                ControlEvent::Keyboard(code, is_pressed) => match code {
                    119 => self.controller_state.up_pressed = *is_pressed,
                    115 => self.controller_state.down_pressed = *is_pressed,
                    97 => self.controller_state.left_pressed = *is_pressed,
                    100 => self.controller_state.right_pressed = *is_pressed,
                    _ => {
                        // don't care
                    }
                },
            }
        }
    }
}
