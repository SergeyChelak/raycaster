use std::time::Instant;

use crate::{
    common::{DrawCommand, Float2d, Size2d},
    control::{ControlEvent, ControllerState},
    map::LevelMap,
    player::Player,
    raycaster::RayCaster,
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

pub struct Scene {
    settings: Settings,
    map: LevelMap,
    state: State,
    player: Player,
    ray_caster: RayCaster,
    controller_state: ControllerState,
    time: Instant,
}

impl Scene {
    pub fn new(settings: Settings) -> Self {
        let opts = &settings.scene;
        let ray_caster = RayCaster::new(opts);
        let player = Player::new(
            opts.player_movement_speed,
            opts.player_rotation_speed,
            opts.tile_size,
        );
        let map = LevelMap::new(opts.tile_size);
        Self {
            settings,
            map,
            state: State::default(),
            player,
            ray_caster,
            controller_state: ControllerState::default(),
            time: Instant::now(),
        }
    }

    pub fn prepare(&mut self) {
        let level_info = &self.settings.level;
        self.map.prepare(&level_info.map);
        self.player
            .setup(Float2d::new(level_info.player_x, level_info.player_y), 0.0);
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
        self.player
            .do_movement(elapsed, &self.controller_state, &self.map);
        self.ray_caster
            .update(self.player.pos(), self.player.angle(), &self.map);
        self.time = Instant::now();
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        self.map.draw(commands);
        self.ray_caster.draw(commands);
        self.player.draw(commands);
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
