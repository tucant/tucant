use crate::{
    COURSEDETAILS_REGEX, TucanConnector, authenticated_retryable_get,
    head::{footer, html_head, logged_in_head, logged_out_head},
};
use data_encoding::BASE64URL_NOPAD;
use html_handler::{MyElementRef, MyNode, Root, parse_document};
use itertools::{Either, Itertools};
use log::info;
use scraper::CaseSensitivity;
use sha3::{Digest, Sha3_256};
use time::{Duration, OffsetDateTime};
use tucant_types::{
    InstructorImage, LoginResponse, RevalidationStrategy, TucanError,
    coursedetails::{CourseAnmeldefrist, CourseDetailsRequest, CourseDetailsResponse, CourseUebungsGruppe, InstructorImageWithLink, Room, Termin},
};

pub async fn course_details(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    revalidation_strategy: RevalidationStrategy,
    request: CourseDetailsRequest,
) -> Result<CourseDetailsResponse, TucanError> {
    let key = format!("unparsed_course_details.{}", request.inner());

    let old_content_and_date = tucan.database.get::<(String, OffsetDateTime)>(&key).await;
    if revalidation_strategy.max_age != 0 {
        if let Some((content, date)) = &old_content_and_date {
            info!("{}", OffsetDateTime::now_utc() - *date);
            if OffsetDateTime::now_utc() - *date < Duration::seconds(revalidation_strategy.max_age) {
                return course_details_internal(login_response, content, &request);
            }
        }
    }

    let Some(invalidate_dependents) = revalidation_strategy.invalidate_dependents else {
        return Err(TucanError::NotCached);
    };

    let url = format!(
        "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N{:015},-N000311,{}",
        login_response.id,
        request.inner()
    );
    let (content, date) = authenticated_retryable_get(tucan, &url, &login_response.cookie_cnsc).await?;
    let result = course_details_internal(login_response, &content, &request)?;
    if invalidate_dependents && old_content_and_date.as_ref().map(|m| &m.0) != Some(&content) {
        // TODO invalidate cached ones?
    }

    tucan.database.put(&key, (content, date)).await;

    Ok(result)
}

fn h(input: &str) -> String {
    BASE64URL_NOPAD.encode(&Sha3_256::digest(input))
}

