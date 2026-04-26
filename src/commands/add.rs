use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::appcontext::AppContext;


pub fn track_file(name: &str, path: &str) -> Result<()> {
    let mut ctx = AppContext::find_and_load()
        .context("Failed to read config, please run henshin init")?;
    let path_expanded = shellexpand::tilde(path).into_owned();
    let path_expanded_buf = PathBuf::from(path_expanded);

    if !path_expanded_buf.exists() {
        anyhow::bail!("'{}' does not exist", path)
    }

    ctx.config.files.insert(name.to_owned(), path.to_owned());
    ctx.save()?;

    Ok(())
}
