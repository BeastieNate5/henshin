use crate::core::paths::{self, create_theme_dir, set_current_pointer};
use anyhow::Result;

pub fn init_hsn(theme_dir: Option<String>) -> Result<()> {
    let theme_dir = theme_dir.as_deref();

    let state_path = paths::create_state_path()?;
    create_theme_dir(theme_dir)?;
    set_current_pointer(&state_path, theme_dir)?;

    Ok(())
}
