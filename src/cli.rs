use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.0.1", about)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    create_conf: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create { config_path: String },
    Rollback { rollback_to: String },
    Clear { leave_generations: usize },
}
