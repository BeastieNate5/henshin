use crate::cli::{Cli, Commands, ThemeCommands};


pub fn run(cli: Cli) {
    match cli.command {
        Commands::Init => {

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
