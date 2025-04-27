mod cli;
mod config;

use clap::Parser;

use crate::cli::CliArgs;
use crate::config::CreateConfig;

fn main() {
    let cli_args = CliArgs::parse();
    match cli_args.command {
        cli::Commands::Create { config_path } => {
            let _config = CreateConfig::from(config_path);
            // run actual command here
        }
        _ => todo!(),
    }
}
