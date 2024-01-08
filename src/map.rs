use crate::{
    common::{DrawCommand, Float2d},
    pbm::PBMImage,
};

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

    pub fn has_collision(&self, point: Float2d) -> bool {
        let Float2d { x, y } = point;
        if x < 0.0 || y < 0.0 {
            println!("[WARN]: map location is out of bounds x:{x:.2}, y:{y:.2} (1)");
            return false;
        }
        let (col, row) = (point.x as usize, point.y as usize);
        if row >= self.content.len() || col >= self.content[0].len() {
            println!("[WARN]: map location is out of bounds x:{x:.2}, y:{y:.2} (2)");
        }
        self.content[row][col] > 0
    }
}
