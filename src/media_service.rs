use std::time::{Duration, Instant};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
    EventPump, VideoSubsystem,
};

use crate::scene::{ControlEvent, DrawCommand, Scene};

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
        scene.prepare();
        Ok(Self {
            video_subsystem,
            event_pump,
            scene,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let window_size = self.scene.window_size();
        let window = self
            .video_subsystem
            .window("Raycaster", window_size.width, window_size.height)
            .position_centered()
            .build()
            .map_err(|op| op.to_string())?;
        let mut canvas = window.into_canvas().build().map_err(|op| op.to_string())?;
        let mut frames = 0;
        let mut time = Instant::now();
        let mut draw_commands = Vec::with_capacity(1000);
        let target_duration = (1000 / self.scene.target_fps()) as u128;
        while self.scene.is_running() {
            let frame_start = Instant::now();
            draw_commands.clear();
            self.process_events();
            self.scene.update();
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            self.draw(&mut canvas, &mut draw_commands);
            canvas.present();
            frames += 1;
            let elapsed = time.elapsed();
            if elapsed.as_millis() > 1000 {
                time = Instant::now();
                let title = format!("FPS: {frames}");
                _ = canvas.window_mut().set_title(&title);
                frames = 0;
            }
            let suspend_ms = target_duration.saturating_sub(frame_start.elapsed().as_millis());
            if suspend_ms > 0 {
                let duration = Duration::from_millis(suspend_ms as u64);
                ::std::thread::sleep(duration);
            }
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut WindowCanvas, commands: &mut Vec<DrawCommand>) {
        self.scene.draw(commands);
        for command in commands {
            match *command {
                DrawCommand::ColorRGB(r, g, b) => {
                    canvas.set_draw_color(Color::RGB(r, g, b));
                }
                DrawCommand::Rectangle(x, y, w, h) => {
                    let rect = Rect::new(x, y, w, h);
                    _ = canvas.draw_rect(rect);
                }
                DrawCommand::Line(x1, y1, x2, y2) => {
                    let start = Point::new(x1, y1);
                    let end = Point::new(x2, y2);
                    _ = canvas.draw_line(start, end)
                }
            }
        }
    }

    fn process_events(&mut self) {
        let mut events = Vec::<ControlEvent>::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.scene.on_terminate(),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    events.push(ControlEvent::Keyboard(keycode as i32, true));
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    events.push(ControlEvent::Keyboard(keycode as i32, false));
                }
                _ => {}
            }
        }
        self.scene.process_events(&events);
    }
}
