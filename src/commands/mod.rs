use add::track_file;
use anyhow::Result;

use crate::{
    appcontext::AppContext, cli::{Cli, Commands, ThemeCommands}, commands::init::init_hsn, core::paths
};

mod init;
mod add;

pub fn run(cli: Cli) {
    match cli.command {
        Commands::Init { theme_dir } => match init_hsn(theme_dir) {
            Ok(()) => println!("[ \x1b[92mOk\x1b[0m ] stuff"),
            Err(err) => println!("[ \x1b[91mErr\x1b[0m ] {err}")
        },
        Commands::Add { name, path } => match track_file(name.as_str(), path.as_str()) {
            Ok(()) => println!("[ \x1b[92mOk\x1b[0m ] '{name}' added"),
            Err(err) => println!("[ \x1b[91mErr\x1b[0m ] {err}")
        }
        Commands::Theme { command } => match command {
            ThemeCommands::Create => {}
            ThemeCommands::Load { theme } => {}
            ThemeCommands::List => {}
        },
    };
}
