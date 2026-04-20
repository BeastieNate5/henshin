use anyhow::Result;

use crate::{
    cli::{Cli, Commands, ThemeCommands}, commands::init::init_hsn
};

mod init;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { theme_dir } =>  {
            match init_hsn(theme_dir) {
                Ok(_) => println!("[ \x1b[92mOK\x1b[0m ] dir successful"),
                Err(err) => println!("[ \x1b[91mERR\x1b[0m ] {err}"),
            }
        }
        Commands::Add { name, path } => {}
        Commands::Theme { command } => match command {
            ThemeCommands::Create => {}
            ThemeCommands::Load { theme } => {}
            ThemeCommands::List => {}
        },
    };

    Ok(())
}
