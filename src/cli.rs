use std::default;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.0.1", about)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        // Path to the config file containing the other config files
        #[arg(short, long)]
        config_path: String,
        // Directory to put the newly generated config(folder that will be RO) into
        #[arg(short, long)]
        output_directory: Option<String>,
    },
    Rollback {
        #[arg(short, long)]
        rollback_to: String,
    },
    Clear {
        #[arg(short, long)]
        leave_generations: usize,
    },
}
