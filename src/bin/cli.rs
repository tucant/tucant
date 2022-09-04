// https://docs.rs/clap/latest/clap/_derive/index.html


use clap::{Parser, Subcommand};
use tucan_scraper::Tucan;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Specify your username
    username: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login
    Login {
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tucan = Tucan::new().await?;

    match cli.command {
        Commands::Login { } => {
            let password = rpassword::prompt_password("TUCAN password: ").unwrap();

            
        }
    }

    Ok(())
}