use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme_dir: PathBuf,
    pub files: Vec<String>,
}

impl Config {
    pub fn new(theme_dir: String) -> Self {
        Self {
            theme_dir: PathBuf::from(theme_dir),
            files: Vec::new(),
        }
    }
}
