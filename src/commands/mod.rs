use crate::{cli::{Cli, Commands, ThemeCommands}, commands::init::init_hsn};

mod init;

pub fn run(cli: Cli) {
    match cli.command {
        Commands::Init { theme_dir }=> {
            match init_hsn(theme_dir) {
                Ok(_) => println!("[ OK ] dir created"),
                Err(err) => println!("[ ERROR ] {err}")
            }
        },
        Commands::Add { name, path } => {

        }
        Commands::Theme { command } => match command {
            ThemeCommands::Create => {

            },
            ThemeCommands::Load { theme } => {

            },
            ThemeCommands::List => {

            }
        }
    }
}
