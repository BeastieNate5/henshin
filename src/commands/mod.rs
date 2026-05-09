use add::track_file;

use crate::{
    cli::{Cli, Commands, ThemeCommands}, commands::init::init_hsn
};

mod init;
mod add;
mod theme;

pub fn run(cli: Cli) {
    match cli.command {
        Commands::Init { theme_dir } => match init_hsn(theme_dir) {
            Ok(()) => println!("[ \x1b[92mOk\x1b[0m ] Init successful"),
            Err(err) => println!("[ \x1b[91mErr\x1b[0m ] {err}")
        },
        Commands::Add { name, path } => match track_file(name.as_str(), path.as_str()) {
            Ok(()) => println!("[ \x1b[92mOk\x1b[0m ] '{name}' added"),
            Err(err) => println!("[ \x1b[91mErr\x1b[0m ] {err}")
        }
        Commands::Theme { command } => match command {
            ThemeCommands::Create { name } => {
                theme::create_theme(name.as_str());
            }
            ThemeCommands::Load { theme } => {}
            ThemeCommands::List => if let Err(err) = theme::list_themes() {
                println!("[ \x1b[91mErr\x1b[0m ] {err}")
            }
        },
    };
}
