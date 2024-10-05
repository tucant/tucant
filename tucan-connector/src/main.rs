pub mod html_handler;

use data_encoding::HEXLOWER;
use html_handler::Root;
use reqwest::{Client, ClientBuilder};
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
        let html_handler = Root::new(document.tree.root());
        let html_handler = html_handler.document_start();
        let html_handler = html_handler.doctype();
        let html_handler = html_handler.tag_open_start("html");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.child_tag_open_start("head");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.skip_comment("RMGklg_XASh8hhew3hZIhYXmZF9hdbOOrS4pTp7U4-Q");
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.child_tag_open_start("script");
        let html_handler = html_handler.attribute("type", "text/javascript");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.child_tag_open_start("title");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.skip_text("Technische Universität Darmstadt");
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.child_tag_open_start("meta");
        let html_handler = html_handler.attribute("http-equiv", "X-UA-Compatible");
        let html_handler = html_handler.attribute("content", "IE=EmulateIE9");
        let html_handler = html_handler.tag_open_end();

        let html_handler = html_handler.child_tag_open_start("meta");
        let html_handler = html_handler.attribute("cache-control", "no-cache");
        let html_handler = html_handler.tag_open_end();

        Ok(Self { client })
    }
}
