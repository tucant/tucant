use html_extractor::html;
use reqwest::Client;
use scraper::{html, Html};

use crate::{
    common::head::{html_head, html_head_2, logged_in_head, page_start, vv_something},
    html_handler::Root,
    login::{self, LoginResponse},
    TucanError,
};

pub async fn after_login(client: &Client, login_response: LoginResponse) -> Result<(), TucanError> {
    let id = login_response.id;
    /*let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{},-N000019,", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    println!("{response:#?}");
    let content = response.text().await?;*/
    let content = tokio::fs::read_to_string("input.html").await?;
    let document = Html::parse_document(&content);
    println!("{}", document.html());
    //tokio::fs::write("input.html", document.html()).await;
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
    let html_handler = logged_in_head(html_handler, login_response.id);
    html!(
    <!--"EkIRwtbzV1S0qAPx6If3Ye8Ey0JkAZsONsPW8C2Tf3Y"-->_
        <script type="text/javascript"></script>_
        <h1>"Herzlich willkommen, Moritz Hedtke!"</h1>_
        <h2>_</h2>_
        <h2>_text</h2>_
              <div class="tb rw-table">_
            <div class="tbhead">"Heutige Veranstaltungen:"</div>_
            <div class="tbcontrol">_
                          <a href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000019,-A09.10.2024,-A,-N1")} class="img" name="schedulerLink">"Stundenplan"</a>_
                                </div>_
                    <div class="tbsubhead">
                "\n        \tFür heute sind keine Termine angesetzt!\n\t\t"
            </div>_
                  </div>_

                  <!-- "jcECXQ7Iovu3-f4IpT-2ykwKMYpSGOecnocvEf5bo3A" -->_
                  <div class="tb rw-table">_
                    <div class="tbhead">"Eingegangene Nachrichten:"</div>_
                    <div class="tbcontrol">_
                                   <a href=_archive class="img">"Archiv"</a>_
                                 </div>_
                              <table class="nb rw-table rw-all" summary="Eingegangene Nachrichten">_
                          <tbody>
                                 <tr class="tbsubhead rw-hide">_
                          <th id="Datum">"Datum"</th>_
                          <th id="Uhrzeit">"Uhrzeit"</th>_
                          <th id="Absender">"Absender"</th>_
                          <th id="Betreff">"Betreff"</th>_
                          <th id="Aktion">"Aktion"</th>_
                        </tr>_
        );
    let mut html_handler = html_handler;
    while html_handler.peek().is_some() {
        html_handler = {
            html!(
              <tr class="tbdata">_
              <td headers="Datum" class="rw rw-maildate"><a class="link" href=_url>date</a></td>_
              <td headers="Uhrzeit" class="rw rw-mailtime"><a class="link" href=_url>hour</a></td>_
              <td headers="Absender" class="rw rw-mailpers"><a class="link" href=_url>source</a></td>_
              <td headers="Betreff" class="rw rw-mailsubject"><a class="link" href=_url>
            );
            let (html_handler, any_child) = html_handler.next_any_child();
            println!("{:?}", any_child.value());
            html!(
              </a></td>_
              <td headers="Aktion" class="rw rw-maildel"><a class="link" href=_url>"Löschen"</a></td>_
            </tr>_
            );
            html_handler
        };
    }
    html!(
      </tbody>
          </table>_
          </div>_
    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
     </div>_
    </div>_
    </div>_
    <div id="pageFoot" class="pageElementTop">_
    <div id="pageFootControls" class="pageElementTop">_
     <div id="pageFootControlsLeft">_
                             <a href={&format!("?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000019,-Aimprint")} class="img img_arrowImprint pageElementLeft" id="pageFootControl_imp">"Impressum"</a>_
                                                                             <a href={&format!("?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000019,-Acontact")}  class="img img_arrowContact pageElementLeft" id="pageFootControl_con">"Kontakt"</a>_
                                                 <a href="#" onclick="window.print();" class="img img_arrowPrint pageElementLeft" id="pageFootControl_pri">"Drucken"</a>_
                             </div>_
     <div id="pageFootControlsRight">_
       <a href="#top" class="img img_arrowUp pageElementRight" id="pageFootControl_up">_</a>_
     </div>_
    </div>_
    </div>_
    </div>_
    <div id="IEdiv">_</div><!-- "sA0YIGyByIKeA31YLo4xBo8n4XODq22IfHyrzzrnD-w"-->_
    <!-- "em2y7JxbjqWZd3r7SQA-YKIJZsneemykpZ46ZXTq7Tw"-->_
    <!--"VwiU8OlvNnMu2C0d8thjT7A2X3pYuFyyhLNGOJ87AXc"-->_
    <div class="invAnchor">_
    <a name="bottom" class="invAnchor">_</a>_
    </div>_
    </body>
    </html>
        );
    Ok(())
}
