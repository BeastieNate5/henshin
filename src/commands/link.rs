use anyhow::{Result, anyhow};

use crate::{appcontext::AppContext, core::paths};

pub fn link_file_to_theme(ctx: AppContext, theme_file: &str, path: &str) -> Result<()> {
    let resolved_path = paths::resolve_path(path)?;

    if resolved_path.exists() {
        anyhow::bail!("'{}' already exists", path)
    }

    let theme_files = ctx.get_current_theme_files()?;

    if !theme_files.contains(&theme_file.to_string()) {
        anyhow::bail!("{theme_file} does not exist");
    }

    ctx.link_file_to_theme_file(theme_file, path)?;

    println!("[ \x1b[92mOK\x1b[0m ] linked file");
    Ok(())
}
