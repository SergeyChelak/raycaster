mod engine;
use engine::*;

mod settings;
use settings::*;

const SETTINGS_FILE_PATH: &str = "assets/settings.toml";

fn main() -> Result<(), String> {
    let Ok(main_settings) = Settings::with_file(SETTINGS_FILE_PATH) else {
        return Err("Failed to read settings".to_string());
    };

    let mut platform = EngineSDL::new(main_settings.renderer)?;
    platform.start()?;
    Ok(())
}
