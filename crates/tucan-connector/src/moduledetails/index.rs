use scraper::CaseSensitivity::CaseSensitive;
use scraper::{ElementRef, Html};
use tucant_types::{
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    LoginResponse,
};

use crate::TucanConnector;
use crate::{
    common::head::{footer, html_head, logged_in_head, logged_out_head},
    html_handler::Root,
    TucanError,
};

pub async fn module_details_cached(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    request: ModuleDetailsRequest,
) -> Result<ModuleDetailsResponse, TucanError> {
    let key = format!("moduledetails.{}", request.arguments.clone());
    if let Some(response) = tucan.database.get(&key).await {
        return Ok(response);
    }

    let response = module_details(tucan, login_response, request).await?;

    tucan.database.put(&key, &response).await;

    Ok(response)
}

pub async fn module_details(
    tucan: &TucanConnector,
    login_response: &LoginResponse,
    args: ModuleDetailsRequest,
) -> Result<ModuleDetailsResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N{:015}{}", id, args.arguments);
    println!("{url}");
    // TODO FIXME generalize
    let key = format!("url.{url}");
    let content = if let Some(content) = tucan.database.get(&key).await {
        content
    } else {
        let response = tucan
            .client
            .get(url)
            .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
            .send()
            .await?
            .error_for_status()?;
        let content = response.text().await?;
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
    };
    let html_handler = html_head(html_handler)?;
    html_extractor::html! {
            <style type="text/css">
                "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
            </style>_
            <style type="text/css">
                "3CC0xpJgjHprYY59D1krvfwrI2LSV2-OtaN3CviYnG8"
            </style>_
        </head>_
        <body class="moduledetails">_
    };
    let html_handler = if login_response.id == 1 {
        logged_out_head(html_handler, 311)
    } else {
        logged_in_head(html_handler, login_response.id).0
    };
    html_extractor::html! {
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
    };

    let (html_handler, registered) = if html_handler
        .peek()
        .unwrap()
        .value()
        .as_element()
        .unwrap()
        .attr("class")
        .unwrap()
        == "tbsubhead"
    {
        html_extractor::html! {
            <tr class="tbsubhead">_
                <td colspan="3">
                    "\n\t\t\t\t\tSie sind angemeldet!\n\t\t\t\t"
                </td>_
            </tr>_
        };
        (html_handler, true)
    } else {
        (html_handler, false)
    };
    html_extractor::html! {
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
                length
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
    };
    let html_handler = if html_handler.peek().unwrap().value().is_text() {
        html_extractor::html! {
            "Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.\n                                                             "
            <br></br>
        };
        html_handler
    } else {
        html_handler
    };
    html_extractor::html! {
        <br></br>_
        <b>
            "Startsemester: "
        </b>
        start_semester
        <br></br>
        <br></br>_
        <!--"ht3ZhEBbY24m_TsTzk888qBQdrwgMawUHy-7WLRZ64E"-->_
        <!--"dTJeqGsAPhiwl6lY8BwASSkwEUwc22jswDtjP8U2nwk"-->_
        <!--"FAZCaZTDbb4OpO3ZiNhfY9eB8iBPTRyUJmS1mRrUbG4"-->_
    };
    let mut description = Vec::new();
    while !html_handler.peek().unwrap().value().is_comment() {
        let child;
        (html_handler, child) = html_handler.next_any_child();
        match child.value() {
            scraper::Node::Text(text) => description.push(text.trim().to_owned()),
            scraper::Node::Element(_element) => {
                description.push(ElementRef::wrap(child).unwrap().html());
            }
            _ => panic!(),
        }
    }
    html_extractor::html! {
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
    }
    let html_handler = if **html_handler.peek().unwrap().value().as_text().unwrap() == *" " {
        html_extractor::html! {_
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
        }
        html_handler
    } else {
        html_extractor::html! {
                " Direkte Zulassung "
            </td>_
            <td class="rw rw-detail-block">
                " Vorlesungszeit "
            </td>_
            <td class="rw rw-detail-regstart">
                registration_range
            </td>_
            <td class="rw rw-detail-unreg">
                unregistration
        }
        html_handler
    };
    html_extractor::html! {
                    </td>_
                </tr>_
            </tbody>
        </table>_
        <!--"_8_RUJ-7SbM4FO6YEtXyjl9DGFNUKS7bRQWuZem55j8"-->_
        <!--"hytjHG1ygOTxnrK8R8oSrKCt_AYYyEg9yfxJA9JCPA4"-->_
    };
    let html_handler = if html_handler.peek().unwrap().value().is_element() {
        html_extractor::html! {
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
        };
        while html_handler.peek().is_some() {
            html_handler = {
                html_extractor::html! {
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
                };
                let html_handler = if html_handler.peek().is_some() {
                    html_extractor::html! {
                        semester
                    };
                    html_handler
                } else {
                    html_handler
                };

                html_extractor::html! {
                        </td>_
                        <td class="rw rw-detail-credits">
                            _credits
                        </td>_
                        <td>_
                        </td>_
                    </tr>_
                };
                while html_handler
                    .peek()
                    .and_then(|e| e.value().as_element())
                    .map(|e| e.has_class("tbdata", CaseSensitive))
                    == Some(true)
                {
                    html_handler = {
                        html_extractor::html! {
                            <tr class="tbdata">_
                                <td class="tbdata">
                                    <!--"cKueW5TXNZALIFusa3P6ggsr9upFINMVVycC2TDTMY4"-->_
                        };
                        let html_handler = if html_handler.peek().is_some() {
                            html_extractor::html! {
                                <img src="../../gfx/_default/icons/eventIcon.gif" title="Gefährdungspotential für Schwangere"></img>_
                            }
                            html_handler
                        } else {
                            html_handler
                        };
                        html_extractor::html! {
                                </td>_
                                <td>
                                    <a name="eventLink" class="link" href=course_url_1>
                                        course_no
                                    </a>
                                </td>_
                                <td>
                                    <a name="eventLink" class="link" href=course_url_1>
                                        name
                                    </a>
                                </td>_
                                <td>_
                                </td>_
                                <td>
                                    <a name="eventLink" class="link" href=course_url_1>
                                        semester
                                    </a>
                                </td>_
                                <td>_
                                </td>_
                                <td>_
                                </td>_
                            </tr>_
                        };
                        html_handler
                    }
                }
                html_handler
            }
        }
        html_extractor::html! {
                </tbody>
            </table>_
        };
        html_handler
    } else {
        html_handler
    };
    // either Leistungen or Leistungskombination
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N675523572713350,-N000311,-N390004723934573
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N675523572713350,-N000311,-N389455489906019
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N391325656494429
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N390344407743131
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N390293258177073
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N389304870207425
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N390274491911206
    // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N383725573139210,-N000311,-N390836907216477
    html_extractor::html! {
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
    };
    let (html_handler, leistungskombination) =
        if **html_handler.peek().unwrap().value().as_text().unwrap() == *"Leistungskombination" {
            html_extractor::html! {
                    "Leistungskombination"
                </th>_
                <th scope="col">
            }
            (html_handler, true)
        } else {
            (html_handler, false)
        };
    html_extractor::html! {
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
    }
    while html_handler.peek().is_some() {
        html_handler = {
            html_extractor::html! {
                <!--"Q978vY9eIUQSe-WWhOD-KiCLuTJDGO6f_xVROPE7soI"-->_
                <tr>_
                    <td rowspan=rowspan class="tbsubhead level02_color ">
                        modulabschlussleistungen_or_module_name
                    </td>_
            }

            // this part is almost repeated but not exactly
            let html_handler = if leistungskombination {
                html_extractor::html! {
                        <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                        <td rowspan="0002" class="level03_color tbborderleft">_
                            <b>
                                exam_type
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
                }
                html_handler
            } else {
                html_extractor::html! {
                    <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                }
                html_handler
            };
            html_extractor::html! {
                    <td class="tbborderleft rw rw-detail-reqachieve">
                        examination_type
                    </td>_
                    <td class="rw rw-detail-compulsory">
                        compulsory
                    </td>_
                    <td class="rw rw-detail-weight alignRight">
                        weight
                    </td>_
                </tr>_
            };
            let mut rowspan: u64 = rowspan.parse().unwrap();
            rowspan -= 1;
            // TODO FIXME count using rowspan here
            while !leistungskombination && rowspan > 0 {
                html_handler = {
                    html_extractor::html! {
                        <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                        <tr class="tbdata">_
                            <td class="tbborderleft rw rw-detail-reqachieve">
                                examination_type
                            </td>_
                            <td class="rw rw-detail-compulsory">
                                compulsory
                            </td>_
                            <td class="rw rw-detail-weight alignRight">
                                weight
                            </td>_
                        </tr>_
                    }
                    html_handler
                };
                rowspan -= 1;
            }

            while leistungskombination && html_handler.peek().is_some() {
                html_handler = {
                    html_extractor::html! {
                        <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                        <tr>_
                    };
                    let html_handler = if leistungskombination {
                        html_extractor::html! {
                                <td rowspan="0002" class="level03_color tbborderleft">_
                                    <b>
                                        exam_type
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
                        }
                        html_handler
                    } else {
                        html_handler
                    };
                    html_extractor::html! {
                        <td class="tbborderleft rw rw-detail-reqachieve">
                            examination_type
                        </td>_
                        <td class="rw rw-detail-compulsory">
                            "\tJa"
                        </td>_
                        <td class="rw rw-detail-weight alignRight">
                            weight
                    }
                    let html_handler = if html_handler.peek().is_some() {
                        html_extractor::html! {
                            <br></br>
                            weight_more
                        }
                        html_handler
                    } else {
                        html_handler
                    };
                    html_extractor::html! {
                            </td>_
                        </tr>_
                    };
                    html_handler
                };
            }
            html_handler
        }
    }
    html_extractor::html! {
            </tbody>_
        </table>_
        <!--"2ZbUIAyW1jo5-WUMeTNt-IKv23wZ26ul3DgqOFYk-Cs"-->_
        <!--"yzI2g2lOkYEZ9daP_HPMEVsNji03iv9OjslJBotOfZ0"-->_
    }
    // here
    let html_handler = if html_handler.peek().unwrap().value().is_comment() {
        html_handler
    } else {
        html_extractor::html! {
            <table class="tb rw-table rw-all" summary="Modulabschlussprüfungen">_
                <caption>
                    "Modulabschlussprüfungen"
                </caption>_
                <thead>_
                    <tr class="tbsubhead rw-hide">_
                        <th scope="col">
        };

        let (html_handler, leistungskombination) =
            if **html_handler.peek().unwrap().value().as_text().unwrap() == *"Leistungskombination"
            {
                html_extractor::html! {
                        "Leistungskombination"
                    </th>_
                    <th scope="col">
                };
                (html_handler, true)
            } else {
                (html_handler, false)
            };
        html_extractor::html! {
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
        };
        while html_handler.peek().is_some() {
            html_handler = {
                let html_handler = if leistungskombination {
                    html_extractor::html! {
                        <!--"m9kKtyJq8n6Nc3k3DA46XI-06Jmq77IMLKAgoMJn5zE"-->_
                        <tr class="tbdata">_
                            <td rowspan=rowspan class="level03_color rw rw-detail-combination ">_
                                <b>
                                    Fachprüfung
                                </b>_
                            </td>_
                            <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                            <td class="tbborderleft rw rw-detail-exam">
                                exam_type
                            </td>_
                            <td class="rw rw-detail-date">
                                exam_date
                            </td>_
                            <td class="rw rw-detail-instructors">
                                instructor
                            </td>_
                            <td class="rw rw-detail-compulsory">
                                compulsory
                            </td>_
                        </tr>_
                    };
                    let mut rowspan: u64 = rowspan.parse().unwrap();
                    rowspan -= 1;
                    while rowspan > 0 {
                        html_handler = {
                            html_extractor::html! {
                                <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                                <tr class="tbdata">_
                                    <td class="tbborderleft rw rw-detail-exam">
                                        exam_type
                                    </td>_
                                    <td class="rw rw-detail-date">
                                        exam_date
                                    </td>_
                                    <td class="rw rw-detail-instructors">
                                        instructor
                                    </td>_
                                    <td class="rw rw-detail-compulsory">
                                        compulsory
                                    </td>_
                                </tr>_
                            };
                            html_handler
                        };
                        rowspan -= 1;
                    }

                    html_handler
                } else {
                    html_extractor::html! {
                        <!--"wZPrppUHfMMSm1oo3-4LsQWn8863dt2JZSJPupEG9Oo"-->_
                        <tr class="tbdata">_
                            <td class="tbborderleft rw rw-detail-exam">
                                exam_type
                            </td>_
                            <td class="rw rw-detail-date">
                                exam_date
                            </td>_
                            <td class="rw rw-detail-instructors">
                                instructor
                            </td>_
                            <td class="rw rw-detail-compulsory">
                                compulsory
                            </td>_
                        </tr>_
                    };
                    html_handler
                };
                html_handler
            }
        }
        html_extractor::html! {
                </tbody>_
            </table>_
        };
        html_handler
    };
    // until here
    html_extractor::html! {
            <!--"uhyYYbUSVjP7_XQEDDQOad7J3GgMGl4q_WFqXNEWGOA"-->_
        </div>_
        <!--"Dy5f5hoTub6F0a3hjk3r6NHBbyjBZKm2Ax1gR8Jn7HQ"-->_
        <div class="contentlayoutright" id="contentlayoutright">_
    };
    let html_handler = if html_handler.peek().is_some() {
        html_extractor::html! {
            <table class="tb_contentright">_
                <caption>
                    "Modulverantwortliche"
                </caption>_
                <tbody>
        };
        while html_handler.peek().is_some() {
            html_handler = {
                let html_handler = if html_handler
                    .peek()
                    .unwrap()
                    .value()
                    .as_element()
                    .unwrap()
                    .attrs
                    .is_empty()
                {
                    html_extractor::html! {
                        <tr>_
                            <td class="tbdata_nob" style="text-align:center;padding-top:10px;padding-left:0px;">_
                                <img src=_src width="120" height="160" border="0" alt=_alt></img>_
                            </td>_
                        </tr>_
                    }
                    html_handler
                } else {
                    html_handler
                };
                html_extractor::html! {
                    <tr class="tbdata">_
                        <td style="text-align:center;">
                            name
                        </td>_
                    </tr>_
                };
                html_handler
            };
        }
        html_extractor::html! {
                </tbody>
            </table>_
        }
        html_handler
    } else {
        html_handler
    };
    html_extractor::html! {
                        </div>_
                        <!--"SzJAJfnnubn5SpplE3qoUsG2QoqW6EEMiB36flFP3BQ"-->_
                        <br style="clear:both;"></br>_
                    </form>_
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    };
    let html_handler = footer(html_handler, id, 311);

    Ok(ModuleDetailsResponse {
        module_id,
        registered,
        count_elective_courses,
        credits,
        description,
        display_in_timetable,
        dozenten,
        duration: length,
    })
}
