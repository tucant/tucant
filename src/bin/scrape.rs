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

    let username = env::args().nth(1).unwrap();

    let password = env::args().nth(2).unwrap();
    //let password = rpassword::prompt_password("TUCAN password: ").unwrap();

    let params: [(&str, &str); 10] = [
        ("usrname", &username),
        ("pass", &password),
        ("APPNAME", "CampusNet"),
        ("PRGNAME", "LOGINCHECK"),
        (
            "ARGUMENTS",
            "clino,usrname,pass,menuno,menu_type,browser,platform",
        ),
        ("clino", "000000000000001"),
        ("menuno", "000344"),
        ("menu_type", "classic"),
        ("browser", ""),
        ("platform", ""),
    ];
    let res_headers = tucan
        .client
        .post("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll")
        .form(&params)
        .send()
        .await?;

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

    tucan.start(redirect_url).await
}
