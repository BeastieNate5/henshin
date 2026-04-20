use crate::{appcontext::AppContext, core::paths};
use anyhow::Result;

pub fn init_hsn(theme_dir: Option<String>) -> Result<()> {
    let theme_dir = theme_dir.as_deref();

    let state_path = paths::create_state_path()?;
    let theme_dir = paths::create_theme_dir(theme_dir)?;
    paths::set_current_pointer(&state_path, &theme_dir)?;

    let theme_dir_str = theme_dir.to_string_lossy().to_string();
    AppContext::create_new(&state_path, theme_dir_str)?;

    Ok(())
}
