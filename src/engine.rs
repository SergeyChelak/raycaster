use std::time::Duration;

use sdl2::{event::Event, pixels::Color, AudioSubsystem, EventPump, Sdl, VideoSubsystem};

use crate::settings::RendererSettings;

enum PlatformState {
    Initial,
    Running,
    // Suspended,
    Terminated,
}

pub struct EngineSDL {
    context: Sdl,
    video_subsystem: VideoSubsystem,
    audio_subsystem: AudioSubsystem,
    event_pump: EventPump,
    state: PlatformState,
    settings: RendererSettings,
}

impl EngineSDL {
    pub fn new(settings: RendererSettings) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let audio_subsystem = context.audio()?;
        let event_pump = context.event_pump()?;
        Ok(Self {
            context,
            video_subsystem,
            audio_subsystem,
            event_pump,
            state: PlatformState::Initial,
            settings,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        if let PlatformState::Running = self.state {
            return Err("Main loop is in the progress already".to_string());
        }
        let window = self
            .video_subsystem
            .window(
                "Raycaster",
                self.settings.screen_width as u32,
                self.settings.screen_height as u32,
            )
            .position_centered()
            .build()
            .map_err(|op| op.to_string())?;
        let mut canvas = window.into_canvas().build().map_err(|op| op.to_string())?;
        self.state = PlatformState::Running;
        while !matches!(self.state, PlatformState::Terminated) {
            self.process_events();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();
            let duration = Duration::from_millis(50);
            ::std::thread::sleep(duration);
        }
        Ok(())
    }

    fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.state = PlatformState::Terminated,
                // Event::KeyDown { keycode, .. } => self.on_key_down(keycode),
                // Event::KeyUp { keycode, .. } => self.on_key_up(keycode),
                _ => {}
            }
        }
    }
}
