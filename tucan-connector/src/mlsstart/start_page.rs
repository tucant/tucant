use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, html_head_2, page_start, vv_something},
    html_handler::Root,
    login::LoginResponse,
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
                          <li class="tree depth_1 linkItem branchLinkItem " title="Stundenplan" id="link000268">
                            <a  class="depth_1 link000268 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000268,-A,-A,-N1")} >"Stundenplan"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Tagesansicht" id="link000269"><a  class="depth_2 link000269 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000269,-A,-A,-N0")}>"Tagesansicht"</a></li>
                              <li class="intern depth_2 linkItem " title="Wochenansicht" id="link000270"><a  class="depth_2 link000270 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER&ARGUMENTS=-N{id:015},-N000270,-A,-A,-N1")} >"Wochenansicht"</a></li>
                              <li class="intern depth_2 linkItem " title="Monatsansicht" id="link000271"><a  class="depth_2 link000271 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MONTH&ARGUMENTS=-N{id:015},-N000271,-A")} >"Monatsansicht"</a></li>
                              <li class="intern depth_2 linkItem " title="Export" id="link000272"><a  class="depth_2 link000272 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCHEDULER_EXPORT&ARGUMENTS=-N{id:015},-N000272,")} >"Export"</a></li>
                            </ul>
                          </li>
                          <li class="tree depth_1 linkItem branchLinkItem " title="Veranstaltungen" id="link000273">
                            <a  class="depth_1 link000273 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000273,-Astudveranst%2Ehtml")} >"Veranstaltungen"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Meine Module" id="link000275"><a  class="depth_2 link000275 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N{id:015},-N000275,")} >"Meine Module"</a></li>
                              <li class="intern depth_2 linkItem " title="Meine Veranstaltungen" id="link000274"><a  class="depth_2 link000274 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N{id:015},-N000274,")} >"Meine Veranstaltungen"</a></li>
                              <li class="intern depth_2 linkItem " title="Meine Wahlbereiche" id="link000307"><a  class="depth_2 link000307 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENTCHOICECOURSES&ARGUMENTS=-N{id:015},-N000307,")} >"Meine Wahlbereiche"</a></li>
                              <li class="intern depth_2 linkItem " title="Anmeldung" id="link000311"><a  class="depth_2 link000311 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{id:015},-N000311,-A")} >"Anmeldung"</a></li>
                              <li class="intern depth_2 linkItem " title="Mein aktueller Anmeldestatus" id="link000308"><a  class="depth_2 link000308 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYREGISTRATIONS&ARGUMENTS=-N{id:015},-N000308,-N000000000000000")} >"Mein aktueller Anmeldestatus"</a></li>
                            </ul>
                          </li>
                          <li class="tree depth_1 linkItem branchLinkItem " title="Prüfungen" id="link000280">
                            <a  class="depth_1 link000280 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000280,-Astudpruefungen%2Ehtml")} >"Prüfungen"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Meine Prüfungen" id="link000318"><a  class="depth_2 link000318 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N{id:015},-N000318,")} >"Meine Prüfungen"</a></li>
                              <li class="tree depth_2 linkItem branchLinkItem " title="Mein Prüfungsplan" id="link000389">
                                <a  class="depth_2 link000389 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SCPCHOICE&ARGUMENTS=-N{id:015},-N000389,")} >"Mein Prüfungsplan"</a>
                                <ul class="nav depth_3 linkItemContainer">
                                  <li class="intern depth_3 linkItem " title="Wichtige Hinweise" id="link000391"><a  class="depth_3 link000391 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000391,-Astudplan%2Ehtml")} >"Wichtige Hinweise"</a></li>
                                </ul>
                              </li>
                              <li class="tree depth_2 linkItem branchLinkItem " title="Semesterergebnisse" id="link000323">
                                <a  class="depth_2 link000323 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000323,-Astudergebnis%2Ehtml")} >"Semesterergebnisse"</a>
                                <ul class="nav depth_3 linkItemContainer">
                                  <li class="intern depth_3 linkItem " title="Modulergebnisse" id="link000324"><a  class="depth_3 link000324 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N{id:015},-N000324,")} >"Modulergebnisse"</a></li>
                                  <li class="intern depth_3 linkItem " title="Prüfungsergebnisse" id="link000325"><a  class="depth_3 link000325 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N{id:015},-N000325,")} >"Prüfungsergebnisse"</a></li>
                                </ul>
                              </li>
                              <li class="intern depth_2 linkItem " title="Leistungsspiegel" id="link000316"><a  class="depth_2 link000316 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N{id:015},-N000316,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000")} >"Leistungsspiegel"</a></li>
                            </ul>
                          </li>
                          <li class="tree depth_1 linkItem branchLinkItem " title="Service" id="link000337">
                            <a  class="depth_1 link000337 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000337,-Aservice%2Ehtml")} >"Service"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Persönliche Daten" id="link000339"><a  class="depth_2 link000339 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=PERSADDRESS&ARGUMENTS=-N{id:015},-N000339,-A")} >"Persönliche Daten"</a></li>
                              <li class="intern depth_2 linkItem " title="Meine Dokumente" id="link000557"><a  class="depth_2 link000557 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{id:015},-N000557,")} >"Meine Dokumente"</a></li>
                              <li class="intern depth_2 linkItem " title="Anträge" id="link000600"><a  class="depth_2 link000600 navLink " href=_url>"Anträge"</a></li>
                              <li class="intern depth_2 linkItem " title="Sperren" id="link000652"><a  class="depth_2 link000652 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=HOLDINFO&ARGUMENTS=-N{id:015},-N000652,")} >"Sperren"</a></li>
                            </ul>
                          </li>
                          <li class="tree depth_1 linkItem branchLinkItem " title="Bewerbung" id="link000441">
                            <a  class="depth_1 link000441 navLink branchLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000441,-Abewerbung")} >"Bewerbung"</a>
                            <ul class="nav depth_2 linkItemContainer">
                              <li class="intern depth_2 linkItem " title="Herzlich Willkommen" id="link000442"><a  class="depth_2 link000442 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000442,-Abewerbung")} >"Herzlich Willkommen"</a></li>
                              <li class="intern depth_2 linkItem " title="Meine Bewerbung" id="link000443"><a  class="depth_2 link000443 navLink " href=_url >"Meine Bewerbung"</a></li>
                              <li class="intern depth_2 linkItem " title="Meine Dokumente" id="link000444"><a  class="depth_2 link000444 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N{id:015},-N000444,")} >"Meine Dokumente"</a></li>
                            </ul>
                          </li>
                          <li class="intern depth_1 linkItem " title="Hilfe" id="link000340"><a  class="depth_1 link000340 navLink " href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{id:015},-N000340,-Ahilfe%2Ehtml")} >"Hilfe"</a></li>
                          </ul>_
                  </div>_
                  <div id="pageHeadBottom_3" class="pageElementTop">_
                        <div id="pageHeadSwitchLang" class="pageElementRight">_
                            <a href=_wef class="img img_LangEnglish pageElementLeft" title="English">"English"</a>_
                            <a href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOGOUT&ARGUMENTS=-N{id:015},-N001")} id="logoutButton"    class="img img_arrowLogout logout" title="Abmelden">"Abmelden"</a>_
                        </div>_
                    </div>_
              </div>_
    );
    Ok(())
}
