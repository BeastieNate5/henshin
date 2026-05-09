use crate::appcontext::AppContext;
use anyhow::{Context, Result};

pub fn create_theme(name: &str) {

}

pub fn list_themes() -> Result<()> {
    let ctx = AppContext::find_and_load()
        .context("Failed to read config, please run henshin init")?;

    let themes = ctx.list_themes()?;

    for theme in themes {
        println!("{theme}")
    }

    Ok(())
}
