pub mod streaming_http_parser;

use futures_util::TryStreamExt as _;
use reqwest::{Client, ClientBuilder, Response};
use streaming_http_parser::StreamingHttpParser;

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
        let content = resp.bytes_stream();
        let mut parser = StreamingHttpParser {
            async_read: tokio_util::io::StreamReader::new(
                content.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
            ),
        };
        parser.doctype().await?;
        Ok(Tucan { client })
    }
}
