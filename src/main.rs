mod common;
mod control;

mod renderer;
use renderer::*;

mod scene;
use scene::*;

mod settings;
use settings::*;

mod pbm;

mod vectors;

const SETTINGS_FILE_PATH: &str = "raycaster.toml";

fn main() -> Result<(), String> {
    let Ok(settings) = Settings::with_file(SETTINGS_FILE_PATH) else {
        return Err("Failed to read settings".to_string());
    };
    let mut scene = Scene::new(settings);
    let mut renderer = RendererSDL::new(&mut scene)?;
    renderer.run()?;
    Ok(())
}
