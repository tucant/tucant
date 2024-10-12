pub mod common;
pub mod externalpages;
pub mod html_handler;
pub mod login;
pub mod mlsstart;
pub mod registration;
pub mod root;
pub mod startpage_dispatch;

use common::head::{html_head, html_head_2};
use data_encoding::HEXLOWER;
use externalpages::studveranst::veranstaltungen;
use html_extractor::html;
use html_handler::Root;
use login::{login, LoginResponse};
use mlsstart::start_page::after_login;
use regex::Regex;
use registration::index::{anmeldung, AnmeldungRequest};
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
    #[error("Tucan session timeout")]
    Timeout,
}

// TODO write small program that converts html to this format? so this is even easier

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        let client = ClientBuilder::new()
            .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
            .build()?;

        /*        let username = std::env::var("USERNAME").unwrap();
                let password = std::env::var("PASSWORD").unwrap();

                let result = login(&client, username.as_str(), password.as_str()).await?;
                println!("{:?}", result);
        */

        let result = LoginResponse {
            id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
            cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
        };

        let anmeldung_response = anmeldung(&client, &result, AnmeldungRequest::new()).await?;

        println!("{anmeldung_response:#?}");
        let entry = &anmeldung_response.entries[2];
        let anmeldung_response = anmeldung(&client, &result, entry.1.to_owned()).await?;
        println!(" {anmeldung_response:#?}");

        for entry in anmeldung_response.entries {
            let anmeldung_response = anmeldung(&client, &result, entry.1.to_owned()).await?;
            println!("  {anmeldung_response:#?}");
        }

        Ok(Self { client })
    }
}
