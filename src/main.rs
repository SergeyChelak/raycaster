mod media_service;
use media_service::*;

mod settings;
use settings::*;

mod common;
mod pbm;

mod vectors;

mod scene;
use scene::*;

const SETTINGS_FILE_PATH: &str = "raycaster.toml";

fn main() -> Result<(), String> {
    let Ok(settings) = Settings::with_file(SETTINGS_FILE_PATH) else {
        return Err("Failed to read settings".to_string());
    };
    let mut ray_caster = Raycaster::new(settings);
    let mut media_service = MediaServiceSDL::new(&mut ray_caster)?;
    media_service.run()?;
    Ok(())
}
