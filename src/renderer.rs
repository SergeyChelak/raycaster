use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
    EventPump,
};

use crate::{common::DrawCommand, control::ControlEvent};
use crate::{common::Float, scene::Scene};

pub struct RendererSDL<'a> {
    canvas: WindowCanvas,
    event_pump: EventPump,
    scene: &'a mut Scene,
}

impl<'a> RendererSDL<'a> {
    pub fn new(scene: &'a mut Scene) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window_size = scene.window_size();
        let window = video_subsystem
            .window("FPS: ??", window_size.width, window_size.height)
            .position_centered()
            .build()
            .map_err(|op| op.to_string())?;
        // context.mouse().show_cursor(false);
        let canvas = window.into_canvas().build().map_err(|op| op.to_string())?;
        let event_pump = context.event_pump()?;
        Ok(Self {
            canvas,
            event_pump,
            scene,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.scene.prepare();
        let texture_creator = self.canvas.texture_creator();
        let textures = Self::load_textures(&texture_creator)?;
        let mut frames = 0;
        let mut time = Instant::now();
        let mut draw_commands = Vec::with_capacity(1000);
        let target_duration = (1000 / self.scene.target_fps()) as u128;
        while self.scene.is_running() {
            let frame_start = Instant::now();
            draw_commands.clear();
            self.process_events();
            self.scene.update();
            self.draw(&textures, &mut draw_commands)?;
            frames += 1;
            let elapsed = time.elapsed();
            if elapsed.as_millis() > 1000 {
                time = Instant::now();
                let title = format!("FPS: {frames}");
                _ = self.canvas.window_mut().set_title(&title);
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

    fn draw(
        &mut self,
        textures: &HashMap<i32, Texture>,
        commands: &mut Vec<DrawCommand>,
    ) -> Result<(), String> {
        self.scene.draw(commands);
        self.canvas.clear();

        for command in commands {
            match *command {
                DrawCommand::ColorRGB(r, g, b) => {
                    self.canvas.set_draw_color(Color::RGB(r, g, b));
                }
                DrawCommand::Rectangle(x, y, w, h) => {
                    let rect = Rect::new(x, y, w, h);
                    self.canvas.draw_rect(rect)?;
                }
                DrawCommand::FillRectangle(x, y, w, h) => {
                    let rect = Rect::new(x, y, w, h);
                    self.canvas.fill_rect(rect)?;
                }
                DrawCommand::Line(x1, y1, x2, y2) => {
                    let start = Point::new(x1, y1);
                    let end = Point::new(x2, y2);
                    self.canvas.draw_line(start, end)?;
                }
                DrawCommand::Texture(depth, x, y, offset, width, projected_height, id) => {
                    let dst = Rect::new(x, y, width, projected_height);
                    let Some(texture) = textures.get(&id) else {
                        // draw gray-scale bars in case of missing texture
                        let clr = (255.0 / (1.0 + depth.powi(5) * 0.00002)) as u8;
                        self.canvas.set_draw_color(Color::RGB(clr, clr, clr));
                        self.canvas.draw_rect(dst)?;
                        continue;
                    };
                    let query = texture.query();
                    let (w, h) = (query.width, query.height);
                    let src =
                        Rect::new((offset * (w as Float - width as Float)) as i32, 0, width, h);
                    self.canvas.copy(texture, src, dst)?;
                }
                DrawCommand::SkyTexture(id, offset) => {
                    let Some(texture) = textures.get(&id) else {
                        continue;
                    };
                    let query = texture.query();
                    let (w, h) = (query.width, query.height);
                    let src = Rect::new(0, 0, w, h);
                    let scene_size = self.scene.window_size();
                    let offset = offset as i32;
                    let half_height = scene_size.height >> 1;
                    let dst = Rect::new(offset, 0, scene_size.width, half_height);
                    self.canvas.copy(texture, src, dst)?;
                    let dst = Rect::new(
                        offset - scene_size.width as i32,
                        0,
                        scene_size.width,
                        half_height,
                    );
                    self.canvas.copy(texture, src, dst)?;
                    let dst = Rect::new(
                        offset + scene_size.width as i32,
                        0,
                        scene_size.width,
                        half_height,
                    );
                    self.canvas.copy(texture, src, dst)?;
                }
            }
        }
        self.canvas.present();
        Ok(())
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
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    events.push(ControlEvent::MouseMotion(x, y, xrel, yrel));
                }
                _ => {}
            }
        }
        self.scene.process_events(&events);
    }

    fn load_textures(
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<HashMap<i32, Texture>, String> {
        let assets = [
            (1, "assets/textures/1.png"),
            (2, "assets/textures/2.png"),
            (3, "assets/textures/3.png"),
            (4, "assets/textures/4.png"),
            (5, "assets/textures/5.png"),
            (999, "assets/textures/sky.png"),
        ];
        let mut textures = HashMap::new();
        for (id, path) in assets {
            let Ok(texture) = texture_creator.load_texture(path) else {
                println!("[ERR] failed to load texture with id: {id} at '{path}'");
                continue;
            };
            textures.insert(id, texture);
        }
        Ok(textures)
    }
}
