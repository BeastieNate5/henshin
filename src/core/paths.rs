use anyhow::{Context, Ok, Result};
use std::{
    fs, os,
    path::{Path, PathBuf},
};

pub fn get_hsn_base() -> Result<PathBuf> {
    dirs::state_dir()
        .context("Could not find XDG_STATE_HOME")
        .map(|p| p.join("hsn"))
}

pub fn create_state_path() -> Result<PathBuf> {
    let path = get_hsn_base()?;
    fs::create_dir_all(&path).context("Failed to create state directory")?;
    Ok(path)
}

pub fn create_theme_dir<P: AsRef<Path>>(theme_dir: Option<P>) -> Result<()> {
    let theme_dir = match theme_dir {
        Some(dir) => PathBuf::from(dir.as_ref()),
        None => get_hsn_base()?.join("themes"),
    };

    if !theme_dir.exists() {
        fs::create_dir_all(&theme_dir)
            .with_context(|| format!("Failed to create theme dir at {:?}", theme_dir))?;
    }

    Ok(())
}

pub fn set_current_pointer<P: AsRef<Path>>(state_path: &Path, theme_dir: Option<P>) -> Result<()> {
    let theme_dir = match theme_dir {
        Some(dir) => PathBuf::from(dir.as_ref()),
        None => state_path.join("themes"),
    };

    let current_pointer_path = state_path.join("current");

    if !current_pointer_path.exists() {
        os::unix::fs::symlink(theme_dir, current_pointer_path)
            .context("Failed to create current pointer")?;
    }

    Ok(())
}
