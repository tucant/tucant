use scraper::Html;
use tucant_types::{LoginResponse, TucanError, Vorlesungsverzeichnis};

use crate::{
    MyClient, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
};
use html_handler::Root;

#[expect(clippy::too_many_lines)]
pub async fn vv(client: &MyClient, login_response: LoginResponse, action: String) -> Result<Vorlesungsverzeichnis, TucanError> {
    let content = authenticated_retryable_get(client, &format!("https://www.tucan.tu-darmstadt.de{action}"), &login_response.cookie_cnsc).await?;
    /*login_response = LoginResponse {
        id: 299831749011778,
        cookie_cnsc: "".to_owned(),
    };
    let content = include_str!("../../../target/index.html");*/
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>_
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "jEU_iZdc3G7CJJrJKJjZNWhmTKwpIEJrFoclfvaBhFQ"
                    </style>_
                </head>_
                <body class="registration_auditor">_
                    use logged_in_head(html_handler, login_response.id).0;
                    <!--"mAgJrK5QnezV6UMxREqfEJS8I4jUgb9auCtX-UqjbRI"-->_
                    <script type="text/javascript">
                    </script>_
                    <h1>
                        "Vorlesungsverzeichnis"
                    </h1>_
                    <!--"kVJ9mNrY2XJb35ukyO3hMoLc_9dEHSgzMALBDLwWpHM"-->_
                    <!--"Z6v-LbjcnKpltlabF99VIGyltOdElMLHxTYIzpsZgUU"-->_
                    <h2>_
                        let vorlesungsverzeichnisse = while html_handler.peek().is_some() {
                            <a href=url>
                                let title = if html_handler.peek().is_some() {
                                    title
                                } => title;
                            </a>_
                        } => (url, title);
                    </h2>_
                    <!--"fVvNiSxy43a6FBZQ0m9H05M74W8TF3aAE1n-6VH7y7g"-->_
                    let children = if html_handler.peek().unwrap().value().is_element() && html_handler.peek().unwrap().value().as_element().unwrap().name() == "div" {
                        <div class="tb nb">
                            let children = while html_handler.peek().is_some() {
                                let any_child = html_handler.next_any_child();
                            } => any_child;
                        </div>_
                    } => children;
                    let entries = if html_handler.peek().unwrap().value().is_element() {
                        <ul class="auditRegistrationList" id="auditRegistration_list">_
                            let entries = while html_handler.peek().is_some() {
                                <li title=_title>
                                    <a class="auditRegNodeLink" href=reg_href>
                                        _title
                                    </a>
                                </li>_
                            } => reg_href;
                        </ul>_
                    } => entries;
                    let wef = if html_handler.peek().unwrap().value().as_comment().unwrap().contains("CourseCatalogue") {
                        <!--"ghFV6aOhMFy66ulVWC-xyzA5Lqi3uWdHa7LqLHaceWQ"-->_
                        <div class="tb">_
                            <div class="tbhead">
                                "Veranstaltungen / Module"
                            </div>_
                            <!--"tY3gu8Sk4aG_lsXAU_2a_w0_Efi8P3WOIpWjl2FxXDw"-->_
                            <!--"bZr1IgdrSm713Ht01158Vkl5zMzSBwIDp2ufIuDtU-g"-->_
                            let veranstaltungen = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "table" {
                                <table class="nb eventTable">_
                                    <tbody>
                                        <tr class="tbsubhead">_
                                            <th>
                                                <!--"P_nzuS6nMPntyFOEKnRuKsS4n5YXNP3TWd4dCLhMjaM"-->
                                            </th>_
                                            <th>
                                                "\n\t\t\t\t\tVeranstaltung / Modul"
                                                <br></br>
                                                "\n\t\t\t\t\tDozenten / Modulverantwortliche\n\t\t\t  \t\t\t\t\t   \t\t\t"
                                                <br></br>
                                                "Zeitraum\n\t\t\t  \t\t\t\t \t\t"
                                            </th>_
                                            <th>_
                                            </th>_
                                            <th colspan="2">
                                                "\n\t\t\t \t\t\t\t \t\t  \t\tVeranstaltungsart"
                                                <br></br>
                                                "\n\t\t \t\t  \t\tRaum\n\t\t \t\t  \t\t\t \t\t"
                                            </th>_
                                        </tr>_
                                        let ent = while html_handler.peek().is_some() {
                                            <tr class="tbdata">_
                                                <td>
                                                    <!--"P_nzuS6nMPntyFOEKnRuKsS4n5YXNP3TWd4dCLhMjaM"-->_
                                                </td>_
                                                <td>_
                                                    <a name="eventLink" href=coursedetails_url class="eventTitle">
                                                        title_url
                                                    </a>
                                                    <br></br>
                                                    name
                                                    let date_range = if html_handler.peek().is_some() {
                                                        <br></br>
                                                        date_range
                                                    } => date_range;
                                                </td>_
                                                <td>_
                                                </td>_
                                                <td colspan="2">
                                                    course_type
                                                </td>_
                                            </tr>_
                                        } => ();
                                    </tbody>
                                </table>_
                            } => () else {
                                <div class="tbdata" colspan="3">
                                    "\n\t\t\t\tEs wurden keine Veranstaltungen gefunden.\n\t\t\t"
                                </div>_
                            } => ();
                        </div>_
                    } => ();
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    }
    let html_handler = footer(html_handler, login_response.id, 326);
    html_handler.end_document();
    Ok(Vorlesungsverzeichnis { entries: entries.unwrap_or_default() })
}
