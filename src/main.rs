use clap::{Parser, Subcommand};

use crate::cli::add;

pub mod cli;

/// Manage and share environments
#[derive(Parser, Debug)]
#[command(author, version, about = "Manage globally named environments")]
pub struct Cli {
    // Manage environments
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[command(arg_required_else_help = true)]
pub enum Command {
    /// Add an environment to the global registry
    Add(add::Args),
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Command::Add(cmd) => add::execute(cmd).await,
        }
    } else {
        std::process::exit(2);
    }
}