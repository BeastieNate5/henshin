use clap::Parser;

use crate::cli::Cli;

mod cli;
mod commands;
mod core;
mod config;
mod appcontext;

fn main() {
    let cli = Cli::parse();
    commands::run(cli);
}
