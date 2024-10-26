use html_extractor::html;
use scraper::Html;

use crate::{common::head::html_head_2, html_handler::Root, MyClient, TucanError};

pub async fn startpage_dispatch_1(client: &MyClient) -> Result<(), TucanError> {
    let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001")
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
    );
    let html_handler = html_head_2(html_handler);

    // TODO FIXME duplication, just grep some strings
    html!(
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
    let _html_handler = html_handler;
    Ok(())
}
