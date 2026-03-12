use crate::core::paths;
use anyhow::Result;

pub fn init_hsn(theme_dir: Option<String>) -> Result<()> {
    let state_path = paths::create_state_path();

    Ok(())
}
