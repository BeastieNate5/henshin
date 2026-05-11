use crate::{appcontext::AppContext, core::paths};
use anyhow::Result;

pub fn init_hsn(theme_dir: Option<String>) -> Result<()> {
    let theme_dir = theme_dir.as_deref();

    let state_path = paths::create_state_path()?;
    let theme_dir = paths::create_theme_dir(theme_dir)?;

    let default_theme_path = paths::create_default_theme(&theme_dir)?;
    paths::set_current_pointer(&state_path, &default_theme_path)?;

    let theme_dir_str = theme_dir.to_string_lossy().to_string();
    AppContext::create_new(theme_dir_str)?;

    println!("[ \x1b[92mOK\x1b[0m ] Init successful");
    Ok(())
}
