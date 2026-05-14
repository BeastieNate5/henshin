use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme_dir: PathBuf,
    pub files: HashMap<String, String>,
    hooks: Hooks,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hooks {
    pub post_load: Vec<String>,
}

impl Config {
    pub fn new(theme_dir: String) -> Self {
        Self {
            theme_dir: PathBuf::from(theme_dir),
            files: HashMap::new(),
            hooks: Hooks {
                post_load: Vec::new(),
            },
        }
    }
}
