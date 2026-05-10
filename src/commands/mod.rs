use crate::{
    appcontext::AppContext, cli::{Cli, Commands}, commands::init::init_hsn
};

mod init;
mod add;
mod theme;

pub fn run(cli: Cli) {
    let result = match cli.command {
        Commands::Init { theme_dir } => init_hsn(theme_dir),
        cmd => {
            let ctx = AppContext::find_and_load()
                .expect("Failed to read config, please run init");
            match cmd {
                Commands::Add { name, path } => add::track_file(ctx, name.as_str(), path.as_str()),
                Commands::Theme { command } => theme::handle_comamnd(ctx, command),
                _ => unreachable!()
            }
        }
    };

    if let Err(err) = result {
        eprintln!("[ \x1b[91mErr\x1b[0m ] {:?}", err);
        std::process::exit(1);
    }
}
