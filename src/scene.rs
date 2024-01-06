use crate::{
    pbm::PBMImage,
    settings::Settings,
    vectors::{Int2d, Size2d},
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

pub trait Scene {
    /// could be considered as reset
    fn prepare(&mut self);

    fn process_events(&mut self, events: &[ControlEvent]);
    fn update(&mut self);
    fn draw(&self) -> Vec<GeometryObject>;

    /// system callbacks
    fn on_terminate(&mut self);

    /// state properties
    fn is_running(&self) -> bool;
    fn window_size(&self) -> Size2d<u32>;
}

type LevelMap = Vec<Vec<i32>>;

pub enum GeometryObject {
    Rectangle(i32, i32, u32, u32),
}

pub struct Raycaster {
    settings: Settings,
    map: LevelMap,
    state: State,
}

impl Raycaster {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            map: Vec::default(),
            state: State::default(),
        }
    }
}

impl Scene for Raycaster {
    fn update(&mut self) {
        // println!("Updating scene state");
    }

    fn draw(&self) -> Vec<GeometryObject> {
        let mut objects = Vec::new();
        // map
        let tile_size = self.settings.scene.tile_size;
        for (r, row) in self.map.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 0 {
                    continue;
                }
                let obj = GeometryObject::Rectangle(
                    (c * tile_size) as i32,
                    (r * tile_size) as i32,
                    tile_size as u32,
                    tile_size as u32,
                );
                objects.push(obj);
            }
        }
        // other object
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
        self.state = State::Running;
    }

    fn window_size(&self) -> Size2d<u32> {
        Size2d {
            width: self.settings.scene.screen_width as u32,
            height: self.settings.scene.screen_height as u32,
        }
    }

    fn process_events(&mut self, events: &[ControlEvent]) {
        // println!("Processing {} events", events.len());
    }
}
