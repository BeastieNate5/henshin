use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hsn")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add {
        path: String
    },
    Init,
    List
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { path } => {
            println!("Add {path}");
        }

        Commands::Init => {

        }
        
        Commands::List => println!("List")
    };
}
