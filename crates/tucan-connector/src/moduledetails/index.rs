use itertools::Itertools;
use scraper::CaseSensitivity::CaseSensitive;
use scraper::{ElementRef, Html};
use tucant_types::InstructorImage;
use tucant_types::moduledetails::{Anmeldefristen, Kurs, KursKategorie, Leistung, Pruefung, Pruefungstermin};
use tucant_types::{
    LoginResponse,
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
};

use crate::{TucanConnector, authenticated_retryable_get};
use crate::{
    TucanError,
    common::head::{footer, html_head, logged_in_head, logged_out_head},
};
use html_handler::Root;

pub async fn module_details_cached(tucan: &TucanConnector, login_response: &LoginResponse, request: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
    let key = format!("moduledetails.{}", request.inner());
    if let Some(response) = tucan.database.get(&key).await {
        return Ok(response);
    }

    let response = module_details(tucan, login_response, request).await?;

    tucan.database.put(&key, &response).await;

    Ok(response)
}

#[expect(clippy::too_many_lines)]
pub async fn module_details(tucan: &TucanConnector, login_response: &LoginResponse, args: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N{:015},-N000311,{}", id, args.inner());
    println!("{url}");
    // TODO FIXME generalize
    let key = format!("unparsed_module_details.{}", args.inner());
    let content = if let Some(content) = tucan.database.get(&key).await {
        content
    } else {
        let content = authenticated_retryable_get(&tucan.client, &url, &login_response.cookie_cnsc).await?;
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
                <body class="moduledetails">_
                    use if login_response.id == 1 { logged_out_head(html_handler, 311) } else { logged_in_head(html_handler, login_response.id).0 };
                    <!--"-h_LWY1o6IWQvq6DnWxWgp2Zp06F4JZitgy9Jh20j3s"-->_
                    <script type="text/javascript">
                    </script>_
                    <h1>
                        module_id
                    </h1>_
                    <form name="moduleform" id="moduleform" action="/scripts/mgrqispi.dll" method="post">_
                        <div class="contentlayoutleft" id="contentlayoutleft">_
                            <table class="tb">_
                                <caption>
                                    "Moduldetails"
                                </caption>_
                                <tbody>
                                    let registered = if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap() == "tbsubhead" {
                                        <tr class="tbsubhead">_
                                            <td colspan="3">
                                                "\n\t\t\t\t\tSie sind angemeldet!\n\t\t\t\t"
                                            </td>_
                                        </tr>_
                                    } => ();
                                    <tr class="tbcontrol">_
                                        <td>_
                                        </td>_
                                    </tr>_
                                    <tr class="tbdata">_
                                        <td colspan="3">_
                                            <b>
                                                "Modulverantwortliche: "
                                            </b>_
                                            <span id="dozenten">
                                                dozenten
                                            </span>_
                                            <br></br>
                                            <br></br>_
                                            <b>
                                                "Anzeige im Stundenplan: "
                                            </b>
                                            display_in_timetable
                                            <br></br>
                                            <br></br>_
                                            <b>
                                                "Dauer: "
                                            </b>
                                            duration
                                            <br></br>
                                            <br></br>_
                                            <b>
                                                "Anzahl Wahlkurse: "
                                            </b>
                                            count_elective_courses
                                            <br></br>
                                            <br></br>_
                                            <b>
                                                "Credits: "
                                            </b>
                                            credits
                                            <br></br>
                                            let abweichende_credits = if html_handler.peek().unwrap().value().is_text() {
                                                "Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.\n                                                             "
                                                <br></br>
                                            } => ();
                                            <br></br>_
                                            <b>
                                                "Startsemester: "
                                            </b>
                                            start_semester
                                            <br></br>
                                            <br></br>_
                                            <!--"ht3ZhEBbY24m_TsTzk888qBQdrwgMawUHy-7WLRZ64E"-->_
                                            let warteliste = if html_handler.peek().unwrap().value().is_element() {
                                                <p>_
                                                    <b>
                                                        "Warteliste:"
                                                    </b>_
                                                    <input type="checkbox" class="checkBox" checked="checked" disabled="disabled"></input>_
                                                </p>_
                                                <p>_
                                                    <b>
                                                        "Wartelistenquote:"
                                                    </b>
                                                    percentage
                                                </p>_
                                            } => ();
                                            <!--"dTJeqGsAPhiwl6lY8BwASSkwEUwc22jswDtjP8U2nwk"-->_
                                            <!--"FAZCaZTDbb4OpO3ZiNhfY9eB8iBPTRyUJmS1mRrUbG4"-->_
                                            let description = while !html_handler.peek().unwrap().value().is_comment() {
                                                let child = html_handler.next_any_child();
                                            } => match child.value() {
                                                scraper::Node::Text(text) => text.trim().to_owned(),
                                                scraper::Node::Element(_element) => ElementRef::wrap(child).unwrap().html(),
                                                _ => panic!(),
                                            };
                                            <!--"QHWpWjdi1Od1UH7a5kQVEbkt567_ZwnRI-Za5HHOrHg"-->_
                                        </td>_
                                    </tr>_
                                </tbody>
                            </table>_
                            <!--"g4GIjAX9XWI8KdgiZYN9CpX0xleUBUwHkZKUxJfi6EQ"-->_
                            <table class="tb rw-table rw-all">_
                                <caption>
                                    "Anmeldefristen "
                                </caption>_
                                <tbody>
                                    <tr class="tbsubhead rw-hide">_
                                        <td>
                                            " Phase "
                                        </td>_
                                        <td>
                                            " Block "
                                        </td>_
                                        <td>
                                            " Anmeldung von | bis "
                                        </td>_
                                        <td>
                                            " Ende Abmeldung"
                                        </td>_
                                    </tr>_
                                    <tr class="tbdata">_
                                        <td class="rw rw-detail-phase">
                                            let anmeldefristen = if **html_handler.peek().unwrap().value().as_text().unwrap() == *" " {_
                                                    <!--"kPjkB9iIB5XqgqsRtfVaZtHvbKDQKU61Hu3gnq6EKAw"-->_
                                                </td>_
                                                <td class="rw rw-detail-block">_
                                                    <!--"uV4w2sL7zvCR7idL5yosP3b9yaa4VOMWjVn7OckmSXA"-->_
                                                </td>_
                                                <td class="rw rw-detail-regstart">_
                                                    <!--"tHYPRHTO0NAcg1WsKTurAev3L2lUda8MaTE3b2IrBDo"-->_
                                                </td>_
                                                <td class="rw rw-detail-unreg">_
                                                    <!--"Eu0RetmnaGYewt3dcmPEOlL9zLLQgN_Qp4HbEiivkLc"-->_
                                            } => () else {
                                                    anmeldeart
                                                </td>_
                                                <td class="rw rw-detail-block">
                                                    " Vorlesungszeit "
                                                </td>_
                                                <td class="rw rw-detail-regstart">
                                                    registration_range
                                                </td>_
                                                <td class="rw rw-detail-unreg">
                                                    unregistration_range
                                            } => Anmeldefristen { registration_range, unregistration_range };
                                        </td>_
                                    </tr>_
                                </tbody>
                            </table>_
                            <!--"_8_RUJ-7SbM4FO6YEtXyjl9DGFNUKS7bRQWuZem55j8"-->_
                            <!--"hytjHG1ygOTxnrK8R8oSrKCt_AYYyEg9yfxJA9JCPA4"-->_
                            let kurskategorien = if html_handler.peek().unwrap().value().is_element() {
                                <table class="tb rw-table rw-all">_
                                    <caption>
                                        "Kurse"
                                    </caption>_
                                    <tbody>
                                        <tr class="tbsubhead rw-hide">_
                                            <td>
                                                <!--"8vHLi99O2SybT1z2ozFMDBJ5m4XT2KjEAoJCxdT0AvY"-->
                                            </td>_
                                            <td>
                                                "Nummer"
                                            </td>_
                                            <td>
                                                "Name"
                                            </td>_
                                            <td>
                                                "Pflicht"
                                            </td>_
                                            <td>
                                                "Semester"
                                            </td>_
                                            <td>
                                                "Credits"
                                            </td>_
                                            <td>_
                                            </td>_
                                        </tr>_
                                        let kurskategorien = while html_handler.peek().is_some() {
                                            <tr class="tbsubhead">_
                                                <td class="rw rw-detail-logo">
                                                    <!--"8vHLi99O2SybT1z2ozFMDBJ5m4XT2KjEAoJCxdT0AvY"-->
                                                </td>_
                                                <td class="rw rw-detail-courseno">
                                                    course_no
                                                </td>_
                                                <td class="rw rw-detail-name">
                                                    name
                                                </td>_
                                                <td class="rw rw-detail-mandatory">
                                                    mandatory
                                                </td>_
                                                <td class="rw rw-detail-semester">
                                                    let semester = if html_handler.peek().is_some() {
                                                        semester
                                                    } => semester;
                                                </td>_
                                                <td class="rw rw-detail-credits">
                                                    credits
                                                </td>_
                                                <td>_
                                                </td>_
                                            </tr>_
                                            let kurse = while html_handler.peek().and_then(|e| e.value().as_element()).map(|e| e.has_class("tbdata", CaseSensitive)) == Some(true) {
                                                <tr class="tbdata">_
                                                    <td class="tbdata">
                                                        <!--"cKueW5TXNZALIFusa3P6ggsr9upFINMVVycC2TDTMY4"-->_
                                                        let gefaehrungspotential_schwangere = if html_handler.peek().is_some() {
                                                            <img src="../../gfx/_default/icons/eventIcon.gif" title="Gefährdungspotential für Schwangere"></img>_
                                                        } => ();
                                                    </td>_
                                                    <td>
                                                        <a name="eventLink" class="link" href=url>
                                                            course_id
                                                        </a>
                                                    </td>_
                                                    <td>
                                                        <a name="eventLink" class="link" href={|v| assert_eq!(v, url)}>
                                                            name
                                                        </a>
                                                    </td>_
                                                    <td>_
                                                    </td>_
                                                    <td>
                                                        <a name="eventLink" class="link" href={|v| assert_eq!(v, url)}>
                                                            semester
                                                        </a>
                                                    </td>_
                                                    <td>_
                                                    </td>_
                                                    <td>_
                                                    </td>_
                                                </tr>_
                                            } => Kurs { name, course_id, gefaehrungspotential_schwangere: gefaehrungspotential_schwangere.is_some(), semester, url };
                                        } => KursKategorie {
                                            course_no,
                                            name,
                                            mandatory: if mandatory.trim() == "Ja" {
                                                true
                                            } else if mandatory.trim() == "Nein" {
                                                false
                                            } else {
                                                panic!("unknown mandatory {mandatory}")
                                            },
                                            semester,
                                            credits,
                                            kurse
                                        };
                                    </tbody>
                                </table>_
                            } => kurskategorien;
                            <!--"XcS-L7xmJsSo5diKeWPZAV2RODpFrumE7AcbFe7AScI"-->_
                            <!--"XmeYv2pdNCa3eVg5mHzpnB67M0-EIs1lMtB2eTrYM6A"-->_
                            <!--"WqHIJmzxI_wd1gXFBYNCiRZr6szuNek-ldCeZFo3R8M"-->_
                            <!--"RbiwK6SpZ7Au8p2XBS1t7LR2XF4kwjqMkFfIEgv-rKc"-->_
                            <!--"WYZJEW9m0LQLxHI4fNLAXyP9Usi68W5DvBNIymfLpa0"-->_
                            <!--"RiWbv8Xb_X5unLSu-h2dOXvsMSfM9vnOkC0FzKSUbIY"-->_
                            <!--"IM7UyQ8J2Prc4k7ngbYVxLKq5_3M-nyvLH65J72ju_c"-->_
                            <!--"G_ubVsxEEhlOjm-QIAX4HfC7IIP5TBrEBFAo95WO3GM"-->_
                            <table class="tb rw-table rw-all" summary="Leistungen">_
                                <caption>
                                    "Leistungen"
                                </caption>_
                                <thead>_
                                    <tr class="tbsubhead rw-hide">_
                                        <th scope="col">
                                            "Kurs/Modulabschlussleistungen"
                                        </th>_
                                        <th scope="col">
                                            let leistungskombination = if **html_handler.peek().unwrap().value().as_text().unwrap() == *"Leistungskombination" {
                                                    "Leistungskombination"
                                                </th>_
                                                <th scope="col">
                                            } => ();
                                            "Leistungen"
                                        </th>_
                                        <th scope="col">
                                            "Bestehenspflicht"
                                        </th>_
                                        <th scope="col">
                                            "Gewichtung"
                                        </th>_
                                    </tr>_
                                </thead>_
                                <tbody>_
                                    let leistungen = while html_handler.peek().is_some() {
                                        <!--"Q978vY9eIUQSe-WWhOD-KiCLuTJDGO6f_xVROPE7soI"-->_
                                        <tr>_
                                            <td rowspan=rowspan class="tbsubhead level02_color ">
                                                modulabschlussleistungen_or_module_name
                                            </td>_
                                            extern {
                                                let mut rowspan: u64 = rowspan.parse().unwrap();
                                            }
                                            let leistungen = if leistungskombination.is_some() {
                                                    <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                                                    <td rowspan="0002" class="level03_color tbborderleft">_
                                                        <b>
                                                            name
                                                        </b>_
                                                    </td>_
                                                    <td colspan="2" class="level03_color alignRight">
                                                        <b>
                                                            "Summe"
                                                        </b>
                                                    </td>_
                                                    <td colspan="1" class="level03_color alignRight rw-detail-weight">
                                                        <b>
                                                            weight
                                                        </b>
                                                    </td>_
                                                </tr>_
                                                <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                <tr class="tbdata">_
                                                    <td class="tbborderleft rw rw-detail-reqachieve">
                                                        {|v: String| assert_eq!(name.trim(), v.trim())}
                                                    </td>_
                                                    <td class="rw rw-detail-compulsory">
                                                        compulsory
                                                    </td>_
                                                    <td class="rw rw-detail-weight alignRight">
                                                        {|v: String| assert_eq!(weight.trim(), v.trim())}
                                                    </td>_
                                                </tr>_
                                                let leistungen = while rowspan > 2 {
                                                    <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                                                    <tr>_
                                                        <td rowspan="0002" class="level03_color tbborderleft">_
                                                            <b>
                                                                name
                                                            </b>_
                                                        </td>_
                                                        <td colspan="2" class="level03_color alignRight">
                                                            <b>
                                                                "Summe"
                                                            </b>
                                                        </td>_
                                                        <td colspan="1" class="level03_color alignRight rw-detail-weight">
                                                            <b>
                                                                weight
                                                            </b>
                                                        </td>_
                                                    </tr>_
                                                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                    <tr class="tbdata">_
                                                        <td class="tbborderleft rw rw-detail-reqachieve">
                                                            {|v: String| assert_eq!(name.trim(), v.trim())}
                                                        </td>_
                                                        <td class="rw rw-detail-compulsory">
                                                            compulsory
                                                        </td>_
                                                        <td class="rw rw-detail-weight alignRight">
                                                            {|v: String| assert_eq!(weight.trim(), v.trim())}
                                                            let weight_more = if html_handler.peek().is_some() {
                                                                <br></br>
                                                                weight_more
                                                            } => weight_more;
                                                        </td>_
                                                    </tr>_
                                                } => {
                                                    rowspan -= 2;
                                                    Leistung {
                                                        name: name.trim().to_owned(),
                                                        weight,
                                                        compulsory: if compulsory.trim() == "Ja" {
                                                            true
                                                        } else if compulsory.trim() == "Nein" {
                                                            false
                                                        } else {
                                                            panic!("unknown compulsory {compulsory}")
                                                        },
                                                        weight_more,
                                                    }
                                                };
                                            } => {
                                                leistungen.insert(
                                                    0,
                                                    Leistung {
                                                        name: name.trim().to_owned(),
                                                        weight,
                                                        compulsory: if compulsory.trim() == "Ja" {
                                                            true
                                                        } else if compulsory.trim() == "Nein" {
                                                            false
                                                        } else {
                                                            panic!("unknown compulsory {compulsory}")
                                                        },
                                                        weight_more: None,
                                                    },
                                                );
                                                leistungen
                                            } else {
                                                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                    <td class="tbborderleft rw rw-detail-reqachieve">
                                                        name
                                                    </td>_
                                                    <td class="rw rw-detail-compulsory">
                                                        compulsory
                                                    </td>_
                                                    <td class="rw rw-detail-weight alignRight">
                                                        weight
                                                    </td>_
                                                </tr>_
                                                let leistungen = while rowspan > 1 {
                                                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                    <tr class="tbdata">_
                                                        <td class="tbborderleft rw rw-detail-reqachieve">
                                                            name
                                                        </td>_
                                                        <td class="rw rw-detail-compulsory">
                                                            compulsory
                                                        </td>_
                                                        <td class="rw rw-detail-weight alignRight">
                                                            weight
                                                        </td>_
                                                    </tr>_
                                                } => {
                                                    rowspan -= 1;
                                                    Leistung {
                                                        name: name.trim().to_owned(),
                                                        weight,
                                                        compulsory: if compulsory.trim() == "Ja" {
                                                            true
                                                        } else if compulsory.trim() == "Nein" {
                                                            false
                                                        } else {
                                                            panic!("unknown compulsory {compulsory}")
                                                        },
                                                        weight_more: None,
                                                    }
                                                };
                                            } => {
                                                leistungen.insert(
                                                    0,
                                                    Leistung {
                                                        name: name.trim().to_owned(),
                                                        weight,
                                                        compulsory: if compulsory.trim() == "Ja" {
                                                            true
                                                        } else if compulsory.trim() == "Nein" {
                                                            false
                                                        } else {
                                                            panic!("unknown compulsory {compulsory}")
                                                        },
                                                        weight_more: None,
                                                    },
                                                );
                                                leistungen
                                            };
                                    } => leistungen.either_into::<Vec<Leistung>>();
                                </tbody>_
                            </table>_
                            <!--"2ZbUIAyW1jo5-WUMeTNt-IKv23wZ26ul3DgqOFYk-Cs"-->_
                            <!--"yzI2g2lOkYEZ9daP_HPMEVsNji03iv9OjslJBotOfZ0"-->_
                            let pruefungen = if !html_handler.peek().unwrap().value().is_comment() {
                                <table class="tb rw-table rw-all" summary="Modulabschlussprüfungen">_
                                    <caption>
                                        "Modulabschlussprüfungen"
                                    </caption>_
                                    <thead>_
                                        <tr class="tbsubhead rw-hide">_
                                            <th scope="col">
                                                let leistungskombination = if **html_handler.peek().unwrap().value().as_text().unwrap() == *"Leistungskombination" {
                                                        "Leistungskombination"
                                                    </th>_
                                                    <th scope="col">
                                                } => ();
                                                "Prüfung"
                                            </th>_
                                            <th scope="col">
                                                "Datum"
                                            </th>_
                                            <th scope="col">
                                                "Lehrende"
                                            </th>_
                                            <th scope="col">
                                                "Bestehenspflicht"
                                            </th>_
                                        </tr>_
                                    </thead>_
                                    <tbody>_
                                        let pruefungen = while html_handler.peek().is_some() {
                                            let pruefung = if leistungskombination.is_some() {
                                                <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                                                <tr class="tbdata">_
                                                    <td rowspan=rowspan class="level03_color rw rw-detail-combination ">_
                                                        <b>
                                                            name
                                                        </b>_
                                                    </td>_
                                                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                    <td class="tbborderleft rw rw-detail-exam">
                                                        subname
                                                    </td>_
                                                    <td class="rw rw-detail-date">
                                                        date
                                                    </td>_
                                                    <td class="rw rw-detail-instructors">
                                                        examiner
                                                    </td>_
                                                    <td class="rw rw-detail-compulsory">
                                                        compulsory
                                                    </td>_
                                                </tr>_
                                                extern {
                                                    let mut rowspan: u64 = rowspan.parse().unwrap();
                                                }
                                                let termine = while rowspan > 1 {
                                                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                    <tr class="tbdata">_
                                                        <td class="tbborderleft rw rw-detail-exam">
                                                            subname
                                                        </td>_
                                                        <td class="rw rw-detail-date">
                                                            date
                                                        </td>_
                                                        <td class="rw rw-detail-instructors">
                                                            examiner
                                                        </td>_
                                                        <td class="rw rw-detail-compulsory">
                                                            {|v: String| assert_eq!(compulsory.trim(), v.trim())}
                                                        </td>_
                                                    </tr>_
                                                } => {
                                                    rowspan -= 1;
                                                    Pruefungstermin { date, examiner, subname }
                                                };
                                            } => {
                                                termine.insert(0, Pruefungstermin { date, examiner, subname });
                                                Pruefung {
                                                    compulsory: if compulsory.trim() == "Ja" {
                                                        true
                                                    } else if compulsory.trim() == "Nein" {
                                                        false
                                                    } else {
                                                        panic!("unknown compulsory {compulsory}")
                                                    },
                                                    name,
                                                    termine,
                                                }
                                            } else {
                                                <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                                <tr class="tbdata">_
                                                    <td class="tbborderleft rw rw-detail-exam">
                                                        name
                                                    </td>_
                                                    <td class="rw rw-detail-date">
                                                        date
                                                    </td>_
                                                    <td class="rw rw-detail-instructors">
                                                        examiner
                                                    </td>_
                                                    <td class="rw rw-detail-compulsory">
                                                        compulsory
                                                    </td>_
                                                </tr>_
                                            } => Pruefung {
                                                name: name.clone(),
                                                compulsory: if compulsory.trim() == "Ja" {
                                                    true
                                                } else if compulsory.trim() == "Nein" {
                                                    false
                                                } else {
                                                    panic!("unknown compulsory {compulsory}")
                                                },
                                                termine: vec![Pruefungstermin { date, examiner, subname: name }]
                                            };
                                        } => pruefung.either_into();
                                    </tbody>_
                                </table>_
                            } => pruefungen;
                            <!--"uhyYYbUSVjP7_XQEDDQOad7J3GgMGl4q_WFqXNEWGOA"-->_
                        </div>_
                        <!--"Dy5f5hoTub6F0a3hjk3r6NHBbyjBZKm2Ax1gR8Jn7HQ"-->_
                        <div class="contentlayoutright" id="contentlayoutright">_
                            let modulverantwortliche = if html_handler.peek().is_some() {
                                <table class="tb_contentright">_
                                    <caption>
                                        "Modulverantwortliche"
                                    </caption>_
                                    <tbody>
                                        let modulverantwortliche = while html_handler.peek().is_some() {
                                            let bild = if html_handler.peek().unwrap().value().as_element().unwrap().attrs.is_empty() {
                                                <tr>_
                                                    <td class="tbdata_nob" style="text-align:center;padding-top:10px;padding-left:0px;">_
                                                        <img src=imgsrc width="120" height="160" border="0" alt=alt></img>_
                                                    </td>_
                                                </tr>_
                                            } => InstructorImage { alt, imgsrc };
                                            <tr class="tbdata">_
                                                <td style="text-align:center;">
                                                    name
                                                </td>_
                                            </tr>_
                                        } => (name, bild);
                                    </tbody>
                                </table>_
                            } => modulverantwortliche;
                        </div>_
                        <!--"SzJAJfnnubn5SpplE3qoUsG2QoqW6EEMiB36flFP3BQ"-->_
                        <br style="clear:both;"></br>_
                    </form>_
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    };
    // TODO pass value depending on module details url or maybe normalize 275
    let html_handler = footer(html_handler, id, 311);

    let modulverantwortliche = modulverantwortliche.unwrap_or_default();
    if modulverantwortliche.is_empty() {
        assert_eq!(dozenten, "N.N.");
    } else {
        assert_eq!(dozenten.split("; ").sorted().collect::<Vec<_>>(), modulverantwortliche.iter().map(|m| &m.0).sorted().collect::<Vec<_>>());
    }
    Ok(ModuleDetailsResponse {
        module_id,
        registered: registered.is_some(),
        count_elective_courses,
        credits,
        description,
        display_in_timetable,
        duration,
        abweichende_credits: abweichende_credits.is_some(),
        start_semester,
        anmeldefristen: anmeldefristen.right(),
        kurskategorien: kurskategorien.unwrap_or_default(),
        modulverantwortliche,
        leistungen: leistungen.into_iter().flatten().collect(),
        pruefungen: pruefungen.unwrap_or_default(),
    })
}
