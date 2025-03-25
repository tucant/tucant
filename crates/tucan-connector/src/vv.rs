use scraper::{ElementRef, Html};
use tucant_types::{
    LoginResponse, TucanError,
    coursedetails::CourseDetailsRequest,
    vv::{Veranstaltung, Vorlesungsverzeichnis},
};

use crate::{
    COURSEDETAILS_REGEX, TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
};
use html_handler::{MyElementRef, MyNode, Root, parse_document};

#[expect(clippy::too_many_lines)]
pub async fn vv(connector: &TucanConnector, login_response: LoginResponse, action: String) -> Result<Vorlesungsverzeichnis, TucanError> {
    let content = authenticated_retryable_get(connector, &format!("https://www.tucan.tu-darmstadt.de{action}"), &login_response.cookie_cnsc).await?;
    /*login_response = LoginResponse {
        id: 299831749011778,
        cookie_cnsc: "".to_owned(),
    };
    let content = include_str!("../../../target/index.html");*/
    let document = parse_document(&content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "BjqYdi4ObBJkUieEorQw0YdKnpcMGF20vHHYGe3AyYE"
                    </style>
                </head>
                <body class="registration_auditor">
                    use logged_in_head(html_handler, login_response.id).0;
                    <script type="text/javascript">
                    </script>
                    <h1>
                        "Vorlesungsverzeichnis"
                    </h1>
                    <h2>
                        let path = while html_handler.peek().is_some() {
                            <a href=url>
                                let title = if html_handler.peek().is_some() {
                                    title
                                } => title;
                            </a>
                        } => (url, title);
                    </h2>
                    let description = if html_handler.peek().unwrap().value().is_element() && html_handler.peek().unwrap().value().as_element().unwrap().has_class("nb", scraper::CaseSensitivity::CaseSensitive) {
                        <div class="tb nb">
                            let description = while html_handler.peek().is_some() {
                                let any_child = html_handler.next_any_child();
                            } => match any_child.value() {
                                MyNode::Text(text) => text.trim().to_owned(),
                                MyNode::Element(_element) => MyElementRef::wrap(any_child).unwrap().html(),
                                _ => panic!(),
                            };
                        </div>
                    } => description;
                    let entries = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "ul" {
                        <ul class="auditRegistrationList" id="auditRegistration_list">
                            let entries = while html_handler.peek().is_some() {
                                <li title=_title>
                                    <a class="auditRegNodeLink" href=reg_href>
                                        _title
                                    </a>
                                </li>
                            } => reg_href;
                        </ul>
                    } => entries;
                    let veranstaltungen_or_module = if html_handler.peek().is_some() {
                        <div class="tb">
                            <div class="tbhead">
                                "Veranstaltungen / Module"
                            </div>
                            let veranstaltungen_or_module = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "table" {
                                <table class="nb eventTable">
                                    <tbody>
                                        <tr class="tbsubhead">
                                            <th>
                                            </th>
                                            <th>
                                                "Veranstaltung / Modul"
                                                <br></br>
                                                "Dozenten / Modulverantwortliche"
                                                <br></br>
                                                "Zeitraum"
                                            </th>
                                            <th>
                                            </th>
                                            <th colspan="2">
                                                "Veranstaltungsart"
                                                <br></br>
                                                "Raum"
                                            </th>
                                        </tr>
                                        let veranstaltungen = while html_handler.peek().is_some() {
                                            <tr class="tbdata">
                                                <td>
                                                </td>
                                                <td>
                                                    <a name="eventLink" href=coursedetails_url class="eventTitle">
                                                        title
                                                    </a>
                                                    <br></br>
                                                    lecturer_name
                                                    let date_range = if html_handler.peek().is_some() {
                                                        <br></br>
                                                        date_range
                                                    } => date_range;
                                                </td>
                                                <td>
                                                </td>
                                                <td colspan="2">
                                                    course_type
                                                </td>
                                            </tr>
                                        } => Veranstaltung {
                                            title,
                                            coursedetails_url: CourseDetailsRequest::parse(&COURSEDETAILS_REGEX.replace(&coursedetails_url, "")),
                                            lecturer_name,
                                            date_range,
                                            course_type
                                        };
                                    </tbody>
                                </table>
                            } => veranstaltungen else {
                                <div class="tbdata" colspan="3">
                                    "Es wurden keine Veranstaltungen gefunden."
                                </div>
                            } => Vec::<Veranstaltung>::new();
                        </div>
                    } => veranstaltungen_or_module.either_into();
                </div>
            </div>
        </div>
    }
    let html_handler = footer(html_handler, login_response.id, 326);
    html_handler.end_document();
    Ok(Vorlesungsverzeichnis {
        entries: entries.unwrap_or_default(),
        path,
        description: description.unwrap_or_default(),
        veranstaltungen_or_module: veranstaltungen_or_module.unwrap_or_default(),
    })
}
