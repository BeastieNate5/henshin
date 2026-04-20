use crate::core::paths;
use anyhow::Result;

pub fn init_hsn(theme_dir: Option<String>) -> Result<()> {
    let theme_dir = theme_dir.as_deref();

    let state_path = paths::create_state_path()?;
    let theme_dir = paths::create_theme_dir(theme_dir)?;
    paths::set_current_pointer(&state_path, &theme_dir)?;
    paths::generate_config_file(&state_path, &theme_dir)?;

    Ok(())
}
