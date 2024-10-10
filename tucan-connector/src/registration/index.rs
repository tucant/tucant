use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, logged_in_head, page_start, vv_something},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

pub async fn anmeldung(client: &Client, login_response: LoginResponse) -> Result<(), TucanError> {
    let id = login_response.id;
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{:015},-N000311,-A", login_response.id))
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
        <style type="text/css">
            "3CC0xpJgjHprYY59D1krvfwrI2LSV2-OtaN3CviYnG8"
        </style>_
        </head>_
        <body class="registration">_
    );
    let html_handler = logged_in_head(html_handler, login_response.id);
    html!(
        <!--"up71ljpj_w5JCBcjI0pvus0gS__0taKvkYJ-_QU1yNk"-->_
            <script type="text/javascript"></script>_

    <h1>"Anmeldung zu Modulen und Veranstaltungen"</h1>_

    <!-- "SWITCH STUDIES" -->_
            <form id="registration" action="/scripts/mgrqispi.dll" method="post">_
                    <table class="tbcoursestatus rw-table rw-all">_
                            <tr>_
                                    <td class="tbhead" colspan="100%">"Weitere Studien"</td>_
                            </tr>_
                            <tr>_
                                <td class="tbcontrol" colspan="100%">_

                                           <div class="inputFieldLabel">_
                                                    <label for="study">"Studium:"</label>_
                                                    <select name="study" id="study" onchange="reloadpage.submitForm(this.form.id);" class="pageElementLeft">_
                                                                                                                            <option value="376333755785484" >"B.Sc. Informatik (2015)"</option>_
                                                                                                                            <option value="391343674191079" selected="selected">"M.Sc. Informatik (2023)"</option>_
                                                                                                            </select>_
                                                    <input name="Aktualisieren" type="submit" value="Aktualisieren" class="img img_arrowReload pageElementLeft"></input>_
                                            </div>_
                                        <input name="APPNAME" type="hidden" value="CampusNet"></input>_
                                        <input name="PRGNAME" type="hidden" value="REGISTRATION"></input>_
                                        <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,study,changestudy,parent1,parent2"></input>_
                                        <input name="sessionno" type="hidden" value="531875782768695"></input>_
                                        <input name="menuno" type="hidden" value="000311"></input>_
                                        <input name="pa rent1" type="hidden" value="000000000000000"></input>_
                                        <input name="parent2" type="hidden" value="000000000000000"></input>_
                                        <input name="changestudy" type="hidden" value="1"></input>_
                                    </td>_
                            </tr>_
                    </table>_
            </form>_
    <!-- "END STUDIES" -->_
    <h2>
            <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N0,-N0">"M.Sc. Informatik (2023)"</a>
            " > "
                    <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N<!$MG_SESSIONIDNAVI>,-N<!$MG_MENUIDNAVI>,-N<!$MG_STUDYNAVI>,-N0,-N<!$MG_STSRNAVI>,-N<!$MG_STCSNAVI>">"<!$MG_DESCNAVI>"</a>
            </h2>


            <ul>
                                <li>
                        <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N383934077885362,-N000000000000000"> "Vertiefungen, Wahlbereiche und Studium Generale"</a>
                    </li>
                                <li>
                        <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N383963665116343,-N000000000000000"> "Masterarbeit"</a>
                    </li>
                                <li>
                        <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N384195230855950,-N000000000000000"> "Auflagen"</a>
                    </li>
                                <li>
                        <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N383963761982346,-N000000000000000"> "Zusätzliche Leistungen"</a>
                    </li>
                                <li>
                        <a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=REGISTRATION&amp;ARGUMENTS=-N531875782768695,-N000311,-N391343674191079,-N0,-N999999999999999,-N000000000000000"> "Weitere Veranstaltungen"</a>
                    </li>
                        </ul>


    <!-- "SHOW STUDY RULE DESCRIPTION" -->
    <!-- "END STUDY RULE DESCRIPTION" -->
    <br></br>

    <!-- "END SELECTED_MODULE" -->

    );
    Ok(())
}
