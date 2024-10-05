pub mod html_handler;

use data_encoding::HEXLOWER;
use html_extractor::html;
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
        html!(
            <head>_
            <!--"RMGklg_XASh8hhew3hZIhYXmZF9hdbOOrS4pTp7U4-Q"-->_
            <script type="text/javascript"></script>_
            <title>
        );
        let html_handler = html_handler.skip_text("Technische Universität Darmstadt");
        html!(
            </title>_
            <meta http-equiv="X-UA-Compatible" content="IE=EmulateIE9"></meta>_
            <!--"y6RvLoAFlJ-yhWOzZ1eFLGpyCih6hv5vxd56zEkIHR4"-->_
            <meta http-equiv="cache-control" content="no-cache"></meta>_
            <meta http-equiv="expires" content="-1"></meta>_
            <meta http-equiv="pragma" content="no-cache"></meta>_
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8"></meta>_
            <meta http-equiv="Content-Script-Type" content="text/javascript"></meta>_
            <meta name="viewport" content="width=device-width, initial-scale=1,user-scalable=0"></meta>_
            <link rel="shortcut icon" type="image/x-icon" href="/gfx/tuda/icons/favicon.ico"></link>_
            <link rel="apple-touch-icon" href="/gfx/tuda/icons/iphone_touch_icon.png" type="image/gif"></link>_
            <meta http-equiv="refresh" content="0; URL=/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001"></meta>_
            <link href="/css/_default/dl.startpage.css" rel="stylesheet" type="text/css"></link>_
            <script type="text/javascript" src="/js/mobile_master/jquery.js"></script>_
            <script type="text/javascript" src="/js/mobile_master/onmediaquery.min.js"></script>_
            </head>_
            <body>_
        );

        Ok(Self { client })
    }
}
