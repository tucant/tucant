use std::sync::LazyLock;

use regex::Regex;
use scraper::{ElementRef, Html};
use tucant_types::{
    LoginResponse,
    coursedetails::CourseDetailsRequest,
    moduledetails::ModuleDetailsRequest,
    registration::{AnmeldungCourse, AnmeldungEntry, AnmeldungExam, AnmeldungModule, AnmeldungRequest, AnmeldungResponse, RegistrationState, Studiumsauswahl},
};

use crate::{
    TucanConnector, TucanError, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
};
use html_handler::Root;

pub async fn anmeldung_cached(tucan: &TucanConnector, login_response: &LoginResponse, request: AnmeldungRequest) -> Result<AnmeldungResponse, TucanError> {
    let key = format!("registration.{}", request.inner());
    if let Some(anmeldung_response) = tucan.database.get(&key).await {
        return Ok(anmeldung_response);
    }

    let anmeldung_response = anmeldung(tucan, login_response, request).await?;

    tucan.database.put(&key, &anmeldung_response).await;

    Ok(anmeldung_response)
}

#[expect(clippy::too_many_lines)]
pub async fn anmeldung(tucan: &TucanConnector, login_response: &LoginResponse, args: AnmeldungRequest) -> Result<AnmeldungResponse, TucanError> {
    static REGISTRATION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N\\d+,-N000311,").unwrap());
    static MODULEDETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N\\d+,-N000311,").unwrap());
    static COURSEDETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N000311,").unwrap());
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\p{Alphabetic}{2}, \d{1,2}\. \p{Alphabetic}{3}\. \d{4} \[\d\d:\d\d\] - \p{Alphabetic}{2}, \d{1,2}\. \p{Alphabetic}{3}\. \d{4} \[\d\d:\d\d\]$").unwrap());
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{:015},-N000311,{}", login_response.id, args.inner());
    // TODO FIXME generalize
    let key = format!("unparsed_anmeldung.{}", args.inner());
    let content = if let Some(content) = tucan.database.get(&key).await {
        content
    } else {
        let content = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
        tucan.database.put(&key, &content).await;
        content
    };
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>_
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
                    </style>_
                    <style type="text/css">
                        "3CC0xpJgjHprYY59D1krvfwrI2LSV2-OtaN3CviYnG8"
                    </style>_
                </head>_
                <body class="registration">_
                    use logged_in_head(html_handler, login_response.id).0;
                    <!--"up71ljpj_w5JCBcjI0pvus0gS__0taKvkYJ-_QU1yNk"-->_
                    <script type="text/javascript">
                    </script>_
                    <h1>
                        "Anmeldung zu Modulen und Veranstaltungen"
                    </h1>_
                    let studiumsauswahl = if html_handler.peek().unwrap().value().is_comment() {
                        <!--"UU9Ju2ASETVrRfIpA3xWkFcE5n3oN4PCI9QksTmApIA"-->_
                        <form id="registration" action="/scripts/mgrqispi.dll" method="post">_
                            <table class="tbcoursestatus rw-table rw-all">_
                                <tbody>
                                    <tr>_
                                        <td class="tbhead" colspan="100%">
                                            "Weitere Studien"
                                        </td>_
                                    </tr>_
                                    <tr>_
                                        <td class="tbcontrol" colspan="100%">_
                                            <div class="inputFieldLabel">_
                                                <label for="study">
                                                    "Studium:"
                                                </label>_
                                                <select name="study" id="study" onchange="reloadpage.submitForm(this.form.id);" class="pageElementLeft">_
                                                    let studiumsauswahl = while html_handler.peek().is_some() {
                                                        let studiumsauswahl = if html_handler.peek().unwrap().value().as_element().unwrap().attr("selected").is_some() {
                                                            <option value=value selected="selected">
                                                                name
                                                            </option>_
                                                        } => Studiumsauswahl { name, value, selected: true } else {
                                                            <option value=value>
                                                                name
                                                            </option>_
                                                        } => Studiumsauswahl { name, value, selected: false };
                                                    } => studiumsauswahl.either_into();
                                                </select>_
                                                <input name="Aktualisieren" type="submit" value="Aktualisieren" class="img img_arrowReload pageElementLeft"></input>_
                                            </div>_
                                            <input name="APPNAME" type="hidden" value="CampusNet"></input>_
                                            <input name="PRGNAME" type="hidden" value="REGISTRATION"></input>_
                                            <input name="ARGUMENTS" type="hidden" value="sessionno,menuno,study,changestudy,parent1,parent2"></input>_
                                            <input name="sessionno" type="hidden" value={|v: String| {
                                                static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^\\d+$").unwrap());
                                                assert!(REGEX.is_match(&v), "{v}");
                                            }}></input>_
                                            <input name="menuno" type="hidden" value="000311"></input>_
                                            <input name="pa rent1" type="hidden" value="000000000000000"></input>_
                                            <input name="parent2" type="hidden" value="000000000000000"></input>_
                                            <input name="changestudy" type="hidden" value="1"></input>_
                                        </td>_
                                    </tr>_
                                </tbody>
                            </table>_
                        </form>_
                        <!--"mrUJOOH3fqYzcWGWygCuNQGMPfDRh8akKXEihfucyR0"-->_
                    } => studiumsauswahl;
                    <h2>_
                        <a href=registration_url>
                            study
                        </a>
                        let path = while !html_handler.peek().unwrap().value().as_text().unwrap().trim().is_empty() {
                            "\n        \u{a0}>\u{a0}\n                "
                            <a href=url>
                                let any_child = html_handler.next_any_child();
                            </a>
                        } => match any_child.value() {
                            scraper::Node::Comment(_comment) => None,
                            scraper::Node::Text(text) => {
                                let url = REGISTRATION_REGEX.replace(&url, "");
                                Some((text.to_string(), AnmeldungRequest::parse(&url)))
                            }
                            _ => panic!(),
                        };
                        extern {
                            let registration_url = REGISTRATION_REGEX.replace(&registration_url, "");
                            path.insert(0, Some((study, AnmeldungRequest::parse(&registration_url))));
                        }_
                    </h2>_
                    let submenus = if html_handler.peek().is_some() && html_handler.peek().unwrap().value().is_element() {
                        <ul>_
                            let submenus = while html_handler.peek().is_some() {
                                <li>_
                                    <a href=url>
                                        item
                                    </a>_
                                </li>_
                            } => (item.trim().to_owned(), AnmeldungRequest::parse(&REGISTRATION_REGEX.replace(&url, "")));
                        </ul>_
                    } => submenus;
                    <!--"gACLM-J4jmb4gKmvgI-c8EqENeLydqGZuryaUY-7Lm4"-->_
                    let additional_information = while !html_handler.peek().unwrap().value().is_comment() {
                        let child = html_handler.next_any_child();
                    } => match child.value() {
                        scraper::Node::Text(text) => {
                            assert!(text.trim().is_empty());
                            None
                        }
                        scraper::Node::Element(_element) => {
                            Some(ElementRef::wrap(child).unwrap().html())
                        }
                        _ => panic!(),
                    };
                    <!--"PQQwWAU_NypeYX1Jw191sjka_fWLRqDlYVWZm-gWSFs"-->_
                    <br></br>_
                    <!--"9XmEOh66hIETO2XPWUf_msfayuKwcwW3Q-0NvQQ6mvA"-->_
                    let anmeldung_entries = if html_handler.peek().unwrap().value().is_element() {
                        <table class="tbcoursestatus rw-table rw-all">_
                            <tbody>
                                <tr>_
                                    <td class="tbhead" colspan="100%">
                                        "Anmeldung zu Modulen und Veranstaltungen"
                                    </td>_
                                </tr>_
                                <tr>_
                                    let anmeldung_entries = if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap() == "tbdata" {
                                                    <td class="tbdata" colspan="4">
                                                        "Keine Module oder Veranstaltungen zur Anmeldung gefunden"
                                                    </td>_
                                                </tr>_
                                            </tbody>
                                        </table>_
                                    } => () else {
                                                    <td class="tbsubhead">_
                                                        <!--"OyACS3xJTkWGHAVncWgagM4cYhq_aivzGyGMi9Ycvhc"-->_
                                                    </td>_
                                                    <td class="tbsubhead">
                                                        "\n\n\t\t\t\t\t\tVeranstaltung"
                                                        <br></br>
                                                        "\n\t\t\t\t\t\tDozenten\n\t\t\t\t\t\t\t\t\t\t\t\t\t"
                                                        <br></br>
                                                        "Zeitraum\n\t\t\t\t\t\t\t\t\t\t\t\t"
                                                        <br></br>
                                                        "Anmeldegruppe\n\t\t\t\t\t\t"
                                                        <br></br>
                                                        "Standort\n\t\t\t\t\t"
                                                    </td>_
                                                    <td class="tbsubhead">
                                                        "\n\t\t\t\t\t\t\t\t\t\t\t\tAnmeld. bis\n\t\t\t\t\t\t\t\t\t\t"
                                                        <br></br>
                                                        "\n\t\t\t\t\tMax.Teiln.|Anm.\n\t\t\t   "
                                                    </td>_
                                                    <td class="tbsubhead">_
                                                    </td>_
                                                </tr>_
                                                let anmeldung_entries = while html_handler.peek().is_some() {
                                                    let module = if html_handler.peek().is_some() && html_handler.peek().unwrap().children().nth(1).unwrap().value().as_comment().unwrap().to_string() == "logo column" {
                                                        <tr>_
                                                            <!--"cKueW5TXNZALIFusa3P6ggsr9upFINMVVycC2TDTMY4"-->_
                                                            <td class="tbsubhead">_
                                                                <!--"5IqHfue5CE0Heo5nzO7DJGi3oBaXZc5Ldk_iJ-M2h-0"-->_
                                                            </td>_
                                                            <!--"Oed-0ppULuj5oPWBUECe-K3BAgMKxIzcX4-pZZuvMjU"-->_
                                                            <td class="tbsubhead dl-inner">_
                                                                <p>
                                                                    <strong>
                                                                        <a href=module_url>
                                                                            module_id
                                                                            <span class="eventTitle">
                                                                                module_name
                                                                            </span>
                                                                        </a>
                                                                    </strong>
                                                                </p>_
                                                                <p>
                                                                    lecturer
                                                                </p>_
                                                            </td>_
                                                            <td class="tbsubhead">
                                                                date
                                                                <br></br>
                                                                limit_and_size
                                                            </td>_
                                                            <td class="tbsubhead rw-qbf">_
                                                                let registration_button_link = if html_handler.peek().is_some() {
                                                                    let registered = if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap() == "img noFloat register" {
                                                                        <a href=registration_button_link class="img noFloat register">
                                                                            "Anmelden"
                                                                        </a>_
                                                                    } => RegistrationState::NotRegistered { register_link: registration_button_link } else {
                                                                        <a href=registration_button_link class="img img_arrowLeftRed noFLoat unregister">
                                                                            "Abmelden"
                                                                        </a>_
                                                                    } => RegistrationState::Registered { unregister_link: registration_button_link };
                                                                } => registered.either_into::<RegistrationState>() else {
                                                                } => RegistrationState::Unknown;
                                                            </td>_
                                                            <!--"o10-cLtyMRZ7GTG_AsgU91-xv5MS_W-LjurxsulBAKI"-->_
                                                            <!--"-SsWn7gBGa5GC1Ds7oXC-dHS2kBuF2yJjZzwt6ieu_E"-->_
                                                            <!--"EfR5cxw_o8B_kd0pjKiSGEdMGoTwEUFKD7nwyOK5Qhc"-->_
                                                            <!--"I1qHM7Q-rAMXujuYDjTzmkkUzH0c2zK1Z43rc_xoiIY"-->_
                                                            <!--"1SjHxH8_QziRK63W2_1gyP4qaAMQP4Wc0Bap0cE8px8"-->_
                                                            <!--"ybVEa17xGUste1jxqx8VN9yhVuTCZICjBaDfIp7y728"-->_
                                                        </tr>_
                                                    } => {
                                                        let module_url = MODULEDETAILS_REGEX.replace(&module_url, "");
                                                        let module_url = module_url.split_once(",-A").unwrap().0;
                                                        let module = AnmeldungModule {
                                                            url: ModuleDetailsRequest::parse(module_url),
                                                            id: module_id.trim().to_owned(),
                                                            name: module_name,
                                                            lecturer: if lecturer == "N.N." { None } else { Some(lecturer) },
                                                            date: date.trim().to_owned(),
                                                            limit_and_size: limit_and_size.trim().to_owned(),
                                                            registration_button_link: registration_button_link.either_into(),
                                                        };
                                                        module
                                                    };
                                                    let courses = while html_handler.peek().is_some() && html_handler.peek().unwrap().children().nth(1).unwrap().value().as_comment().unwrap().to_string() != "logo column" {
                                                        let exam = if !html_handler.peek().unwrap().children().nth(5).unwrap().value().is_comment() {
                                                            <tr>_
                                                                <!--"o10-cLtyMRZ7GTG_AsgU91-xv5MS_W-LjurxsulBAKI"-->_
                                                                <!--"-SsWn7gBGa5GC1Ds7oXC-dHS2kBuF2yJjZzwt6ieu_E"-->_
                                                                <td class="tbdata">_
                                                                    <!--"r60FpxPoqFJu64MiLDBXezdJpTET0vVgi2dvCZ0TUI8"-->_
                                                                </td>_
                                                                <td class="tbdata">
                                                                    exam_name
                                                                    let exam_type = if html_handler.peek().is_some() {
                                                                        <br></br>
                                                                        exam_type
                                                                    } => exam_type;
                                                                </td>_
                                                                <td class="tbdata">_
                                                                </td>_
                                                                <td class="tbdata">_
                                                                </td>_
                                                                <!--"EfR5cxw_o8B_kd0pjKiSGEdMGoTwEUFKD7nwyOK5Qhc"-->_
                                                                <!--"I1qHM7Q-rAMXujuYDjTzmkkUzH0c2zK1Z43rc_xoiIY"-->_
                                                                <!--"1SjHxH8_QziRK63W2_1gyP4qaAMQP4Wc0Bap0cE8px8"-->_
                                                                <!--"ybVEa17xGUste1jxqx8VN9yhVuTCZICjBaDfIp7y728"-->_
                                                            </tr>_
                                                        } => AnmeldungExam { name: exam_name.trim().to_owned(), typ: exam_type };
                                                        <tr>_
                                                            <!--"o10-cLtyMRZ7GTG_AsgU91-xv5MS_W-LjurxsulBAKI"-->_
                                                            <!--"-SsWn7gBGa5GC1Ds7oXC-dHS2kBuF2yJjZzwt6ieu_E"-->_
                                                            <!--"EfR5cxw_o8B_kd0pjKiSGEdMGoTwEUFKD7nwyOK5Qhc"-->_
                                                            <!--"I1qHM7Q-rAMXujuYDjTzmkkUzH0c2zK1Z43rc_xoiIY"-->_
                                                            <!--"1SjHxH8_QziRK63W2_1gyP4qaAMQP4Wc0Bap0cE8px8"-->_
                                                            <!--"cKueW5TXNZALIFusa3P6ggsr9upFINMVVycC2TDTMY4"-->_
                                                            <td class="tbdata">_
                                                                let gefaehrdung_schwangere = if html_handler.peek().is_some() {
                                                                    <img src="../../gfx/_default/icons/eventIcon.gif" title="Gefährdungspotential für Schwangere"></img>_
                                                                } => ();
                                                            </td>_
                                                            <td class="tbdata dl-inner">_
                                                                <p>
                                                                    <strong>
                                                                        <a href=course_url name="eventLink">
                                                                            course_id
                                                                            <span class="eventTitle">
                                                                                course_name
                                                                            </span>
                                                                        </a>
                                                                    </strong>
                                                                </p>_
                                                                <p>
                                                                    let lecturers = if html_handler.peek().is_some() && !RE.is_match(html_handler.peek().unwrap().value().as_text().unwrap()) {
                                                                            lecturers
                                                                        </p>_
                                                                        <p>
                                                                    } => lecturers;
                                                                    let begin_and_end = if html_handler.peek().is_some() {
                                                                            begin_and_end
                                                                        </p>_
                                                                        <p>
                                                                    } => begin_and_end;
                                                                    let location_or_additional_info = if html_handler.peek().is_some() {
                                                                            let location_or_additional_info = html_handler.next_any_child();
                                                                        </p>_
                                                                    } => match location_or_additional_info.value() {
                                                                        scraper::Node::Text(text) => text.trim().to_owned(),
                                                                        scraper::Node::Element(_element) => ElementRef::wrap(location_or_additional_info).unwrap().html(),
                                                                        _ => panic!(),
                                                                    } else {
                                                                        </p>_
                                                                    } => ();
                                                                let location = if html_handler.peek().is_some() {
                                                                    <p>
                                                                        let location = if html_handler.peek().is_some() {
                                                                            location
                                                                        } => location;
                                                                    </p>_
                                                                } => location;
                                                            </td>_
                                                            <td class="tbdata">
                                                                registration_until
                                                                <br></br>
                                                                limit_and_size
                                                            </td>_
                                                            <td class="tbdata rw-qbf">_
                                                                let registration_button_link = if html_handler.peek().is_some() {
                                                                    let registration_button_link = if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap() == "img noFLoat register" {
                                                                        <a href=registration_button_link class="img noFLoat register">
                                                                            "Anmelden"
                                                                        </a>_
                                                                    } => RegistrationState::NotRegistered { register_link: registration_button_link } else {
                                                                        <a href=registration_button_link class="img img_arrowLeftRed noFLoat unregister">
                                                                            " Abmelden"
                                                                        </a>_
                                                                    } => RegistrationState::Registered { unregister_link: registration_button_link };
                                                                } => registration_button_link.either_into::<RegistrationState>() else {
                                                                } => RegistrationState::Unknown;
                                                            </td>_
                                                            <!--"ybVEa17xGUste1jxqx8VN9yhVuTCZICjBaDfIp7y728"-->_
                                                        </tr>_
                                                    } => {
                                                        let course_url = COURSEDETAILS_REGEX.replace(&course_url, "");
                                                        let course_url = course_url.split_once(",-A").unwrap().0;
                                                        let course = AnmeldungCourse {
                                                            gefaehrdung_schwangere: gefaehrdung_schwangere.is_some(),
                                                            url: CourseDetailsRequest::parse(course_url),
                                                            id: course_id.trim().to_owned(),
                                                            name: course_name.trim().to_owned(),
                                                            lecturers,
                                                            begin_and_end,
                                                            registration_until: registration_until.trim().to_owned(),
                                                            limit_and_size: limit_and_size.trim().to_owned(),
                                                            registration_button_link: registration_button_link.either_into(),
                                                            location_or_additional_info: location_or_additional_info.left(),
                                                            location: location.flatten(),
                                                        };
                                                        (exam, course)
                                                    };
                                                } => AnmeldungEntry { module, courses };
                                            </tbody>
                                        </table>_
                                    } => anmeldung_entries;
                    } => anmeldung_entries.right().unwrap_or_default();
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    };
    let _html_handler = footer(html_handler, id, 311);
    Ok(AnmeldungResponse {
        path: path.into_iter().flatten().collect(),
        submenus: submenus.unwrap_or_default(),
        entries: anmeldung_entries.unwrap_or_default(),
        additional_information: additional_information.into_iter().flatten().collect(),
        studiumsauswahl: studiumsauswahl.unwrap_or_default(),
    })
}
