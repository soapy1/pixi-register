use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use directories::UserDirs;

use clap::{Parser};


#[derive(Parser, Debug)]
#[command(name="pixi-register", version, about = "Register named environments")]
struct Cli {
    /// Name of the environment to register
    #[arg(long, short)]
    name: String,

    /// The path to `pixi.toml`, `pyproject.toml`, or the project directory
    #[arg(long, short, default_value = ".")]
    manifest_path: PathBuf,
}

// Define a struct that matches the structure of your JSON objects
#[derive(Debug, Serialize, Deserialize)]
struct RegisteredEnvironment {
    name: String,
    path: String,
}

fn environment_registry() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Could not determine user directories");
    let register_dir = user_dirs.home_dir().join(".pixi/register");
    fs::create_dir_all(&register_dir).expect("Could not create register directory");
    return register_dir.join("environments.json");
}

fn register_environment(name: &str, manifest_path: &PathBuf) -> Result<(), String> {
    let registry_path = environment_registry();
    let data = fs::read_to_string(&registry_path).unwrap_or_else(|_| "[]".to_string());
    let mut envs: Vec<RegisteredEnvironment> = serde_json::from_str(&data).unwrap();
    
    // Check if name already exists
    if envs.iter().any(|env| env.name == name) {
        return Err(format!("Environment name '{}' already exists in the registry", name));
    }
    
    envs.push(RegisteredEnvironment { name: name.to_string(), path: manifest_path.to_string_lossy().into() });
    fs::write(&registry_path, serde_json::to_string_pretty(&envs).unwrap()).unwrap();
    Ok(())
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match fs::canonicalize(&cli.manifest_path) {
        Ok(absolute_path) => {
            match register_environment(&cli.name, &absolute_path) {
                Ok(_) => {
                    println!("Successfully registered environment '{}'", &cli.name);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error canonicalizing path: {}", e);
            std::process::exit(1);
        }
    }

}
