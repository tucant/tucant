pub mod externalpages;
pub mod html_handler;
pub mod login;
pub mod root;
pub mod startpage_dispatch;

use data_encoding::HEXLOWER;
use html_extractor::html;
use html_handler::Root;
use regex::Regex;
use reqwest::{header::HeaderValue, Client, ClientBuilder};
use scraper::Html;

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = Tucan::new().await?;
    Ok(())
}

pub struct Tucan {
    client: Client,
}

#[derive(thiserror::Error, Debug)]
pub enum TucanError {
    #[error("HTTP error {0:?}")]
    Http(#[from] reqwest::Error),
    #[error("IO error {0:?}")]
    Io(#[from] std::io::Error),
}

// TODO write small program that converts html to this format? so this is even easier

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        let client = ClientBuilder::new()
            .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
            .build()?;

        let username = std::env::var("USERNAME").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        /*
                let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N{id},-N000019,-N000000000000000"))
                .send()
                .await?
                .error_for_status()?;
                println!("{response:#?}");
                let content = response.text().await?;
                let document = Html::parse_document(&content);
                println!("{}", document.html());
        */
        Ok(Self { client })
    }
}
