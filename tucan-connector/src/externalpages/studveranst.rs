use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, logged_in_head, page_start, vv_something},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

pub async fn veranstaltungen(
    client: &Client,
    login_response: LoginResponse,
) -> Result<(), TucanError> {
    let id = login_response.id;
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000273,-Astudveranst%2Ehtml", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    println!("{response:#?}");
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    println!("{}", document.html());
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html!(
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
        <head>_
    );
    let html_handler = html_head(html_handler);
    html!(
        <style type="text/css">
            "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
        </style>_
        </head>_
        <body class="external_pages">_
    );
    let html_handler = logged_in_head(html_handler, login_response.id);
    html!(
    <!--"Ur30ahmaXh5XzV5xIHsTj20h-0qX1_GS1SR0QttvqB0"-->_
        <script type="text/javascript"></script>_
        <!-- "n-Z4H5FHWoHAA64FTQESwvBoOya6RXPFuEU50U1jCJQ"-->_

        );
    Ok(())
}
