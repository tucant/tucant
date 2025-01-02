use scraper::Html;
use tucant_types::LoginResponse;

use crate::{
    common::head::{footer, html_head, logged_in_head},
    html_handler::Root,
    MyClient, TucanError,
};

pub async fn after_login(
    client: &MyClient,
    login_response: LoginResponse,
) -> Result<(), TucanError> {
    let id = login_response.id;
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{},-N000019,", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    let content = response.text().await?;
    //let content = tokio::fs::read_to_string("input.html").await?;
    let document = Html::parse_document(&content);
    //tokio::fs::write("input.html", document.html()).await;
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
            <head>_
    };
    let html_handler = html_head(html_handler);
    html_extractor::html! {
            <style type="text/css">
                "XN0jaYaHLeXpiJk0Z7v_FOEBkC5jmdtJaIxqyRqEMj4"
            </style>_
        </head>_
        <body class="currentevents">_
    };
    let html_handler = logged_in_head(html_handler, login_response.id);
    html_extractor::html! {
        <!--"EkIRwtbzV1S0qAPx6If3Ye8Ey0JkAZsONsPW8C2Tf3Y"-->_
        <script type="text/javascript">
        </script>_
        <h1>
            _welcome_message
        </h1>_
        <h2>_
        </h2>_
        <h2>
            _text
        </h2>_
        <div class="tb rw-table">_
            <div class="tbhead">
                "Heutige Veranstaltungen:"
            </div>_
            <div class="tbcontrol">_
                <a href=_ class="img" name="schedulerLink">
                    "Stundenplan"
                </a>_
            </div>_
            <div class="tbsubhead">
                "\n        \tFür heute sind keine Termine angesetzt!\n\t\t"
            </div>_
        </div>_
        <!--"jcECXQ7Iovu3-f4IpT-2ykwKMYpSGOecnocvEf5bo3A"-->_
        <div class="tb rw-table">_
            <div class="tbhead">
                "Eingegangene Nachrichten:"
            </div>_
            <div class="tbcontrol">_
                <a href=_archive class="img">
                    "Archiv"
                </a>_
            </div>_
            <table class="nb rw-table rw-all" summary="Eingegangene Nachrichten">_
                <tbody>
                    <tr class="tbsubhead rw-hide">_
                        <th id="Datum">
                            "Datum"
                        </th>_
                        <th id="Uhrzeit">
                            "Uhrzeit"
                        </th>_
                        <th id="Absender">
                            "Absender"
                        </th>_
                        <th id="Betreff">
                            "Betreff"
                        </th>_
                        <th id="Aktion">
                            "Aktion"
                        </th>_
                    </tr>_
    };
    while html_handler.peek().is_some() {
        html_handler = {
            html_extractor::html! {
                <tr class="tbdata">_
                    <td headers="Datum" class="rw rw-maildate">
                        <a class="link" href=_url>
                            _date
                        </a>
                    </td>_
                    <td headers="Uhrzeit" class="rw rw-mailtime">
                        <a class="link" href=_url>
                            _hour
                        </a>
                    </td>_
                    <td headers="Absender" class="rw rw-mailpers">
                        <a class="link" href=_url>
                            _source
                        </a>
                    </td>_
                    <td headers="Betreff" class="rw rw-mailsubject">
                        <a class="link" href=_url>
            };
            let (html_handler, any_child) = html_handler.next_any_child();
            html_extractor::html! {
                        </a>
                    </td>_
                    <td headers="Aktion" class="rw rw-maildel">
                        <a class="link" href=_url>
                            "Löschen"
                        </a>
                    </td>_
                </tr>_
            };
            html_handler
        };
    }
    html_extractor::html! {
                            </tbody>
                        </table>_
                    </div>_
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    };
    let html_handler = footer(html_handler, id, 19);
    html_handler.end_document();
    Ok(())
}
