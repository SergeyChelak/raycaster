/// DoomLike Project, 2024
///
/// Ray-caster
///
mod media_service;
use media_service::*;

mod settings;
use settings::*;

mod pbm;
use pbm::PBMImage;

mod vectors;

mod scene;
use scene::*;

use crate::vectors::Int2d;

const SETTINGS_FILE_PATH: &str = "assets/settings.toml";

fn main() -> Result<(), String> {
    let Ok(settings) = Settings::with_file(SETTINGS_FILE_PATH) else {
        return Err("Failed to read settings".to_string());
    };
    // let image = PBMImage::with_file("assets/r1l2_ascii.pbm")?;
    // println!("Image h: {}, w: {}", image.cols(), image.rows());
    // println!("{image}");

    let mut ray_caster = Raycaster::new(settings);

    let mut media_service = MediaServiceSDL::new(&mut ray_caster)?;
    media_service.start()?;
    Ok(())
}
