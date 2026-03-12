use std::{fs, os, path::{Path, PathBuf}};
use anyhow::{Context, Ok, Result, anyhow};

pub fn create_state_path() -> Result<PathBuf> {
    let path = dirs::state_dir()
        .context("XDG_STATE_HOME not found")?
        .join("hsn");

    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn get_state_path() -> Result<PathBuf> {
    let path = dirs::state_dir()
        .context("XDG_STATE_HOME not found")?
        .join("hsn");

    if !path.exists() {
        return Err(anyhow!("Config file not found at {:?}", path));
    }

    Ok(path)
}

fn set_current_pointer(theme_dir: Option<String>) -> Result<()> {
    os::unix::fs::symlink(original, link)
    Ok(())
}
