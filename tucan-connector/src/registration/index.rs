use html_extractor::html;
use reqwest::Client;
use scraper::Html;

use crate::{
    common::head::{html_head, logged_in_head, page_start, vv_something},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

pub async fn anmeldung(
    client: &Client,
    login_response: &LoginResponse,
    args: &str,
) -> Result<(), TucanError> {
    let id = login_response.id;
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{:015}{args}", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
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
    let mut html_handler = html_head(html_handler);
    if html_handler.peek().is_none() {
        // timeout?
        html!(
            </head>_
        <body class="timeout">
        );
        return Err(TucanError::Timeout);
    }
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

    <!-- "UU9Ju2ASETVrRfIpA3xWkFcE5n3oN4PCI9QksTmApIA" -->_
            <form id="registration" action="/scripts/mgrqispi.dll" method="post">_
                    <table class="tbcoursestatus rw-table rw-all">_
                    <tbody>
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
                                        <input name="sessionno" type="hidden" value={&id.to_string()}></input>_
                                        <input name="menuno" type="hidden" value="000311"></input>_
                                        <input name="pa rent1" type="hidden" value="000000000000000"></input>_
                                        <input name="parent2" type="hidden" value="000000000000000"></input>_
                                        <input name="changestudy" type="hidden" value="1"></input>_
                                    </td>_
                            </tr>_
                        </tbody>
                    </table>_
            </form>_
    <!-- "mrUJOOH3fqYzcWGWygCuNQGMPfDRh8akKXEihfucyR0" -->_
    <h2>_
            <a href={&format!("/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{id:015},-N000311,-N391343674191079,-N0,-N0,-N0")}>"M.Sc. Informatik (2023)"</a>
    );
    let mut html_handler = html_handler;
    while html_handler.peek().is_some() {
        html_handler = {
            html!(
                "\n        \u{a0}>\u{a0}\n                "
                <a href=url>
            );
            let (html_handler, any_child) = html_handler.next_any_child();
            html!(
                </a>_
            );
            println!("{:?} {url}", any_child.value());
            html_handler
        };
    }
    html!(
        </h2>_
        <ul>_
    );
    let mut html_handler = html_handler;
    while html_handler.peek().is_some() {
        html_handler = {
            html!(
                <li>_
                        <a href=url>item</a>_
                    </li>_
            );
            println!("{item} {url}");
            html_handler
        };
    }

    html!(
                        </ul>_
    <!-- "gACLM-J4jmb4gKmvgI-c8EqENeLydqGZuryaUY-7Lm4" -->_
    <!-- "PQQwWAU_NypeYX1Jw191sjka_fWLRqDlYVWZm-gWSFs" -->_
    <br></br>_
    <!-- "9XmEOh66hIETO2XPWUf_msfayuKwcwW3Q-0NvQQ6mvA" -->_
    );
    Ok(())
}
