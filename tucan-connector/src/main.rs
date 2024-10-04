pub mod html_handler;

use std::marker::PhantomData;

use encoding_rs::{Decoder, Encoding};
use futures_util::TryStreamExt as _;
use html_handler::{BeforeElement, HtmlHandler};
use reqwest::{Client, ClientBuilder, Response};
use scraper::Html;

fn main() -> Result<(), TucanError> {
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

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        let client = ClientBuilder::new().build()?;
        let resp = client
            .get("https://www.tucan.tu-darmstadt.de/")
            .send()
            .await?
            .error_for_status()?;
        println!("{resp:#?}");
        let content = resp.text().await?;
        let document = Html::parse_document(&content);
        println!("{}", document.html());
        let mut html_handler = HtmlHandler {
            state: BeforeElement {
                element: document.root_element(),
            },
            phantom_data: PhantomData,
        };
        let html_handler = html_handler.tag_open_start("html");
        Ok(Tucan { client })
    }
}
