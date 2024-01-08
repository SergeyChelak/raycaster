use std::{fs, io, path::Path};

use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct Settings {
    pub scene: SceneSettings,
    pub level: LevelInfo,
}

impl Settings {
    pub fn with_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Default, Deserialize)]
pub struct SceneSettings {
    pub tile_size: usize,
    pub screen_height: usize,
    pub screen_width: usize,
    pub fps: usize,
    pub player_movement_speed: f32,
    pub player_rotation_speed: f32,
    pub max_depth: usize,
    pub fov: f32,
}

#[derive(Default, Deserialize)]
pub struct LevelInfo {
    pub map: String,
    pub player_x: f32,
    pub player_y: f32,
}
