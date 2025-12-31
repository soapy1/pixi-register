use std::fs;

use clap::{Parser};

use crate::cli::common::{RegisteredEnvironment, environment_registry_path};


#[derive(Parser, Debug)]
pub struct Args {}

pub async fn execute(_args: Args) {
   let registry_path = environment_registry_path();
    let data = fs::read_to_string(&registry_path).unwrap_or_else(|_| "[]".to_string());
    let envs: Vec<RegisteredEnvironment> = serde_json::from_str(&data).unwrap();
    
    println!("Registered environments:");
    // Check if name already exists
    envs.iter().for_each(|env| {
        println!("* {}: {}", env.name, env.path);
    });
}