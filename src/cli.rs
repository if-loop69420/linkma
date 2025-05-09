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
        #[arg(short, long, value_delimiter = ',', group = "skip_logic")]
        skip: Vec<usize>,
        #[arg(short, long, group = "skip_logic")]
        switch_to_oldest: bool,
        #[arg(short, long, group = "skip_logic")]
        switch_to_newest: bool,
    },
    Delete {
        #[arg(short, long)]
        to_delete: usize,
    },
    List,
    Ls,
}
