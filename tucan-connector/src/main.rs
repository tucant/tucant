pub mod html_handler;

use std::marker::PhantomData;

use encoding_rs::{Decoder, Encoding};
use futures_util::TryStreamExt as _;
use html_handler::BeforeElement;
use reqwest::{Client, ClientBuilder, Response};
use scraper::{html, Html};

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
        let mut html_handler = BeforeElement {
            element: document.root_element(),
            outer_state: (),
        };
        let html_handler = html_handler.tag_open_start("html");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.child_tag_open_start("head");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.child_tag_open_start("script");
        let html_handler = html_handler.attribute("type", "text/javascript");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();

        Ok(Tucan { client })
    }
}
