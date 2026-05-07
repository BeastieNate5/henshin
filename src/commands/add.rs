use std::{fs, os, path::PathBuf};

use anyhow::{Context, Result};

use crate::{appcontext::AppContext, core::paths};

pub fn track_file(name: &str, path: &str) -> Result<()> {
    let mut ctx = AppContext::find_and_load()
        .context("Failed to read config, please run henshin init")?;

    let path_expanded = shellexpand::tilde(path).into_owned();
    let path_expanded_buf = PathBuf::from(path_expanded);

    if !path_expanded_buf.exists() {
        anyhow::bail!("'{}' does not exist", path)
    }

    let current_theme = paths::get_hsn_base()?
        .join("current");

    let theme_file = current_theme.join(name);

    fs::rename(path, &theme_file)
        .context("failed to move file")?;

    os::unix::fs::symlink(&theme_file, path)
        .context("failed to symlink")?;

    ctx.config.files.insert(name.to_owned(), path.to_owned());
    ctx.save()?;

    Ok(())
}
