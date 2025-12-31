use std::fs;
use std::path::PathBuf;
use std::path::Path;
use clap::Parser;

use crate::cli::common::{RegisteredEnvironment, environment_registry_path};

#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the environment to register
    #[arg(long, short)]
    name: String,

    /// The path to `pixi.toml`, `pyproject.toml`, or the project directory
    #[arg(long, short, default_value = ".")]
    manifest_path: PathBuf,
}

fn register_environment(name: &str, manifest_path: &Path) -> Result<(), String> {
    let registry_path = environment_registry_path();
    let data = fs::read_to_string(&registry_path).unwrap_or_else(|_| "[]".to_string());
    let mut envs: Vec<RegisteredEnvironment> = serde_json::from_str(&data).unwrap();

    // Check if name already exists
    if envs.iter().any(|env| env.name == name) {
        return Err(format!(
            "Environment name '{}' already exists in the registry",
            name
        ));
    }

    envs.push(RegisteredEnvironment {
        name: name.to_string(),
        path: manifest_path.to_string_lossy().into(),
    });
    fs::write(&registry_path, serde_json::to_string_pretty(&envs).unwrap()).unwrap();
    Ok(())
}

pub async fn execute(args: Args) {
    match fs::canonicalize(&args.manifest_path) {
        Ok(absolute_path) => match register_environment(&args.name, &absolute_path) {
            Ok(_) => {
                println!("Successfully registered environment '{}'", &args.name);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error canonicalizing path: {}", e);
            std::process::exit(1);
        }
    }
}
