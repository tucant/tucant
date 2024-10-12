use html_extractor::html;
use reqwest::Client;
use scraper::{ElementRef, Html};

use crate::{
    common::head::{footer, html_head, logged_in_head, page_start, vv_something},
    html_handler::Root,
    login::LoginResponse,
    TucanError,
};

#[derive(Debug, Clone)]
pub struct AnmeldungRequest {
    arguments: String,
}

impl AnmeldungRequest {
    pub fn new() -> Self {
        Self {
            arguments: ",-N000311,-A".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnmeldungResponse {
    pub path: Vec<(String, String)>,
    pub entries: Vec<(String, AnmeldungRequest)>,
}

pub async fn anmeldung(
    client: &Client,
    login_response: &LoginResponse,
    args: AnmeldungRequest,
) -> Result<AnmeldungResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{:015}{}", login_response.id, args.arguments);
    println!("{url}");
    let response = client
        .get(url)
        .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
        .send()
        .await?
        .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    //println!("{}", document.html());
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
    let mut path = Vec::new();
    let mut html_handler = html_handler;
    while !html_handler
        .peek()
        .unwrap()
        .value()
        .as_text()
        .unwrap()
        .trim()
        .is_empty()
    {
        html_handler = {
            html!(
                "\n        \u{a0}>\u{a0}\n                "
                <a href=url>
            );
            let (html_handler, any_child) = html_handler.next_any_child();
            match any_child.value() {
                scraper::Node::Comment(comment) => {}
                scraper::Node::Text(text) => path.push((text.to_string(), url)),
                _ => panic!(),
            }
            html!(
                </a>
            );
            html_handler
        };
    }
    html!(
        _</h2>_
    );
    let mut entries = Vec::new();
    let html_handler = match html_handler.peek() {
        Some(elem) if elem.value().is_element() => {
            html!(        <ul>_
            );
            let mut html_handler = html_handler;
            while html_handler.peek().is_some() {
                html_handler = {
                    html!(
                        <li>_
                                <a href=url>item</a>_
                            </li>_
                    );
                    let url = url.trim_start_matches(&format!(
                "/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{id:015}"
            ));
                    entries.push((
                        item,
                        AnmeldungRequest {
                            arguments: url.to_owned(),
                        },
                    ));
                    html_handler
                };
            }

            html!(
                        </ul>_);
            html_handler
        }
        _ => html_handler,
    };
    html!(
    <!-- "gACLM-J4jmb4gKmvgI-c8EqENeLydqGZuryaUY-7Lm4" -->_
    );
    while !html_handler.peek().unwrap().value().is_comment() {
        let child;
        (html_handler, child) = html_handler.next_any_child();
        match child.value() {
            scraper::Node::Text(text) => assert!(text.trim().is_empty()),
            scraper::Node::Element(element) => {
                println!(
                    "information node: {}",
                    ElementRef::wrap(child).unwrap().html()
                )
            }
            _ => panic!(),
        }
    }
    html!(
    <!-- "PQQwWAU_NypeYX1Jw191sjka_fWLRqDlYVWZm-gWSFs" -->_
    <br></br>_
    <!-- "9XmEOh66hIETO2XPWUf_msfayuKwcwW3Q-0NvQQ6mvA" -->_
    );
    let html_handler = if entries.is_empty() {
        html!(
            <table class="tbcoursestatus rw-table rw-all">_
            <tbody>
            <tr>_
                <td class="tbhead" colspan="100%">"Anmeldung zu Modulen und Veranstaltungen"</td>_
            </tr>_

            <tr>_
        );
        let html_handler = if (html_handler
            .peek()
            .unwrap()
            .value()
            .as_element()
            .unwrap()
            .attr("class")
            .unwrap()
            == "tbdata")
        {
            html!(
                <td class="tbdata" colspan="4">"Keine Module oder Veranstaltungen zur Anmeldung gefunden"</td>_
                    </tr>_
                    </tbody>
                    </table>_
            );
            html_handler
        } else {
            html!(
                <td class="tbsubhead">_
                    <!-- "Logo column" -->_
                </td>_
                <td class="tbsubhead">_
                    "Veranstaltung"<br></br>_
                    "Dozenten"
                    <br></br>"Zeitraum"
                    <br></br>"Anmeldegruppe"
                    <br></br>"Standort"
                </td>_
                <td class="tbsubhead">
                    "Anmeld. bis"
                    <br></br>_
                    "Max.Teiln.|Anm."
                </td>_
                <td class="tbsubhead">_</td>_
            </tr>_

            <tr>_
                <!--"logo column"-->_
                <td class="tbsubhead"> <!-- "FIXME TDs ... Module Level ??" -->_
                </td>_
                <!-- "MODULE" -->_
                <td class="tbsubhead dl-inner" >_
                    <p><strong><a href="/scripts/mgrqispi.dll?APPNAME=CampusNet&amp;PRGNAME=MODULEDETAILS&amp;ARGUMENTS=-N559411740841178,-N000311,-N383203116623058,-APfHZP-nZWfLkYSP5PQR5vDwTWZpKQgpTHQHZYQoEvMp7vUoxWIU9OQRWQD6eRDmURuHHmdRMejWS4DGl3om8cDUqOMRoc-LlOWl9YSPUxUV-PULaVDGp7ucj4fljvZKwmZoZPQHAPNlAVvZtmdUtvqFwRfG3Rqm6fZW7vNLNxBmHRdLA3ocAvjLECWBZmzUS7YHgWSUeOQiNHUR9xWUDOfwfOupPHqB-7dZtmqH-xDw3CupTxMmy7fWXVdAYPWmNHzLdQjaw4YLARQVjeqob7DwkeMHW7MASfzwZxUpgmBUSHgPCVzHwOZe-OQ5emMAk3oRpxupU4B5C4WDA3QLwxYW-3BoWvIHacoPVVjUfcZUFHIWmxMUuvBD6f-PgWNGhHUetYgmYmDWDxQUvYBweWUWYYWo-PqR6PzLY3oHFHDoNOMeZvuUsWYPqmNPYvBUqO-RuYYPJfjp8CYo-PfLHfDGpvQPBxDLuHWpVRfDNWYWxHQLXHZLvRSmK7fWBVWpBcMR9RYmPWgHEVqPqrqm7fkin">"20-00-0014 "<span class="eventTitle">"Visual Computing (WiSe 2022/23)"</span></a></strong></p>_
                    <p>"N.N."</p>_
                </td>_
                <td class="tbsubhead">_
                    "28.02.2023"<br></br>_
                </td>_
                <td class="tbsubhead rw-qbf">_
                </td>_
                <!-- "MODULE END"-->_
                <!--"MODULE PART" -->_
                <!--"MODULE PART END"-->_
                <!--"COURSE" --> <!-- "FIXME TDs ... Course Level ??" -->_
                <!--"COURSE END" -->_
            </tr>_
            </tbody>
            </table>_
                    );
            html_handler
        };

        html_handler
    } else {
        html_handler
    };
    html!(
        <!-- "fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og" -->_

                </div>_
            </div>_
        </div>_
    );
    let html_handler = footer(html_handler, id, 311);
    // TODO FIXME parse rest of page
    Ok(AnmeldungResponse { path, entries })
}
