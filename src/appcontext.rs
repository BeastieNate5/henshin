use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result};

use crate::{config::Config, core::paths};

pub struct AppContext {
    pub path: PathBuf,
    pub config: Config,
}

impl AppContext {
    pub fn create_new(state_path: &Path, theme_dir: String) -> Result<Self> {
        let path = state_path.join("config.toml");
        let config = Config::new(theme_dir);

        let ctx = Self { path, config };
        ctx.save()?;

        Ok(ctx)
    }

    pub fn load(state_path: &Path) -> Result<Self> {
        let config_path = state_path.join("config.toml");
        let config_str = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(config_str.as_str())?;

        return Ok(Self {
            path: config_path,
            config,
        });
    }

    pub fn find_and_load() -> Result<Self> {
        let path = paths::get_hsn_base()?;
        Self::load(&path)
    }

    pub fn save(&self) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(&self.config).context("Failed to serailize config")?;

        fs::write(&self.path, &toml_string)
            .with_context(|| format!("Failed to write config to {:?}", self.path))?;

        Ok(())
    }

    pub fn list_themes(&self) -> Result<Vec<String>> {
        let mut themes = Vec::new();
        let theme_dir = &self.config.theme_dir;

        for entry in fs::read_dir(&theme_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(theme_name) = path.file_name().and_then(|f| f.to_str()) {
                    themes.push(theme_name.to_string());
                }
            }
        }

        Ok(themes)
    }

    pub fn get_current_theme(&self) -> Result<String> {
        let current_path = paths::get_hsn_base()?.join("current");
        let current_theme_path = fs::read_link(current_path)?;
        let current_theme = current_theme_path.file_name().unwrap();

        Ok(current_theme.to_string_lossy().to_string())
    }

    pub fn create_blank_theme(&self, name: &str) -> Result<()> {
        let theme_path = format!("{}/{}", self.config.theme_dir, name);
        fs::create_dir(theme_path).context("Failed to create theme directory")?;
        Ok(())
    }

    pub fn delete_theme(&self, name: &str) -> Result<()> {
        let theme_path = format!("{}/{}", self.config.theme_dir, name);
        fs::remove_dir_all(theme_path).context("Failed to delete theme directory")?;
        Ok(())
    }
}
