use crate::{
    settings::Settings,
    vectors::{Int2d, Size2d},
};

/// DoomLike Project, 2024
///
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

#[derive(Default)]
pub struct UserInput {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,

    pub delta_time: usize, // ????
}

pub trait Scene {
    fn prepare(&mut self);

    fn update(&mut self, input: &UserInput) -> State;
    fn draw(&self);

    fn on_terminate(&mut self);
    fn is_running(&self) -> bool;

    fn window_size(&self) -> Size2d<u32>;
}

pub struct Raycaster {
    settings: Settings,
    state: State,
}

impl Raycaster {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            state: State::default(),
        }
    }
}

impl Scene for Raycaster {
    fn update(&mut self, input: &UserInput) -> State {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }

    fn is_running(&self) -> bool {
        !matches!(self.state, State::Terminated)
    }

    fn on_terminate(&mut self) {
        self.state = State::Terminated;
    }

    fn prepare(&mut self) {
        self.state = State::Running;
    }

    fn window_size(&self) -> Size2d<u32> {
        Size2d {
            width: self.settings.scene.screen_width as u32,
            height: self.settings.scene.screen_height as u32,
        }
    }
}
