use html_extractor::html;
use scraper::Html;

use crate::{html_handler::Root, MyClient, TucanError};

pub async fn root(client: &MyClient) -> Result<(), TucanError> {
    let response = client
        .get("https://www.tucan.tu-darmstadt.de/")
        .send()
        .await?
        .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html!(
        <html>
        <head>_
            <!--"RMGklg_XASh8hhew3hZIhYXmZF9hdbOOrS4pTp7U4-Q"-->_
            <script type="text/javascript"></script>_
            <title>"Technische Universität Darmstadt"</title>_
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
            <div id="wrapper">_
                <a href="http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                    <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                </a>_
                <ul id="langMenu">_
                    <!--"OKMmJxVa9MEpv1nT-faADAcITZqNMeN44hxFZOI5duQ"-->_
                    <li><a class="img img_LangGerman" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001">"de"</a></li>_
                    <!--"B_W58bA9r6Y9MF-DHPyZIP45rNK-6Ba3bAydB3VM8DM"-->_
                    <li><a class="img img_LangEnglish" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000002">"en"</a></li>_
                </ul>_
            </div>_
        </body>
        </html>
    );
    let _html_handler = html_handler;
    Ok(())
}
