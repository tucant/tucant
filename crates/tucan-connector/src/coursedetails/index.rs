use scraper::{ElementRef, Html};
use tucant_types::{
    LoginResponse, TucanError,
    coursedetails::{CourseAnmeldefrist, CourseDetailsRequest, CourseDetailsResponse, CourseUebungsGruppe},
};

use crate::{
    TucanConnector, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head, logged_out_head},
};
use html_handler::Root;

pub async fn course_details_cached(tucan: &TucanConnector, login_response: &LoginResponse, request: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
    let key = format!("coursedetails.{}", request.arguments.clone());
    if let Some(response) = tucan.database.get(&key).await {
        return Ok(response);
    }

    let response = course_details(tucan, login_response, request).await?;

    tucan.database.put(&key, &response).await;

    Ok(response)
}

pub(crate) async fn course_details(tucan: &TucanConnector, login_response: &LoginResponse, args: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
    let id = login_response.id;
    let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N{:015}{}", id, args.arguments);
    // TODO FIXME generalize
    let key = format!("url.{url}");
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
        <body class="coursedetails">_
    };
    let html_handler = if login_response.id == 1 { logged_out_head(html_handler, 311) } else { logged_in_head(html_handler, login_response.id).0 };
    html_extractor::html! {
        <!--"dqf58hG7HHGpXGyye2_RfFRU9OdHxiBSQr2SeCdraDU"-->_
        <script type="text/javascript">
        </script>_
        <script type="text/javascript">
            _trash
        </script>_
        <form name="courseform" action="/scripts/mgrqispi.dll" method="post">_
            <h1>
                name
            </h1>_
            <div class="contentlayoutleft" id="contentlayoutleft">_
                <table class="tb rw-table rw-all">_
                    <caption>
                        "Veranstaltungsdetails "
                    </caption>_
                    <tbody>
                        <tr>_
                            <td class="tbcontrol" colspan="3">_
    }
    let material_and_messages_url = if html_handler.peek().is_some() {
        // if you are registered for the course
        let material_url;
        let messages_url;
        (html_handler, material_url, messages_url) = {
            html_extractor::html! {
                <a href=material_url class="arrow">
                    "Material"
                </a>_
            }
            html_handler = html_handler.skip_any_comment();
            html_extractor::html! {_
                <a href=messages_url class="arrow">
                    "Nachrichten"
                </a>_
            }
            (html_handler, material_url, messages_url)
        };
        Some((material_url, messages_url))
    } else {
        None
    };
    html_extractor::html! {
            </td>_
        </tr>_
        <tr>_
            <td class="tbdata" colspan="3">_
                <!--"7mR3L45uIzjYs57_yUuqAgGUVvt88EQ1apLxlExwuH4"-->_
                if html_handler.peek().unwrap().first_child().unwrap().value().is_text() {
                    <p>_
                        <b>
                            "Lehrende: "
                        </b>
                        <span id="dozenten">
                            dozent
                        </span>_
                    </p>_
                } => dozent = dozent;
                <p>
                    <b>
                        "Veranstaltungsart:"
                    </b>
                    course_type
                    <input type="hidden" name="coursetyp" value=course_type_number></input>_
                </p>_
                <p>
                    <b>
                        "Orga-Einheit:"
                    </b>_
                    <span name="courseOrgUnit">
                        fachbereich
                    </span>
                </p>_
                <p>_
                    <b>
                        "Anzeige im Stundenplan: "
                    </b>
                    anzeige_im_stundenplan
                    <input type="hidden" name="shortdescription" value=shortname></input>_
                </p>_
                <input type="hidden" name="courselevel" value=courselevel></input>_
                <p>
                    <b>
                        "Fach:"
                    </b>_
                    <input type="hidden" name="coursearea" value=""></input>_
                </p>_
                <p>
                    <b>
                        "Anrechenbar für:"
                    </b>_
                    <input type="hidden" name="creditingfor" value=""></input>_
                </p>_
    }
    let sws;
    (html_handler, sws) = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
        html_extractor::html! {
            <input type="hidden" name="sws" value="0"></input>_
        }
        (html_handler, None)
    } else {
        html_extractor::html! {
            <p>
                <b>
                    "Semesterwochenstunden: "
                </b>
                sws_text
                <input type="hidden" name="sws" value=sws></input>_
            </p>_
        }
        assert_eq!(sws_text.trim(), sws);
        (html_handler, Some(sws))
    };
    let credits;
    (html_handler, credits) = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "input" {
        html_extractor::html! {
            <input type="hidden" name="credits" value="  0,0"></input>_
        }
        (html_handler, None)
    } else {
        html_extractor::html! {
            <p>
                <b>
                    "Credits: "
                </b>
                credits_text
                <input type="hidden" name="credits" value=credits></input>_
            </p>_
        }
        assert_eq!(credits_text, credits);
        (html_handler, Some(credits))
    };
    html_extractor::html! {
        <input type="hidden" name="location" value="327576461398991"></input>_
        <p>
            <b>
                "Unterrichtssprache: "
            </b>_
            <span name="courseLanguageOfInstruction">
                language
            </span>_
            <input type="hidden" name="language" value=language_id></input>_
        </p>_
        <p>
            <b>
                "Min. | Max. Teilnehmerzahl:"
            </b>
            teilnehmer_range
            <input type="hidden" name="min_participantsno" value="-"></input>_
            <input type="hidden" name="max_participantsno" value=teilnehmer_max></input>_
        </p>_
        <!--"u8GEiL8QtgIxvCs-Vf3CkMBYw-XHp4bjwN_4-b3nrOQ"-->_
    }
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
                        <!--"xdnrtl8EoTjGxC3Tn8ZgU7vEsjh7SULK5uyEXMTrPYw"-->_
                    </td>_
                </tr>_
            </tbody>
        </table>_
        if html_handler.peek().unwrap().value().as_comment().unwrap().comment == " KG START ".into() {
            <!--"BJVxG97RSYn0rh25cerEgm9r0KvMqIm48tBzBZmL9fA"-->_
            <div class="tb">_
                <div>_
                    <div class="tbhead">
                        "Kleingruppe(n)"
                    </div>_
                    <div class="tbdata">
                        "\n\t\t\t\tDie Veranstaltung ist in die folgenden Kleingruppen aufgeteilt:\n\t\t\t\t\t\t\t"
                    </div>_
                </div>_
                <ul class="dl-ul-listview">_
                    while html_handler.peek().is_some() {
                        <li class="tbdata listelement">_
                            <div class="dl-inner">_
                                <p class="dl-ul-li-headline">
                                    <strong>
                                        uebung_name
                                    </strong>
                                </p>_
                                <p>
                                    uebungsleiter
                                </p>_
                                <p>
                                    if html_handler.peek().is_some() {
                                        date_range
                                    } => date_range = date_range;
                                </p>_
                            </div>_
                            <div class="dl-link">_
                                <a href=_url class="img img_arrowLeft pageElementRight">
                                    "\n\t\t\t\t\t\t\t\t\tKleingruppe anzeigen\n\t\t\t\t\t\t\t\t"
                                </a>_
                            </div>_
                        </li>_
                    } => uebungsgruppen = CourseUebungsGruppe {
                        date_range,
                        name: uebung_name,
                        uebungsleiter,
                    };
                </ul>_
            </div>_
            <!--"0x4FAGT9tkPZPnjGhLVSIyUwzWJVg5LmPPopzaVekvg"-->_
        } => uebungsgruppen = uebungsgruppen;
        <!--"gjmJkszfvlTVATkzxj9UfHJAWhksvjlPhatwUMepicA"-->_
        <table class="tb rw-table">_
            <caption>
                "\n                        Literatur\n                                        "
            </caption>_
            <tbody>
                <tr>_
                    <td class="tbsubhead">_
                        <span name="literatureCategory">
                            <!--"EdGg5F530M2nVMCHhp1bEr4g_yMTeijq2NDDbwiJXzI"-->
                        </span>_
                    </td>_
                </tr>_
            </tbody>
        </table>_
        <!--"rLgWPHovMo94GGr9fjSOcwUR-V0yqvfB-QchTzSNf04"-->_
        <!--"GwYigtfCarUUFmHd9htM5OAGB7-tTFf7jgzMI1jnYLc"-->_
        if html_handler.peek().unwrap().value().is_element() {
            <table class="tb rw-table">_
                <caption>
                    "Material zur gesamten Veranstaltung"
                </caption>_
                <tbody>
                    <tr>
                        <td class="tbdata" colspan="3">
                            "Es liegt kein Material vor."
                        </td>
                    </tr>_
                </tbody>
            </table>_
        } => unused = ();
        <!--"9hTczu-fkDkzcT9pdtsf0mVFViOxhsg27F08pHvlprA"-->_
        <!--"hcTmLh_Cojhg5bcfJ6dO6SnSw0Z-aNG6pVtxpGhGkK0"-->_
        if html_handler.peek().unwrap().value().is_element() {
            <table class="tb list rw-table">_
                <caption>
                    "Anmeldefristen"
                </caption>_
                <tbody>
                    <tr>_
                        <td class="tbsubhead">
                            " Phase "
                        </td>_
                        <td class="tbsubhead">
                            " Block "
                        </td>_
                        <td class="tbsubhead">
                            " Start "
                        </td>_
                        <td class="tbsubhead">
                            " Ende Anmeldung "
                        </td>_
                        <td class="tbsubhead">
                            " Ende Abmeldung"
                        </td>_
                        <td class="tbsubhead">
                            " Ende Hörer "
                        </td>_
                    </tr>_
                    while html_handler.peek().is_some() {
                        <tr>_
                            <td class="tbdata">
                                zulassungstyp
                            </td>_
                            <td class="tbdata">
                                block_type
                            </td>_
                            <td class="tbdata">
                                start
                            </td>_
                            <td class="tbdata">
                                ende_anmeldung
                            </td>_
                            <td class="tbdata">
                                ende_abmeldung
                            </td>_
                            <td class="tbdata">
                                ende_hoerer
                            </td>_
                        </tr>_
                    } => course_anmeldefristen = CourseAnmeldefrist {
                        zulassungstyp,
                        block_type,
                        start,
                        ende_anmeldung,
                        ende_abmeldung,
                        ende_hoerer
                    };
                </tbody>
            </table>_
        } => course_anmeldefristen = course_anmeldefristen;
        <!--"jqi9g3rkaAfzvYMoNoUy1kaNO-LZHLBDXL8OW4hAioM"-->_
        <!--"y8Y0kF-8a-W4aY1VMRgIGgsP_KmWzGK6jhpfDWop4Wc"-->_
        <table class="tb list rw-table rw-all">_
            <caption>
                "Termine"
            </caption>_
            <tbody>
                <tr class="rw-hide">_
                    <td class="tbsubhead">
                    </td>_
                    <td class="tbsubhead" style="width:120px;">
                        "Datum"
                    </td>_
                    <td class="tbsubhead">
                        "Von"
                    </td>_
                    <td class="tbsubhead">
                        "Bis"
                    </td>_
                    <td class="tbsubhead">
                        "Raum"
                    </td>_
                    <td class="tbsubhead">
                        if html_handler.peek().is_some() {
                            "Lehrende"
                        } => lehrende = ();
                    </td>_
                </tr>_
    };
    if html_handler.peek().unwrap().children().nth(1).unwrap().value().as_element().unwrap().attr("colspan").is_some() {
        html_handler = {
            html_extractor::html! {
                <tr>_
                    <td class="tbdata" colspan="6">
                        "Es liegen keine Termine vor."
                    </td>_
                </tr>_
            }
            html_handler
        }
    } else {
        while html_handler.peek().is_some() {
            html_handler = {
                html_extractor::html! {
                    <tr>_
                        <td class="tbdata rw">
                            id
                        </td>_
                        <td class="tbdata rw rw-course-date" name="appointmentDate">
                            date
                        </td>_
                        <td class="tbdata rw rw-course-from" name="appointmentTimeFrom">
                            time_start
                        </td>_
                        <td class="tbdata rw rw-course-to" name="appointmentDateTo">
                            time_end
                        </td>_
                        <td class="tbdata rw rw-course-room">
                }
                if html_handler.peek().unwrap().value().as_text().unwrap().trim().is_empty() {
                    html_handler = {
                        html_extractor::html! {_
                            if html_handler.peek().is_some() {
                                <a name="appointmentRooms" href=room_url>
                                    room
                                </a>
                                while !html_handler.peek().unwrap().value().as_text().unwrap().trim().is_empty() {
                                    "\n                                                                                                                                                                                                                                                                                                                                                                   ,\u{a0}\n                                                                                                                                                            "
                                    <a name="appointmentRooms" href=room_url>
                                        room
                                    </a>
                                } => teewfwest = ();
                            } => test = ();
                        }
                        html_handler
                    }
                } else {
                    html_handler = {
                        html_extractor::html! {
                            room_text
                        }
                        html_handler
                    }
                }
                html_extractor::html! {
                        </td>_
                        <td class="tbdata rw rw-course-instruct" name="appointmentInstructors">
                            instructors
                        </td>_
                    </tr>_
                }
                html_handler
            }
        }
    }
    html_extractor::html! {
                </tbody>
            </table>_
            <!--"FWVkdRmmQuTMcELIsP6K4V7eWsWq-329gXr8xe8lNtA"-->_
            <!--"Fi8w2ZKNHGT6_59uLcZc14yUPGASGOgkhbLwk5XwAqs"-->_
            <!--"AE7T_bGb3mAQes9i_TGusWvs3SWorP2rUYbWMxtz360"-->_
            <!--"gHS9yEb7gEJDeScOtAZVCap074mrvjNhbKo847wghz0"-->_
            <!--"mbPNYaxxs1wcICUrnywS30UgNmaCxMVGn19JDG2Cdcc"-->_
            <!--"Jr35iwnqHKCxqhgkYtMNg-l-g8g9FFUlmpPW5CyF_3A"-->_
            if login_response.id != 1 {
                <table class="tb rw-table rw-all">_
                    <caption>
                        "Enthalten in Modulen"
                    </caption>_
                    <tbody>
                        <tr>_
                            <td class="tbsubhead">
                                "Modul"
                            </td>_
                        </tr>_
                        while html_handler.peek().is_some() {
                            <tr>_
                                <td class="tbdata">
                                    module_name
                                </td>_
                            </tr>_
                        } => dew = ();
                    </tbody>
                </table>_
            } => agwef = ();
            <!--"ugaD_Kkb-bp5Gg7mdtxXDcj0jeHrTsW_v8Nh9DQBdB0"-->_
            <!--"1ip8eDvrLDhgIPqPeWuUMJdlOaat0QKUTPyfIPoyqBE"-->_
            <!--"9BaxcLXoDbvFC8da2E3MHfCwukHBrtNa5jNlA1FIvws"-->_
            <!--"XyuICPyAWz8IJtSwZZnvBlOVbQJZ2CMAdzVEI3Fg_C4"-->_
            <!--"hq703PrdSGo-1uz3Zu6cI0gGslVvsGwN9EMyJBruQbs"-->_
            <!--"c5-23WihsxAzZQoCFufa3hZA3LPopllavGhaAWJBjrM"-->_
            <!--"LYa-bgADgcQv5a0fIqWl_5bD1B-QfSDS5-Ln5JEn-eQ"-->_
            <!--"zrkedIcKXMmMCMIEbPrxYDCb5ol3FSPWl9OQkbYA-5w"-->_
        </div>_
        <!--"Dy5f5hoTub6F0a3hjk3r6NHBbyjBZKm2Ax1gR8Jn7HQ"-->_
        <!--"rIAsNF0w6-x9uu_FjLOGU1keplSp1eCKVPofF6SN-s0"-->_
        <div class="contentlayoutright" id="contentlayoutright">_
            <div class="tb courseList">_
                <div class="tbhead">
                    "Übersicht der Kurstermine"
                </div>_
                <ul class="courseList">_
    }
    if **html_handler.peek().unwrap().children().next().unwrap().value().as_text().unwrap() == *"Es liegen keine Termine vor." {
        html_handler = {
            html_extractor::html! {
                <li class="courseListCell noLink">
                    "Es liegen keine Termine vor."
                </li>_
            }
            html_handler
        }
    } else {
        while html_handler.peek().is_some() {
            for i in 0..5 {
                if html_handler.peek().unwrap().value().as_element().unwrap().attr("class").unwrap() == "courseListCell numout" {
                    html_handler = {
                        html_extractor::html! {
                            <li class="courseListCell numout" title=title>
                                number
                            </li>_
                            if i == 4 {
                                <!--"i8Po0v92EOSGgcX-6wsqvMrRzAhexv5hS7uSfRxFXQ4"-->_
                            } => wefewfwfef = ();
                        }
                        html_handler
                    }
                } else {
                    html_handler = {
                        html_extractor::html! {
                            <li class="courseListCell noLink">_
                            </li>_
                        }
                        html_handler
                    }
                }
            }
        }
    }
    html_extractor::html! {
                                </ul>_
                            </div>_
                            if html_handler.peek().unwrap().value().is_element() {
                                <table class="tb rw-table">_
                                    <tbody>
                                        <tr class="rw-all">_
                                            <td class="tbhead">
                                                "Lehrende"
                                            </td>_
                                        </tr>_
                                        while html_handler.peek().is_some() {
                                            if html_handler.peek().unwrap().children().nth(1).unwrap().value().as_element().unwrap().attr("name").is_none() {
                                                <tr>_
                                                    <td class="tbdata_nob h_center">_
                                                        <a href=href>_
                                                            <img src=imgsrc width="120" height="160" border="0" alt=alt></img>
                                                        </a>_
                                                    </td>_
                                                </tr>_
                                            } => few = ();
                                            <tr>_
                                                <td class="tbdata" name="instructorTitle">
                                                    instructors
                                                </td>_
                                            </tr>_
                                        } => efw = ();
                                    </tbody>
                                </table>_
                            } => ewf = ();
                            <!--"f3Dd2OExxbOC6O6K52a9HWTpBxipUfPXKU7YBJsuGck"-->_
                        </div>_
                    </form>_
                    <script type="text/javascript">
                        _trash
                    </script>_
                    <noscript>
                    </noscript>_
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    }
    let html_handler = footer(html_handler, login_response.id, 311);
    html_handler.end_document();
    Ok(CourseDetailsResponse {
        name,
        material_and_messages_url,
        dozent,
        r#type: course_type,
        type_number: course_type_number.parse().unwrap(),
        fachbereich,
        anzeige_im_stundenplan,
        shortname,
        courselevel: courselevel.parse().unwrap(),
        sws: sws.map(|sws| sws.parse().unwrap()),
        credits: credits.map(|credits| credits.trim().trim_end_matches(",0").parse().expect(&credits)),
        language,
        language_id: language_id.parse().unwrap(),
        teilnehmer_range,
        teilnehmer_max,
        description,
        uebungsgruppen: uebungsgruppen.unwrap_or_default(),
        course_anmeldefristen: course_anmeldefristen.unwrap_or_default(),
    })
}
