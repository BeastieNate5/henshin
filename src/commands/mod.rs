use anyhow::{Ok, Result};

use crate::{
    appcontext::AppContext, cli::{Cli, Commands, ThemeCommands}, commands::init::init_hsn, core::paths
};

mod init;

pub fn run(cli: Cli) -> Result<()> {
    let state_path = paths::get_hsn_base().map_err(|err| {
        println!("[ - ] {err}");
        
    });

    match cli.command {
        Commands::Init { theme_dir } => init_hsn(theme_dir)?,
        Commands::Add { name, path } => {
        }
        Commands::Theme { command } => match command {
            ThemeCommands::Create => {}
            ThemeCommands::Load { theme } => {}
            ThemeCommands::List => {}
        },
    };

    Ok(())
}
