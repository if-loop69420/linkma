mod cli;
mod config;
mod files;

use std::cell::LazyCell;

use clap::Parser;
use files::create_files;

use crate::cli::CliArgs;
use crate::config::CreateConfig;

fn main() {
    let cli_args = CliArgs::parse();
    match cli_args.command {
        cli::Commands::Create { config_path } => {
            let config = CreateConfig::from(config_path);
            create_files(config).unwrap();
        }
        _ => todo!(),
    }
}