#[expect(clippy::similar_names, clippy::too_many_lines, clippy::cognitive_complexity)]
fn course_details_internal(login_response: &LoginResponse, content: &str, request: &CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
    let document = parse_document(content);
    let html_handler = Root::new(document.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "lbOQfuwTSH1NQfB9sjkC-_xOS0UGzyKBoNNl8bXs_FE"
                    </style>
                    <style type="text/css">
                        "qZ_1IiJLIcPvkbl6wYm5QbasBhsSKdRw5fl6vVyINxY"
                    </style>
                </head>
                <body class="coursedetails">
                    use if login_response.id == 1 {
                        logged_out_head(html_handler).0
                    } else {
                        logged_in_head(html_handler, login_response.id).0
                    };
                    <script type="text/javascript">
                    </script>
                    <script type="text/javascript">
                        _trash
                    </script>
                    <form name="courseform" action="/scripts/mgrqispi.dll" method="post">
                        let id_and_name = if html_handler.peek().unwrap().value().as_element().unwrap().attrs().next().is_some() {
                            <h1 class="eventTitle img img_arrowEventIcon" title="Gefährdungspotential für Schwangere">
                                id_and_name
                            </h1>
                        } => id_and_name else {
                            <h1>
                                id_and_name
                            </h1>
                        } => id_and_name;
                        let _kleingruppe = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "h2" {
                            <h2>
                                _kleingruppe
                            </h2>
                        } => ();
                        <div class="contentlayoutleft" id="contentlayoutleft">
                            <table class="tb rw-table rw-all">
                                <caption>
                                    "Veranstaltungsdetails"
                                </caption>
                                <tbody>
                                    <tr>
                                        <td class="tbcontrol" colspan="3">
                                            let material_and_messages_url = if html_handler.peek().is_some() {
                                                <a href=material_url class="arrow">
                                                    "Material"
                                                </a>
                                                use html_handler.skip_any_comment();
                                                <a href=messages_url class="arrow">
                                                    "Nachrichten"
                                                </a>
                                            } => (material_url, messages_url);
                                        </td>
                                    </tr>
                                    <tr>
                                        <td class="tbdata" colspan="3">
                                            let dozent = if &**html_handler
                                                .peek()
                                                .unwrap()
                                                .first_child()
                                                .unwrap()
                                                .first_child()
                                                .unwrap()
                                                .value()
                                                .as_text()
                                                .unwrap()
                                                == "Lehrende:" {
                                                <p>
                                                    <b>
                                                        "Lehrende:"
                                                    </b>
                                                    <span id="dozenten">
                                                        dozent
                                                    </span>
                                                </p>
                                            } => dozent;
                                            let course_type_and_number = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="coursetyp" value=course_type_number></input>
                                            } => ("unknown".to_owned(), course_type_number) else {
                                                <p>
                                                    <b>
                                                        "Veranstaltungsart:"
                                                    </b>
                                                    course_type
                                                    <input type="hidden" name="coursetyp" value=course_type_number></input>
                                                </p>
                                            } => (course_type, course_type_number);
                                            <p>
                                                <b>
                                                    "Orga-Einheit:"
                                                </b>
                                                <span name="courseOrgUnit">
                                                    fachbereich
                                                </span>
                                            </p>
                                            <p>
                                                <b>
                                                    "Anzeige im Stundenplan:"
                                                </b>
                                                let anzeige_im_stundenplan = if html_handler.peek().unwrap().value().is_text() {
                                                    anzeige_im_stundenplan
                                                } => anzeige_im_stundenplan;
                                                <input type="hidden" name="shortdescription" value=shortname></input>
                                            </p>
                                            <input type="hidden" name="courselevel" value=courselevel></input>
                                            let _unused = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="coursearea" value=""></input>
                                            } => () else {
                                                <p>
                                                    <b>
                                                        "Fach:"
                                                    </b>
                                                    <input type="hidden" name="coursearea" value=""></input>
                                                </p>
                                            } => ();
                                            let _unused = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="creditingfor" value=""></input>
                                            } => () else {
                                                <p>
                                                    <b>
                                                        "Anrechenbar für:"
                                                    </b>
                                                    <input type="hidden" name="creditingfor" value=""></input>
                                                </p>
                                            } => ();
                                            let sws = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="sws" value="0"></input>
                                            } => () else {
                                                <p>
                                                    <b>
                                                        "Semesterwochenstunden:"
                                                    </b>
                                                    sws_text
                                                    <input type="hidden" name="sws" value=sws></input>
                                                </p>
                                            } => {
                                                assert_eq!(sws_text, sws);
                                                sws
                                            };
                                            let credits = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="credits" value="  0,0"></input>
                                            } => () else {
                                                <p>
                                                    <b>
                                                        "Credits:"
                                                    </b>
                                                    credits_text
                                                    <input type="hidden" name="credits" value=credits></input>
                                                </p>
                                            } => {
                                                assert_eq!(credits_text, credits.trim());
                                                credits_text
                                            };
                                            <input type="hidden" name="location" value="327576461398991"></input>
                                            let language_and_id = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="language" value=language_id></input>
                                            } => ("unknown".to_owned(), language_id) else {
                                                <p>
                                                    <b>
                                                        "Unterrichtssprache:"
                                                    </b>
                                                    <span name="courseLanguageOfInstruction">
                                                        language
                                                    </span>
                                                    <input type="hidden" name="language" value=language_id></input>
                                                </p>
                                            } => (language, language_id);
                                            let teilnehmer = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
                                                <input type="hidden" name="min_participantsno" value=teilnehmer_min></input>
                                                <input type="hidden" name="max_participantsno" value=teilnehmer_max></input>
                                            } => (format!("{teilnehmer_min} | {teilnehmer_max}"), teilnehmer_min, teilnehmer_max) else {
                                                <p>
                                                    <b>
                                                        "Min. | Max. Teilnehmerzahl:"
                                                    </b>
                                                    teilnehmer_range
                                                    <input type="hidden" name="min_participantsno" value=teilnehmer_min></input>
                                                    <input type="hidden" name="max_participantsno" value=teilnehmer_max></input>
                                                </p>
                                            } => (teilnehmer_range, teilnehmer_min, teilnehmer_max);
                                            let description = while html_handler.peek().is_some() {
                                                let child = html_handler.next_any_child();
                                            } => match child.value() {
                                                MyNode::Text(text) => text.to_string(),
                                                MyNode::Element(_element) => MyElementRef::wrap(child).unwrap().html(),
                                                _ => panic!(),
                                            };
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                            let uebungsgruppen = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "div" {
                                <div class="tb">
                                    <div>
                                        <div class="tbhead">
                                            "Kleingruppe(n)"
                                        </div>
                                        <div class="tbdata">
                                            "Die Veranstaltung ist in die folgenden Kleingruppen aufgeteilt:"
                                            let plenumsveranstaltung_url = if html_handler.peek().is_some() {
                                                <a href=coursedetails_url class="img img_arrowLeft pageElementRight">
                                                    "Plenumsveranstaltung anzeigen"
                                                </a>
                                            } => CourseDetailsRequest::parse(&COURSEDETAILS_REGEX.replace(&coursedetails_url, ""));
                                        </div>
                                    </div>
                                    <ul class="dl-ul-listview">
                                        let uebungsgruppen = if html_handler.peek().unwrap().children().count() == 1 {
                                            <li class="tbdata listelement">
                                                "Es sind keine Kleingruppen eingerichtet."
                                            </li>
                                        } => Vec::<CourseUebungsGruppe>::new() else {
                                            let uebungsgruppen = while html_handler.peek().is_some() {
                                                let uebungsgruppe = if html_handler
                                                    .peek()
                                                    .unwrap()
                                                    .value()
                                                    .as_element()
                                                    .unwrap()
                                                    .has_class("tbsubhead", CaseSensitivity::CaseSensitive) {
                                                    <li class="tbsubhead listelement">
                                                        <div class="dl-inner">
                                                            <p class="dl-ul-li-headline">
                                                                <strong>
                                                                    uebung_name
                                                                </strong>
                                                            </p>
                                                            <p>
                                                                uebungsleiter
                                                            </p>
                                                            <p>
                                                                let date_range = if html_handler.peek().is_some() {
                                                                    date_range
                                                                } => date_range;
                                                            </p>
                                                        </div>
                                                        <div class="dl-link">
                                                            <p>
                                                                "Diese Kleingruppe wird aktuell angezeigt."
                                                            </p>
                                                        </div>
                                                    </li>
                                                } => CourseUebungsGruppe {
                                                    date_range,
                                                    name: uebung_name,
                                                    uebungsleiter,
                                                    url: request.clone(),
                                                    active: true
                                                } else {
                                                    <li class="tbdata listelement">
                                                        <div class="dl-inner">
                                                            <p class="dl-ul-li-headline">
                                                                <strong>
                                                                    uebung_name
                                                                </strong>
                                                            </p>
                                                            <p>
                                                                uebungsleiter
                                                            </p>
                                                            <p>
                                                                let date_range = if html_handler.peek().is_some() {
                                                                    date_range
                                                                } => date_range;
                                                            </p>
                                                        </div>
                                                        <div class="dl-link">
                                                            <a href=url class="img img_arrowLeft pageElementRight">
                                                                "Kleingruppe anzeigen"
                                                            </a>
                                                        </div>
                                                    </li>
                                                } => CourseUebungsGruppe {
                                                    date_range,
                                                    name: uebung_name,
                                                    uebungsleiter,
                                                    url: CourseDetailsRequest::parse(&COURSEDETAILS_REGEX.replace(&url, "")),
                                                    active: false
                                                };
                                            } => uebungsgruppe.either_into();
                                        } => uebungsgruppen;
                                    </ul>
                                </div>
                            } => (plenumsveranstaltung_url, uebungsgruppen.either_into());
                            <table class="tb rw-table">
                                <caption>
                                    "Literatur"
                                </caption>
                                <tbody>
                                    <tr>
                                        <td class="tbsubhead">
                                            <span name="literatureCategory">
                                            </span>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                            let _kein_material = if html_handler
                                .peek()
                                .unwrap()
                                .first_child()
                                .unwrap()
                                .first_child()
                                .unwrap()
                                .value()
                                .as_text()
                                .is_some_and(|v| &**v == "Material zur gesamten Veranstaltung") {
                                <table class="tb rw-table">
                                    <caption>
                                        "Material zur gesamten Veranstaltung"
                                    </caption>
                                    <tbody>
                                        <tr>
                                            <td class="tbdata" colspan="3">
                                                "Es liegt kein Material vor."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            } => ();
                            let course_anmeldefristen = if html_handler
                                .peek()
                                .unwrap()
                                .first_child()
                                .unwrap()
                                .first_child()
                                .unwrap()
                                .value()
                                .as_text()
                                .is_none_or(|v| &**v != "Termine") {
                                let course_anmeldefristen = if !html_handler
                                    .peek()
                                    .unwrap()
                                    .value()
                                    .as_element()
                                    .unwrap()
                                    .has_class("list", CaseSensitivity::CaseSensitive) {
                                    <table class="tb rw-table">
                                        <tbody>
                                            <tr>
                                                <td class="tbhead" colspan="6">
                                                    "Anmeldefristen"
                                                </td>
                                            </tr>
                                            <tr>
                                                <td class="tbdata">
                                                    "Für diese Veranstaltung sind keine Anmeldephasen eingerichtet. Sie können sich zu der Veranstaltung nicht über das Webportal anmelden."
                                                </td>
                                            </tr>
                                        </tbody>
                                    </table>
                                } => Vec::<CourseAnmeldefrist>::new() else {
                                    <table class="tb list rw-table">
                                        <caption>
                                            "Anmeldefristen"
                                        </caption>
                                        <tbody>
                                            <tr>
                                                <td class="tbsubhead">
                                                    "Phase"
                                                </td>
                                                <td class="tbsubhead">
                                                    "Block"
                                                </td>
                                                <td class="tbsubhead">
                                                    "Start"
                                                </td>
                                                <td class="tbsubhead">
                                                    "Ende Anmeldung"
                                                </td>
                                                <td class="tbsubhead">
                                                    "Ende Abmeldung"
                                                </td>
                                                <td class="tbsubhead">
                                                    "Ende Hörer"
                                                </td>
                                            </tr>
                                            let course_anmeldefristen = while html_handler.peek().is_some() {
                                                <tr>
                                                    <td class="tbdata">
                                                        zulassungstyp
                                                    </td>
                                                    <td class="tbdata">
                                                        block_type
                                                    </td>
                                                    <td class="tbdata">
                                                        let start = if html_handler.peek().is_some() {
                                                            start
                                                        } => start;
                                                    </td>
                                                    <td class="tbdata">
                                                        let ende_anmeldung = if html_handler.peek().is_some() {
                                                            ende_anmeldung
                                                        } => ende_anmeldung;
                                                    </td>
                                                    <td class="tbdata">
                                                        let ende_abmeldung = if html_handler.peek().is_some() {
                                                            ende_abmeldung
                                                        } => ende_abmeldung;
                                                    </td>
                                                    <td class="tbdata">
                                                        let ende_hoerer = if html_handler.peek().is_some() {
                                                            ende_hoerer
                                                        } => ende_hoerer;
                                                    </td>
                                                </tr>
                                            } => CourseAnmeldefrist {
                                                zulassungstyp,
                                                block_type,
                                                start,
                                                ende_anmeldung,
                                                ende_abmeldung,
                                                ende_hoerer
                                            };
                                        </tbody>
                                    </table>
                                } => course_anmeldefristen;
                            } => course_anmeldefristen;
                            <table class="tb list rw-table rw-all">
                                <caption>
                                    "Termine"
                                </caption>
                                <tbody>
                                    <tr class="rw-hide">
                                        <td class="tbsubhead">
                                        </td>
                                        <td class="tbsubhead" style="width:120px;">
                                            "Datum"
                                        </td>
                                        <td class="tbsubhead">
                                            "Von"
                                        </td>
                                        <td class="tbsubhead">
                                            "Bis"
                                        </td>
                                        <td class="tbsubhead">
                                            "Raum"
                                        </td>
                                        <td class="tbsubhead">
                                            let _lehrende = if html_handler.peek().is_some() {
                                                "Lehrende"
                                            } => ();
                                        </td>
                                    </tr>
                                    let termine = if html_handler.peek().unwrap().first_child().unwrap().value().as_element().unwrap().attr("colspan").is_some() {
                                        <tr>
                                            <td class="tbdata" colspan="6">
                                                "Es liegen keine Termine vor."
                                            </td>
                                        </tr>
                                    } => Vec::<Termin>::new() else {
                                        let termine = while html_handler.peek().is_some() {
                                            <tr>
                                                <td class="tbdata rw">
                                                    id
                                                </td>
                                                <td class="tbdata rw rw-course-date" name="appointmentDate">
                                                    date
                                                </td>
                                                <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">
                                                    time_start
                                                </td>
                                                <td class="tbdata rw rw-course-to" name="appointmentDateTo">
                                                    time_end
                                                </td>
                                                <td class="tbdata rw rw-course-room">
                                                    let rooms = if html_handler.peek().is_some()
                                                        && html_handler.peek().unwrap().value().is_element() {
                                                        let room = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "a" {
                                                            <a name="appointmentRooms" href=room_url>
                                                                room
                                                            </a>
                                                        } => Room { name: room, url: Some(room_url) } else {
                                                            <span name="appointmentRooms">
                                                                room
                                                            </span>
                                                        } => Room { name: room, url: None };
                                                        let more_rooms = while html_handler.peek().is_some() {
                                                            ","
                                                            let room = if html_handler.peek().unwrap().value().as_element().unwrap().name()
                                                                == "a" {
                                                                <a name="appointmentRooms" href=room_url>
                                                                    room
                                                                </a>
                                                            } => Room { name: room, url: Some(room_url) } else {
                                                                <span name="appointmentRooms">
                                                                    room
                                                                </span>
                                                            } => Room { name: room, url: None };
                                                        } => room.either_into();
                                                    } => std::iter::once(room.either_into()).chain(more_rooms.into_iter()).collect::<Vec<_>>() else {
                                                        let room = if html_handler.peek().is_some() {
                                                            room_text
                                                        } => vec![Room { name: room_text, url: None }] else {
                                                        } => Vec::<Room>::new();
                                                    } => room.either_into::<Vec<Room>>();
                                                </td>
                                                <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">
                                                    let instructors = if html_handler.peek().is_some() {
                                                        instructors
                                                    } => instructors;
                                                </td>
                                            </tr>
                                        } => Termin {
                                            id,
                                            date,
                                            time_start,
                                            time_end,
                                            instructors,
                                            rooms: rooms.either_into()
                                        };
                                    } => termine;
                                </tbody>
                            </table>
                            let enthalten_in_modulen = if html_handler.peek().is_some() && login_response.id != 1 {
                                <table class="tb rw-table rw-all">
                                    <caption>
                                        "Enthalten in Modulen"
                                    </caption>
                                    <tbody>
                                        <tr>
                                            <td class="tbsubhead">
                                                "Modul"
                                            </td>
                                        </tr>
                                        let enthalten_in_modulen = if html_handler
                                            .peek()
                                            .unwrap()
                                            .first_child()
                                            .unwrap()
                                            .value()
                                            .as_element()
                                            .unwrap()
                                            .attr("colspan")
                                            .is_none() {
                                            let enthalten_in_modulen = while html_handler.peek().is_some() {
                                                <tr>
                                                    <td class="tbdata">
                                                        module_name
                                                    </td>
                                                </tr>
                                            } => module_name;
                                        } => enthalten_in_modulen else {
                                            <tr>
                                                <td class="tbdata" colspan="2">
                                                    "Keine Module gefunden"
                                                </td>
                                            </tr>
                                        } => Vec::<String>::new();
                                    </tbody>
                                </table>
                            } => enthalten_in_modulen.either_into();
                        </div>
                        <div class="contentlayoutright" id="contentlayoutright">
                            <div class="tb courseList">
                                <div class="tbhead">
                                    "Übersicht der Kurstermine"
                                </div>
                                <ul class="courseList">
                                    let short_termine = if **html_handler.peek().unwrap().children().next().unwrap().value().as_text().unwrap()
                                        == *"Es liegen keine Termine vor." {
                                        <li class="courseListCell noLink">
                                            "Es liegen keine Termine vor."
                                        </li>
                                    } => Vec::<(String, String)>::new() else {
                                        let short_termine = while html_handler.peek().is_some() {
                                            extern {
                                                let mut i = 0;
                                            }
                                            let short_termine = while i < 5 {
                                                let short_termin = if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap()
                                                    == "courseListCell numout" {
                                                    <li class="courseListCell numout" title=title xss="">
                                                        number
                                                    </li>
                                                    let _comment = if i == 4 {
                                                    } => ();
                                                } => (title, number) else {
                                                    <li class="courseListCell noLink">
                                                    </li>
                                                } => ();
                                                extern {
                                                    i += 1;
                                                }
                                            } => short_termin.left();
                                        } => short_termine.into_iter().flatten().collect::<Vec<_>>();
                                    } => short_termine.into_iter().flatten().collect::<Vec<_>>();
                                </ul>
                            </div>
                            let instructors = if html_handler.peek().is_some() {
                                <table class="tb rw-table">
                                    <tbody>
                                        <tr class="rw-all">
                                            <td class="tbhead">
                                                "Lehrende"
                                            </td>
                                        </tr>
                                        let instructors = while html_handler.peek().is_some() {
                                            let instructor_image = if html_handler
                                                .peek()
                                                .unwrap()
                                                .first_child()
                                                .unwrap()
                                                .value()
                                                .as_element()
                                                .unwrap()
                                                .attr("name")
                                                .is_none() {
                                                <tr>
                                                    <td class="tbdata_nob h_center">
                                                        <a href=href>
                                                            <img src=imgsrc width="120" height="160" border="0" alt=alt></img>
                                                        </a>
                                                    </td>
                                                </tr>
                                            } => InstructorImageWithLink {
                                                href,
                                                inner: InstructorImage { imgsrc, alt }
                                            };
                                            <tr>
                                                <td class="tbdata" name="instructorTitle">
                                                    instructor
                                                </td>
                                            </tr>
                                        } => (instructor, instructor_image);
                                    </tbody>
                                </table>
                            } => instructors;
                        </div>
                    </form>
                    <script type="text/javascript">
                        _trash
                    </script>
                    <noscript>
                    </noscript>
                </div>
            </div>
        </div>
    }
    let html_handler = footer(html_handler, login_response.id, 311);
    let course_anmeldefristen = course_anmeldefristen.map_or_else(
        Vec::new,
        |anmeldefristen| {
            if anmeldefristen.is_left() { anmeldefristen.unwrap_left() } else { anmeldefristen.unwrap_right() }
        },
    );
    html_handler.end_document();

    let instructors = instructors.unwrap_or_default();
    if dozent.is_none() || dozent == Some("N.N.".to_owned()) {
        assert!(instructors.is_empty());
    } else if h(dozent.as_ref().unwrap()) == "fRArPBELwQcLhe4KzBODOZ7RNkKzNttCYuicWPUNx4w"
        && instructors.iter().map(|m| h(&m.0)).eq([
            "ZhaKKJFX25tOY1kxA60kaVFRXPhnq-2Znq16l9V5acQ",
            "dUAw_-nWeQp2zAi07MFw7M99KQGdgI6QmZMem0wTtgo",
            "o37txCeZ2uWIszeTnl6vocuOugvPMZnSjpKwaHGqfmo",
        ]
        .into_iter())
    {
        // hack, one person has a second name at one place and not at the other place
    } else {
        assert_eq!(dozent.unwrap().split("; ").sorted().collect::<Vec<_>>(), instructors.iter().map(|m| &m.0).sorted().collect::<Vec<_>>());
    }
    assert_eq!(anzeige_im_stundenplan.clone().unwrap_or_default(), shortname.trim());

    let (teilnehmer_range, teilnehmer_min, teilnehmer_max) = teilnehmer.either_into();
    assert_eq!(teilnehmer_range, format!("{teilnehmer_min} | {teilnehmer_max}"));

    let id_and_name: String = id_and_name.either_into();
    let (id, name) = id_and_name.split_once('\n').unwrap();
    let uebungsgruppen = uebungsgruppen.unwrap_or_default();
    let termine: Vec<Termin> = termine.either_into();
    let (termine, termine_kleingruppe): (Vec<Termin>, Vec<Termin>) = if uebungsgruppen.0.is_some() {
        // kleingruppe
        termine.into_iter().partition_map(|mut termin| {
            if termin.date.ends_with('*') {
                termin.date = termin.date.trim_end_matches('*').to_owned();
                Either::Left(termin)
            } else {
                Either::Right(termin)
            }
        })
    } else {
        // plenumsveranstaltung
        (termine, Vec::new())
    };
    Ok(CourseDetailsResponse {
        id: id.trim().to_owned(),
        name: name.trim().to_owned(),
        material_and_messages_url,
        r#type: course_type_and_number.clone().either_into::<(String, String)>().0,
        type_number: course_type_and_number.either_into::<(String, String)>().1.parse().unwrap(),
        fachbereich,
        anzeige_im_stundenplan,
        courselevel: courselevel.parse().unwrap(),
        sws: sws.right().map(|sws| sws.replace(',', ".").parse().expect(&sws)),
        credits: credits.right().map(|credits| credits.trim_end_matches(",0").parse().expect(&credits)),
        language: language_and_id.clone().either_into::<(String, String)>().0,
        language_id: language_and_id.either_into::<(String, String)>().1.parse().unwrap(),
        teilnehmer_min: if teilnehmer_min == "-" { None } else { Some(teilnehmer_min.parse().unwrap()) },
        teilnehmer_max: if teilnehmer_max == "-" { None } else { Some(teilnehmer_max.parse().unwrap()) },
        description,
        uebungsgruppen: uebungsgruppen.1,
        course_anmeldefristen,
        enhalten_in_modulen: enthalten_in_modulen.unwrap_or_default(),
        termine,
        termine_kleingruppe,
        short_termine: short_termine.either_into(),
        instructors,
        plenumsveranstaltung_url: uebungsgruppen.0,
    })
}
