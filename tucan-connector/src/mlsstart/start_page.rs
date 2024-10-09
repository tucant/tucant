use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, html_head_2},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

pub async fn after_login(client: &Client, login_response: LoginResponse) -> Result<(), TucanError> {
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{},-N000019,", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
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
        <div id="Cn-system-desc">_</div>_
            <script type="text/javascript">
                "JSQjAjNPl1OG1yTeHLoTj6JEhV74LO2CassBGq9DPqo"
            </script>_
            <div id="acc_pageDescription" class="hidden"><a name="keypadDescription" class="hidden">"keypadDescription"</a>
                "TvMjLPj4FsS4YUVJn3nppMhuQYkGn5LXsWX2f54ngjY"
                <a href="#mainNavi" accesskey="1">"1 Hauptmenü"</a>_
                <a href="#mainContent" accesskey="2">"2 Inhalt"</a>_
                <a href="#keypadDescription" accesskey="3">"3 Zurück zu dieser Anleitung"</a>_
            </div>_
            <div id="pageContainer" class="pageElementTop">_
                <div class="invAnchor">_
                    <a name="top" class="invAnchor"></a>_
                </div>_
                <div id="pageHead" class="pageElementTop">_
                    <div id="pageHeadTop" class="pageElementTop">_
                        <a href=imprint_url class="img img_arrowImprint pageElementLeft">"Impressum"</a>_
                        <a href=contact_url class="img img_arrowContact pageElementLeft">"Kontakt"</a>_
                        <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft">"Drucken"</a>_
                        <a href="#bottom" class="img img_arrowDown pageElementRight">"Zum Ende der Seite"</a>_
                    </div>_
                    <div id="pageHeadCenter" class="pageElementTop">_
                    <div id="pageHeadLeft" class="pageElementLeft">_
                        <a href="http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                            <img id="imagePageHeadLeft" src="/gfx/tuda/logo.gif" alt="Logo Technische Universität Darmstadt"></img>_
                        </a>_
                    </div>_
                    <div id="pageHeadRight" class="pageElementRight">
                        _
                    </div>_
                </div>_
                <div id="pageHeadBottom_1" class="pageElementTop">_
                    <div id="pageHeadControlsLeft" class="pageElementLeft">_
                        <a class="img pageHeadLink" href="#" id="extraNav_link1" target="_blank">"Homepage"</a>_
                        <a class="img pageHeadLink" href="#" id="extraNav_link2" target="_blank">"standardLink undef"</a>_
                    </div>_
                    <div id="pageHeadControlsRight" class="pageElementRight">_
                        <a class="img" href="#" id="extraNav_link3" target="_blank">"standardLink undef"</a>_
                        <a class="img" href="#" id="extraNav_link4" target="_blank">"standardLink undef"</a>_
                        <a class="img" href="#" id="extraNav_link5" target="_blank">_</a>_
                    </div>_
                </div>_
                <div id="pageHeadBottom_2" class="pageElementTop">_
                    <div id="pageHeadBottom_2sub_1" class="pageElementTop">_</div>_
                    <div id="pageHeadBottom_2sub_2" class="pageElementTop">_</div>_
                </div>_

    );
    Ok(())
}
