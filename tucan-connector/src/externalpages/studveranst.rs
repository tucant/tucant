use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, page_start, vv_something},
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
            "XN0jaYaHLeXpiJk0Z7v_FOEBkC5jmdtJaIxqyRqEMj4"
        </style>_
        </head>_
        <body class="currentevents">_
    );
    let html_handler = page_start(html_handler);
    html!(
                        <li class="tree depth_1 linkItem branchLinkItem " title="Aktuelles" id="link000019">
                            <a  class="depth_1 link000019 navLink branchLink " href=aktuelles_url >"Aktuelles"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Nachrichten" id="link000299"><a  class="depth_2 link000299 navLink " href=messages_url >"Nachrichten"</a></li>
                            </ul>
                          </li>
                          <li class="tree depth_1 linkItem branchLinkItem " title="VV" id="link000326">
                            <a  class="depth_1 link000326 navLink branchLink " href=vv_url >"VV"</a>
    );
    let html_handler = vv_something(html_handler, login_response.id);
    html!(
                              </li>

    );
    Ok(())
}
