use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{html_handler::Root, TucanError};

async fn startpage_dispatch_1(client: &Client) -> Result<(), TucanError> {
    let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001")
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
        <html>
        <head>_
            <!--"TpH4lBnEvBoB3gHo7u9UYwu2X7fAAlmIE2tkBMpvsak"-->_
            <!--"IcATzFs-AhJLlgCbtH_f4J_riUKWfS8yoLLT9ozdTlA"-->_
            <script type="text/javascript"></script>_
            <title>"Technische Universität Darmstadt"</title>_
            <meta http-equiv="X-UA-Compatible" content="IE=EmulateIE9"></meta>_
            <meta http-equiv="cache-control" content="no-cache"></meta>_
            <meta http-equiv="expires" content="-1"></meta>_
            <meta http-equiv="pragma" content="no-cache"></meta>_
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8"></meta>_
            <meta http-equiv="Content-Script-Type" content="text/javascript"></meta>_
            <meta name="viewport" content="width=device-width, initial-scale=1,user-scalable=0"></meta>_
            <link href="/css/_default/dl.startpage.css" rel="stylesheet" type="text/css"></link>_
            <link href="/css/styles.css" rel="stylesheet" type="text/css"></link>_
            <link href="/css/colors.css" rel="stylesheet" type="text/css"></link>_
        </head>_
        <body class="redirect">_
            <div id="wrapper">_
                <a href="http://http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                    <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                </a>_
                <!--"MA-hDUoCrkYqlM3RsS9EUjq0y_UcuN1AB82k4O5O8YU"-->_
                <h2><a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">"Sie werden zur Startseite weitergeleitet ..."</a></h2>_
                <a style="text-decoration: underline;" href="/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome">"Startseite"</a>_
            </div>_
            <div id="sessionId" style="display: none;">"000000000000001"</div>_
            <!--"zhJ3t6XNo2cfpZZEFiqxHJQ9udSXk5D418ej5lEytG8"-->_
            <script>
            "\n\t\twindow.setTimeout(function() {\n\t\t\twindow.location.href = '/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome';\n\t\t}, 500);\n\t\t"
            </script>_
        </body>
        </html>
    );
    Ok(())
}
