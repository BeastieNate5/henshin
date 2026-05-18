use anyhow::{Context, Ok, Result};
use std::{
    env, fs, os,
    path::{Path, PathBuf},
};

pub fn get_hsn_state_path() -> Result<PathBuf> {
    dirs::state_dir()
        .context("Could not find XDG_STATE_HOME")
        .map(|p| p.join("hsn"))
}

pub fn get_hsn_config_path() -> Result<PathBuf> {
    dirs::config_dir()
        .context("Could not find XDG_CONFIG")
        .map(|p| p.join("hsn"))
}

pub fn create_state_path() -> Result<PathBuf> {
    let path = get_hsn_state_path()?;
    fs::create_dir_all(&path).context("Failed to create state directory")?;
    Ok(path)
}

pub fn create_state_path() -> Result<PathBuf> {
    let path = get_hsn_state_path()?;
    fs::create_dir_all(&path).context("Failed to create state directory")?;
    Ok(path)
}

pub fn create_theme_dir<P: AsRef<Path>>(theme_dir: Option<P>) -> Result<PathBuf> {
    let theme_dir = match theme_dir {
        Some(dir) => PathBuf::from(dir.as_ref()),
        None => get_hsn_config_path()?.join("themes"),
    };

    if !theme_dir.exists() {
        fs::create_dir_all(&theme_dir)
            .with_context(|| format!("Failed to create theme dir at {:?}", theme_dir))?;
    }

    Ok(theme_dir)
}

pub fn set_current_pointer<P: AsRef<Path>>(state_path: &Path, theme_dir: P) -> Result<()> {
    let current_pointer_path = state_path.join("current");

    if !current_pointer_path.exists() {
        os::unix::fs::symlink(theme_dir, current_pointer_path)
            .context("Failed to create current pointer")?;
    }

    Ok(())
}

pub fn create_default_theme<P: AsRef<Path>>(theme_dir: P) -> Result<PathBuf> {
    let default_theme_path = theme_dir.as_ref().join("default");
    fs::create_dir_all(&default_theme_path).context("Failed to create default theme")?;
    Ok(default_theme_path)
}

pub fn resolve_path(path: &str) -> Result<PathBuf> {
    let expanded = shellexpand::tilde(path).into_owned();
    let p = PathBuf::from(expanded);

    if p.is_absolute() {
        Ok(p)
    } else {
        Ok(env::current_dir()
            .context("Failed to resolve path")?
            .join(path))
    }
}
