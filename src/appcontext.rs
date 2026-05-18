use std::{
    fs, os,
    path::PathBuf,
};

use anyhow::{Context, Ok, Result};
use std::process::Command;
use fs_extra::dir::CopyOptions;

use crate::{config::Config, core::paths};

pub struct AppContext {
    pub config_path: PathBuf,
    pub state_path: PathBuf,
    pub config: Config,
}

impl AppContext {
    pub fn create_new(theme_dir: String) -> Result<Self> {
        let state_path = paths::get_hsn_state_path()?;
        let config_path = paths::get_hsn_config_path()?.join("config.toml");
        let config = Config::new(theme_dir);

        let ctx = Self {
            config_path,
            state_path,
            config,
        };
        ctx.save()?;

        Ok(ctx)
    }

    pub fn load() -> Result<Self> {
        let state_path = paths::get_hsn_state_path()?;
        let config_path = paths::get_hsn_config_path()?.join("config.toml");

        let config_str = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(config_str.as_str())?;

        Ok(Self {
            config_path,
            state_path,
            config,
        })
    }

    pub fn save(&self) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(&self.config).context("Failed to serailize config")?;

        fs::write(&self.config_path, &toml_string)
            .with_context(|| format!("Failed to write config to {:?}", self.config_path))?;

        Ok(())
    }

    pub fn ensure_initialized(&self) -> Result<()> {
        let current_path = self.state_path.join("current");

        if !self.state_path.exists() {
            paths::create_state_path()?;

            if !current_path.exists() {
                let default_theme_path = self.config.theme_dir.join("default");

                if !default_theme_path.exists() {
                    paths::create_default_theme(&self.config.theme_dir)?;
                }

                paths::set_current_pointer(&self.state_path, &default_theme_path)?;
                println!("[ \x1b[94mINFO\x1b[0m ] Re-initialized missing system state pointer.");
            }
        }

        Ok(())
    }

    pub fn list_themes(&self) -> Result<Vec<String>> {
        let mut themes = Vec::new();
        let theme_dir = &self.config.theme_dir;

        for entry in fs::read_dir(theme_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir()
                && let Some(theme_name) = path.file_name().and_then(|f| f.to_str())
            {
                themes.push(theme_name.to_string());
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
        let theme_path = self.config.theme_dir.join(name);
        fs::create_dir(theme_path).context("Failed to create theme directory")?;
        Ok(())
    }

    pub fn create_theme_from(&self, from_theme_name: &str, new_theme_name: &str) -> Result<()> {
        let from_theme_path = self.config.theme_dir.join(from_theme_name);
        let new_theme_path = self.config.theme_dir.join(new_theme_name);

        let mut options = CopyOptions::new();
        options.copy_inside = true;

        fs_extra::dir::copy(from_theme_path, new_theme_path, &options)?;
        Ok(())
    }

    pub fn delete_theme(&self, name: &str) -> Result<()> {
        let theme_path = self.config.theme_dir.join(name);
        fs::remove_dir_all(theme_path).context("Failed to delete theme directory")?;
        Ok(())
    }

    pub fn switch_current(&self, name: &str) -> Result<()> {
        let current_path = self.state_path.join("current");
        let theme_path = self.config.theme_dir.join(name);

        if fs::metadata(&current_path).is_ok() {
            fs::remove_file(&current_path)?
        }

        os::unix::fs::symlink(theme_path, current_path)
            .context("Failed to set current theme pointer")?;

        Ok(())
    }

    pub fn create_misisng_theme_files(&self) -> Result<()> {
        let current_theme_path = self.config.theme_dir.join(self.get_current_theme()?);
        let current_theme_files = self.get_current_theme_files()?;

        for file in self.config.files.iter() {
            if !current_theme_files.contains(file)
                && let Err(_err) = fs::write(current_theme_path.join(file), "")
            {
                eprintln!("[ \x1b[91mErr\x1b[0m ] Failed to create {file}");
            }
        }

        Ok(())
    }

    pub fn get_current_theme_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        let current_theme_path = self.config.theme_dir.join(self.get_current_theme()?);

        for entry in fs::read_dir(current_theme_path)? {
            let entry = entry?;
            let file_path = entry.path();

            if let Some(name) = file_path.file_name() {
                files.push(name.to_string_lossy().to_string());
            }
        }

        Ok(files)
    }

    pub fn link_file_to_theme_file(&self, theme_file_name: &str, file_path: &str) -> Result<()> {
        let theme_file_path = self.state_path.join("current").join(theme_file_name);
        os::unix::fs::symlink(theme_file_path, file_path)?;
        Ok(())
    }

    pub fn execute_hook(&self) -> Result<()> {
        let current_path = self.state_path.join("current");
        let hook_path = current_path.join("theme_hook");

        if !hook_path.exists() {
            return Ok(())
        }

        if hook_path.is_file() {
            println!("[ \x1b[94mINFO\x1b[0m ] Executing hook");

            let status = Command::new("sh")
                .arg(&hook_path)
                .current_dir(&current_path)
                .status()?;

            if !status.success() {
                eprintln!("[ \x1b[91mERR\x1b[0m ]Hook exited with error code")
            }
        }

        Ok(())
    }
}
