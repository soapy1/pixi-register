use std::fs;

use clap::Parser;

use crate::cli::common::{RegisteredEnvironment, environment_registry_path};

#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the environment to register
    name: String,
}

pub async fn execute(args: Args) {
    let registry_path = environment_registry_path();
    let data = fs::read_to_string(&registry_path).unwrap_or_else(|_| "[]".to_string());
    let envs: Vec<RegisteredEnvironment> = serde_json::from_str(&data).unwrap();

    // Find and print the environment path
    if let Some(env) = envs.iter().find(|env| env.name == args.name) {
        println!("{}", env.path);
    }
}
