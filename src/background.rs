use std::f32::consts::PI;

use crate::common::{DrawCommand, Float, ScreenSize};

const TEXTURE_ID_SKY: i32 = 999;

pub struct Background {
    scene_size: ScreenSize,
    offset: Float,
}

impl Background {
    pub fn new(scene_size: ScreenSize) -> Self {
        Self {
            scene_size,
            offset: 0.0,
        }
    }

    pub fn update(&mut self, angle: Float) {
        let w = self.scene_size.width as Float;
        self.offset = 1.5 * angle * w / PI;
        self.offset %= w;
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        // sky
        commands.push(DrawCommand::SkyTexture {
            id: TEXTURE_ID_SKY,
            offset: -self.offset,
        });
        // floor
        // TODO: move floor color to settings or level data
        commands.push(DrawCommand::ColorRGB(30, 30, 30));
        let obj = DrawCommand::Rectangle {
            x: 0,
            y: self.scene_size.height as i32 >> 1,
            w: self.scene_size.width,
            h: self.scene_size.height >> 1,
            fill: true,
        };
        commands.push(obj);
    }
}
