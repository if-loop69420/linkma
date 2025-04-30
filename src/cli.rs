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
    },
    Switch {
        #[arg(short, long)]
        switch_to: usize,
    },
    Clear {
        #[arg(short, long)]
        keep: usize,
    },
    Delete {
        #[arg(short, long)]
        to_delete: usize,
    },
    List,
    Ls,
}
