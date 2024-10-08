pub mod common;
pub mod externalpages;
pub mod html_handler;
pub mod login;
pub mod root;
pub mod startpage_dispatch;

use common::head::{html_head, html_head_2};
use data_encoding::HEXLOWER;
use html_extractor::html;
use html_handler::Root;
use login::login;
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

        let result = login(&client, username.as_str(), password.as_str()).await?;

        let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N{},-N000019,-N000000000000000", result.id))
                .header("Cookie", format!("cnsc={}", result.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
        println!("{response:#?}");
        let content = response.text().await?;
        let document = Html::parse_document(&content);
        println!("{}", document.root_element().html());
        let html_handler = Root::new(document.tree.root());
        let html_handler = html_handler.document_start();
        let html_handler = html_handler.doctype();
        html!(
            <html>
            <head>_
        );
        let html_handler = html_head_2(html_handler);
        html!(
            </head>_
            <body class="redirect">_
            <div id="wrapper">_
                        <a href="http://http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                                <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                        </a>_
                        <!-- "MA-hDUoCrkYqlM3RsS9EUjq0y_UcuN1AB82k4O5O8YU" -->_
                        <h2><a href=href_link_1>"Sie werden zur Startseite weitergeleitet ..."</a></h2>_
                        <a style="text-decoration: underline;" href=href_link_2>"Startseite"</a>_
                </div>_
                <div id="sessionId" style="display: none;">session_id</div>_
                <!-- "automatic redirect, no meta because firefox stops due to multiple redirects" -->_
                <script>
                "ewfweff"
                </script>
        );

        Ok(Self { client })
    }
}
