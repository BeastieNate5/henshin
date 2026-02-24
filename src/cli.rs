use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hsn")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        name: String,
        path: String
    },
    Theme {
        #[command(subcommand)]
        command: ThemeCommands
    },
    Init
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    Create,
    Load {
        theme: String
    },
    List
}
