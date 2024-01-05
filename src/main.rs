/// DoomLike Project, 2024
///
/// Ray-caster
///
mod engine;
use engine::*;

mod settings;
use pbm::PBMImage;
use settings::*;

mod pbm;

const SETTINGS_FILE_PATH: &str = "assets/settings.toml";

fn main() -> Result<(), String> {
    let Ok(main_settings) = Settings::with_file(SETTINGS_FILE_PATH) else {
        return Err("Failed to read settings".to_string());
    };

    let image = PBMImage::with_file("assets/r1l2_ascii.pbm")?;
    println!("Image h: {}, w: {}", image.cols(), image.rows());
    println!("{image}");

    let mut platform = EngineSDL::new(main_settings.renderer)?;
    platform.start()?;
    Ok(())
}
