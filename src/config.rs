use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    theme_dir: String,
    files: HashMap<String, String>,
    hooks: Hooks
}

#[derive(Debug, Serialize, Deserialize)]
struct Hooks {
    post_load: Vec<String> 
}

impl Config {
    pub fn new(theme_dir: String) -> Self {
        Self {
            theme_dir: theme_dir,
            files: HashMap::new(),
            hooks: Hooks { post_load: Vec::new() }
        } 
    }
}
