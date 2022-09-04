// https://docs.rs/clap/latest/clap/_derive/index.html
use clap::{Parser, Subcommand};
use tucan_scraper::tucan::Tucan;

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
    Login {},

    /// Logged in commands
    #[clap(flatten)]
    LoggedInCommands(LoggedInCommands),
}

#[derive(Subcommand)]
enum LoggedInCommands {
    // Show registration
    Registration { url: Option<String> },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let tucan = Tucan::new().await?;

    match cli.command {
        Commands::Login {} => {
            let password = rpassword::prompt_password("TUCAN password: ").unwrap();

            tucan.login(&cli.username, &password).await?;
        }
        Commands::LoggedInCommands(logged_in_commands) => {
            let tucan = tucan.continue_session(&cli.username).await?;
            match logged_in_commands {
                LoggedInCommands::Registration { url } => {
                    let registration = tucan.registration(url).await?;

                    println!("{:#?}", registration);
                }
            }
        }
    }

    Ok(())
}
