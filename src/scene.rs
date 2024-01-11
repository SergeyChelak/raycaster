use std::time::Instant;

use crate::{
    background::Background,
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
    background: Background,
    controller_state: ControllerState,
    time: Instant,
}

impl Scene {
    pub fn new(settings: Settings) -> Self {
        let opts = &settings.scene;
        let ray_caster = RayCaster::new(opts);
        let player = Player::new(&settings.player, opts.tile_size);
        let map = LevelMap::new(opts.tile_size);
        let background = Background::new(opts.screen_size());
        Self {
            settings,
            map,
            state: State::default(),
            player,
            ray_caster,
            background,
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
                ControlEvent::MouseMotion(_x, _y, x_rel, _y_rel) => {
                    self.controller_state.mouse_x_relative = *x_rel;
                }
            }
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.time.elapsed().as_secs_f32();
        self.player
            .update(elapsed, &self.controller_state, &self.map);
        self.ray_caster
            .update(self.player.pos(), self.player.angle(), &self.map);
        self.background.update(self.player.angle());
        self.controller_state.reset_relative_values();
        self.time = Instant::now();
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        self.background.draw(commands);
        self.ray_caster.draw(commands);
        // TODO: refactor as mini map
        {
            self.map.draw(commands);
            self.player.draw(commands);
        }
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.state, State::Terminated)
    }

    pub fn on_terminate(&mut self) {
        self.state = State::Terminated;
    }

    pub fn window_size(&self) -> Size2d<u32> {
        self.settings.scene.screen_size()
    }

    pub fn target_fps(&self) -> usize {
        self.settings.scene.fps
    }
}
