/// DoomLike Project, 2024
///
use std::time::Duration;

use sdl2::{event::Event, pixels::Color, EventPump, VideoSubsystem};

use crate::scene::Scene;

pub struct MediaServiceSDL<'a> {
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    scene: &'a mut dyn Scene,
}

impl<'a> MediaServiceSDL<'a> {
    pub fn new(scene: &'a mut dyn Scene) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let event_pump = context.event_pump()?;
        Ok(Self {
            video_subsystem,
            event_pump,
            scene,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let window_size = self.scene.window_size();
        let window = self
            .video_subsystem
            .window("Raycaster", window_size.width, window_size.height)
            .position_centered()
            .build()
            .map_err(|op| op.to_string())?;
        let mut canvas = window.into_canvas().build().map_err(|op| op.to_string())?;
        while self.scene.is_running() {
            self.process_events();
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            // TODO: code to draw scene
            canvas.present();
            let duration = Duration::from_millis(50);
            ::std::thread::sleep(duration);
        }
        Ok(())
    }

    fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.scene.on_terminate(),
                // Event::KeyDown { keycode, .. } => self.on_key_down(keycode),
                // Event::KeyUp { keycode, .. } => self.on_key_up(keycode),
                _ => {}
            }
        }
    }
}
