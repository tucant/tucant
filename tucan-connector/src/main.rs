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

macro_rules! make_html {
    ($html_handler: ident <$tag: literal $($attr_name:literal = $attr_value:literal)*>) => {
        let html_handler = $html_handler.next_child_tag_open_start($tag);
        $(
        let html_handler = html_handler.attribute($attr_name, $attr_value);
        )*
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
    };
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
        macro_rules! html {
            ($($rest: tt)*) => {
                make_html!(html_handler $($rest)*)
            };
        }
        let html_handler = html_handler.document_start();
        let html_handler = html_handler.doctype();
        let html_handler = html_handler.tag_open_start("html");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.next_child_tag_open_start("head");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.skip_comment("RMGklg_XASh8hhew3hZIhYXmZF9hdbOOrS4pTp7U4-Q");
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.next_child_tag_open_start("script");
        let html_handler = html_handler.attribute("type", "text/javascript");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.next_child_tag_open_start("title");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.skip_text("Technische Universität Darmstadt");
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.next_child_tag_open_start("meta");
        let html_handler = html_handler.attribute("http-equiv", "X-UA-Compatible");
        let html_handler = html_handler.attribute("content", "IE=EmulateIE9");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.skip_comment("y6RvLoAFlJ-yhWOzZ1eFLGpyCih6hv5vxd56zEkIHR4");
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.next_child_tag_open_start("meta");
        let html_handler = html_handler.attribute("http-equiv", "cache-control");
        let html_handler = html_handler.attribute("content", "no-cache");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
        let html_handler = html_handler.skip_whitespace();
        let html_handler = html_handler.next_child_tag_open_start("meta");
        let html_handler = html_handler.attribute("http-equiv", "expires");
        let html_handler = html_handler.attribute("content", "-1");
        let html_handler = html_handler.tag_open_end();
        let html_handler = html_handler.close_element();
        html!(<"meta" "http-equiv"="pragma" "content"="no-cache">);

        Ok(Self { client })
    }
}
