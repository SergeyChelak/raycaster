use crate::{common::DrawCommand, pbm::PBMImage};

#[derive(Default)]
pub struct LevelMap {
    content: Vec<Vec<i32>>,
    tile_size: usize,
}

impl LevelMap {
    pub fn new(tile_size: usize) -> Self {
        Self {
            tile_size,
            ..Default::default()
        }
    }

    pub fn prepare(&mut self, level_path: &str) {
        // TODO: refactor this method to return Result<...>
        if let Ok(pbm_image) = PBMImage::with_file(&level_path) {
            self.content = pbm_image.transform_to_array(|x| x as i32);
        }
    }

    pub fn draw(&self, commands: &mut Vec<DrawCommand>) {
        commands.push(DrawCommand::ColorRGB(255, 255, 255));
        let tile_size = self.tile_size;
        for (r, row) in self.content.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 0 {
                    continue;
                }
                let obj = DrawCommand::Rectangle(
                    (c * tile_size) as i32,
                    (r * tile_size) as i32,
                    tile_size as u32,
                    tile_size as u32,
                );
                commands.push(obj);
            }
        }
    }
}
