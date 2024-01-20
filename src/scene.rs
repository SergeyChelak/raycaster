use std::time::Instant;

use crate::{
    background::Background,
    common::{DrawCommand, Float2d, Size2d},
    control::{ControlEvent, ControllerState},
    player::Player,
    raycaster::RayCaster,
    settings::Settings,
    walls::Walls,
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
    walls: Walls,
    state: State,
    // -- drawables
    player: Player,
    ray_caster: RayCaster,
    background: Background,
    // --
    controller_state: ControllerState,
    time: Instant,
}

impl Scene {
    pub fn new(settings: Settings) -> Self {
        let opts = &settings.scene;
        let ray_caster = RayCaster::new(opts);
        let player = Player::new(&settings.player, opts.tile_size);
        let walls = Walls::new(opts.tile_size);
        let background = Background::new(opts.screen_size());
        Self {
            settings,
            walls,
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
        self.walls.prepare(&level_info.map);
        self.player
            .setup(Float2d::new(level_info.player_x, level_info.player_y), 0.0);
        self.state = State::Running;
    }

    pub fn process_events(&mut self, events: &[ControlEvent]) {
        for event in events {
            match event {
                ControlEvent::Keyboard {
                    key_code,
                    is_pressed,
                } => self.controller_state.on_key_event(*key_code, *is_pressed),
                ControlEvent::MouseMotion { x_rel, .. } => {
                    self.controller_state.mouse_x_relative = *x_rel;
                }
            }
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.time.elapsed().as_secs_f32();
        // TODO: this design isn't good, need to improve
        self.player
            .update(elapsed, &self.controller_state, &self.walls);
        self.ray_caster
            .update(self.player.pos(), self.player.angle(), &self.walls);
        self.background.update(self.player.angle());
        self.controller_state.reset_relative_values();
        self.time = Instant::now();
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        // TODO: this design isn't good, need to improve
        self.background.draw(commands);
        self.ray_caster.draw(commands);
        // TODO: refactor as mini map
        if self.controller_state.minimap_visible {
            self.walls.draw(commands);
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
