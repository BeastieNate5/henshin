use clap::Parser;

use crate::cli::Cli;

mod cli;
mod core;
mod commands;

fn main() {
    let cli = Cli::parse();
    commands::run(cli);
}
