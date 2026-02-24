use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    dirs::state_dir()
        .expect("Could not find state dir")
        .join("hsn")
}
