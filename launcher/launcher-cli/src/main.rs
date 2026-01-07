mod config;
mod launch;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "launcher-cli",
    version,
    about = "Launches and manages ORE launcher flows"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Launch the full program, mint, LP pool, and vault workflow.
    Launch {
        /// Path to launch.toml or launch.json.
        #[arg(short, long, value_name = "PATH")]
        config: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Launch { config } => launch::run(config),
    }
}
