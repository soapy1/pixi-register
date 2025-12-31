use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use directories::UserDirs;

// Define a struct that matches the structure of your JSON objects
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredEnvironment {
    pub name: String,
    pub path: String,
}

pub fn environment_registry_path() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Could not determine user directories");
    let register_dir = user_dirs.home_dir().join(".pixi/register");
    fs::create_dir_all(&register_dir).expect("Could not create register directory");
    return register_dir.join("environments.json");
}
