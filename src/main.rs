use clap::Parser;

use crate::cli::Cli;

mod cli;
mod commands;
mod core;

fn main() {
    let cli = Cli::parse();
    commands::run(cli);
}
