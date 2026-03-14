use std::{fs, os, path::{Path, PathBuf}};
use anyhow::{Context, Ok, Result, anyhow};

pub fn create_state_path() -> Result<PathBuf> {
    let path = dirs::state_dir()
        .context("XDG_STATE_HOME not found")?
        .join("hsn");

    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn create_theme_dir(theme_dir: Option<&str>) -> Result<()> {
    let theme_dir = match theme_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let mut theme_path = get_state_path()?;
            theme_path.push("themes"); 
            theme_path
        }
    };


    if !theme_dir.exists() {
        fs::create_dir_all(theme_dir)?;
    }

    Ok(())
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

pub fn set_current_pointer(state_path: PathBuf, theme_dir: Option<&str>) -> Result<()> {
    let theme_dir = match theme_dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let mut theme_dir = state_path.clone();
            theme_dir.push("themes");
            theme_dir
        }
    };

    let mut current_pointer_path = state_path.clone();
    current_pointer_path.push("current");

    if !current_pointer_path.exists() {
        os::unix::fs::symlink(theme_dir, current_pointer_path)
            .context("Failed to create current pointer")?;
    }

    Ok(())
}

