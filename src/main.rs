mod cli;
mod config;

use clap::Parser;

use crate::cli::CliArgs;

fn main() {
    let _cli_args = CliArgs::parse();
}
