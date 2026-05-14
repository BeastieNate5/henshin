use std::{fs, os, path::PathBuf};

use anyhow::{Context, Result};

use crate::{appcontext::AppContext, core::paths};

pub fn track_file(mut ctx: AppContext, name: &str, path: &str) -> Result<()> {
    let resolved_path = paths::resolve_path(path)?;

    if !resolved_path.exists() {
        anyhow::bail!("'{}' does not exist", path)
    }

    let current_theme = paths::get_hsn_base()?.join("current");

    let theme_file = current_theme.join(name);

    fs::rename(&resolved_path, &theme_file).context("failed to move file")?;

    os::unix::fs::symlink(&theme_file, &resolved_path).context("failed to symlink")?;

    ctx.config.files.push(name.to_owned());
    ctx.save()?;

    println!("[ \x1b[92mOK\x1b[0m ] '{name}' added");
    Ok(())
}
