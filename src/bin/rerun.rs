use std::{env, str::FromStr};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::Semaphore;
use tucan_scraper::Tucan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = dotenvy::var("DATABASE_URL")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .test_before_acquire(false)
        .connect_with(SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true))
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let tucan = Tucan {
        pool,
        client: reqwest::Client::builder().cookie_store(true).build()?,
        semaphore: Semaphore::new(10), // risky
    };

    let redirect_url = env::args().nth(1).unwrap();

    tucan.start(&redirect_url).await
}
