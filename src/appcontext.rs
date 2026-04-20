use std::{fs, path::{Path, PathBuf}};

use anyhow::{Context, Ok, Result};

use crate::config::Config;

pub struct AppContext {
    pub path: PathBuf,
    pub config: Config
}

impl AppContext {
    pub fn create_new(state_path: &Path, theme_dir: String) -> Result<Self> {
       let path = state_path.join("config.toml");
       let config = Config::new(theme_dir);

       let ctx = Self { path, config };
        ctx.save()?;

       Ok(ctx)
    }


    pub fn save(&self) -> Result<()> {
        let toml_string = toml::to_string_pretty(&self.config)
            .context("Failed to serailize config")?;

        fs::write(&self.path, &toml_string)
            .with_context(|| format!("Failed to write config to {:?}", self.path))?;

        Ok(())
    }
}
