#![allow(non_camel_case_types)]
mod cli;
mod config;
mod files;

use clap::Parser;
use files::create_files;
use files::list_generations;

use crate::cli::CliArgs;
use crate::cli::Commands;
use crate::config::CreateConfig;

fn main() {
    let cli_args = CliArgs::parse();
    match cli_args.command {
        Commands::Create { config_path } => {
            let config = CreateConfig::from(config_path);
            create_files(config).unwrap();
        }
        Commands::List | Commands::Ls => {
            list_generations();
        }
        Commands::Clear {
            keep,
            skip,
            switch_to_oldest,
            switch_to_newest,
        } => {
            println!("Skipping {:?}", skip);
            todo!()
        }
        Commands::Switch { switch_to } => {
            todo!()
        }
        Commands::Delete { to_delete } => {
            todo!()
        }
    }
}
