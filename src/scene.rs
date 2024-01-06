use crate::{settings::Settings, vectors::Size2d};

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

pub enum ControlEvent {
    Keyboard(i32, bool), // key code | is pressed
}

pub trait Scene {
    fn prepare(&mut self);

    fn process_events(&mut self, events: &[ControlEvent]);
    fn update(&mut self);
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
    fn update(&mut self) {
        println!("Updating scene state");
    }

    fn draw(&self) {
        println!("Drawing whole scene");
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

    fn process_events(&mut self, events: &[ControlEvent]) {
        println!("Processing {} events", events.len());
    }
}
