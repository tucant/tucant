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
    Login {},

    /// Logged in commands
    #[clap(flatten)]
    LoggedInCommands(LoggedInCommands),
}

#[derive(Subcommand)]
enum LoggedInCommands {
    // Scrape all modules
    ScrapeModules {},
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
                LoggedInCommands::ScrapeModules {} => {
                    /*
                    let redirect_url = &format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        &res_headers.headers().get("refresh").unwrap().to_str()?[7..]
                    );

                    res_headers.text().await?;

                    let cnt = sqlx::query!(
                        "INSERT INTO entrypoints (entrypoint_url) VALUES (?)",
                        redirect_url
                    )
                    .execute(&tucan.pool)
                    .await?;
                    assert_eq!(cnt.rows_affected(), 1);

                    tucan.start(redirect_url).await*/
                }
            }
        }
    }

    Ok(())
}
