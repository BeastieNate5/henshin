use std::{
    fs, os, path::{Path, PathBuf}
};

use anyhow::{Context, Ok, Result};

use crate::{config::Config, core::paths};

pub struct AppContext {
    pub path: PathBuf,
    pub state_path: PathBuf,
    pub config: Config,
}

impl AppContext {
    pub fn create_new(theme_dir: String) -> Result<Self> {
        let state_path = paths::get_hsn_base()?;
        let path = state_path.join("config.toml");
        let config = Config::new(theme_dir);

        let ctx = Self { path, state_path, config };
        ctx.save()?;

        Ok(ctx)
    }

    pub fn load(state_path: PathBuf) -> Result<Self> {
        let config_path = state_path.join("config.toml");
        let config_str = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(config_str.as_str())?;

        return Ok(Self {
            path: config_path,
            state_path: state_path,
            config,
        });
    }

    pub fn find_and_load() -> Result<Self> {
        let path = paths::get_hsn_base()?;
        Self::load(path)
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
        let current_path = self.state_path.join("current");
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

    pub fn switch_current(&self, name: &str) -> Result<()> {
        let current_path = self.state_path.join("current");
        let theme_path = format!("{}/{}", self.config.theme_dir, name);

        if fs::metadata(&current_path).is_ok() {
            fs::remove_file(&current_path)?
        }

        os::unix::fs::symlink(theme_path, current_path)
            .context("Failed to set current theme pointer")?;

        Ok(())
    }

    pub fn create_misisng_theme_files(&self) -> Result<()> {
        let current_theme_path = PathBuf::from(format!("{}/{}", self.config.theme_dir, self.get_current_theme()?)); // Make this a function later
        let current_theme_files = self.get_current_theme_files()?;

        for file in self.config.files.keys() {
            if !current_theme_files.contains(file) {
                if let Err(_err) = fs::write(current_theme_path.join(file), "") {
                    eprintln!("[ \x1b[91mErr\x1b[0m ] Failed to create {file}");
                }
            }
        }

        Ok(())
    }

    fn get_current_theme_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let current_theme_path = format!("{}/{}", self.config.theme_dir, self.get_current_theme()?); // Make this a function later

        for entry in fs::read_dir(current_theme_path)? {
            let entry = entry?;
            let file_path = entry.path();

            if let Some(name) = file_path.file_name() {
               files.push(name.to_string_lossy().to_string());
            }
        }

        Ok(files)
    }
}
