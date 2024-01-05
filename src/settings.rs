use std::{fs, io, path::Path};

use toml;

use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct Settings {
    pub renderer: RendererSettings,
}

impl Settings {
    pub fn with_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Default, Deserialize)]
pub struct RendererSettings {
    pub tile_size: usize,
    pub screen_height: usize,
    pub screen_width: usize,
}
