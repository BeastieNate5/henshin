use clap::Parser;

use crate::cli::Cli;

mod appcontext;
mod cli;
mod commands;
mod config;
mod core;

fn main() {
    let cli = Cli::parse();
    commands::run(cli);
}
