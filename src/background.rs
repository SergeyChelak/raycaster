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

    pub fn update(&mut self, _delta_time: Float, _angle: Float) {
        // TODO: calculate sky offset
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        // sky
        commands.push(DrawCommand::SkyTexture(TEXTURE_ID_SKY, self.offset));
        // floor
        // TODO: move floor color to settings or level data
        commands.push(DrawCommand::ColorRGB(30, 30, 30));
        let obj = DrawCommand::FillRectangle(
            0,
            self.scene_size.height as i32 >> 1,
            self.scene_size.width,
            self.scene_size.height >> 1,
        );
        commands.push(obj);
    }
}
