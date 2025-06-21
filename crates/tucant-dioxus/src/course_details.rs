use crate::{ Route, common::use_authenticated_data_loader};
use tucant_types::{Tucan, coursedetails::CourseDetailsRequest};
use dioxus::prelude::*;


#[component]
pub fn CourseDetails(course_details: CourseDetailsRequest) -> Element {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.course_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, course_details.to_owned(), 14 * 24 * 60 * 60, 60 * 60, |course, reload| {
        ::yew::html! {
            div {
                h1 {
                    { &course.name }
                    if let Some(credits) = course.credits {
                        { " " }
                        span { class: "badge text-bg-secondary",
                            { format!("{} CP", credits) }
                        }
                    }
                    { " " }
                    button { onclick: reload type: "button" class: "btn btn-light",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg { xmlns: "http://www.w3.org/2000/svg" width: "16" height: "16" fill: "currentColor" class: "bi bi-arrow-clockwise" viewBox: "0 0 16 16",
                            path { fill-rule: "evenodd" d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                if !course.instructors.is_empty() {
                    h2 {
                        { "Lehrende" }
                    }
                    ul {
                        {
                            course
                                .instructors
                                .iter()
                                .map(|instructor| {
                                    ::yew::html! {
                                        li {
                                            { &instructor.0 }
                                        }
                                    }
                                })
                                .collect::<Html>()
                        }
                    }
                }
                div {
                    { format!("Typ: {}", course.r#type) }
                }
                div {
                    { format!("Fachbereich: {}", course.fachbereich) }
                }
                {
                    match (course.teilnehmer_min, course.teilnehmer_max) {
                        (None, None) => yew::html! {},
                        (None, Some(max)) => ::yew::html! {
                            div {
                                { format!("Maximal {max} Teilnehmende") }
                            }
                        },
                        (Some(min), None) => ::yew::html! {
                            div {
                                { format!("Mindestens {min} Teilnehmende",) }
                            }
                        },
                        (Some(min), Some(max)) => ::yew::html! {
                            div {
                                { format!("{min} - {max} Teilnehmende",) }
                            }
                        },
                    }
                }
                h2 {
                    { "Übungsgruppen" }
                }
                table { class: "table",
                    thead {
                        tr {
                            th { scope: "col",
                                { "Name" }
                            }
                            th { scope: "col",
                                { "Zeitraum" }
                            }
                            th { scope: "col",
                                { "Uebungsleitende" }
                            }
                        }
                    }
                    tbody {
                        if let Some(plenumsveranstaltung) = course.plenumsveranstaltung_url {
                            tr {
                                th { scope: "row",
                                    Link { to: Route::CourseDetails { course: plenumsveranstaltung.clone() },
                                        { "Plenumsveranstaltung" }
                                    }
                                }
                                td {
                                }
                                td {
                                }
                            }
                        }
                        {
                            course
                                .uebungsgruppen
                                .iter()
                                .map(|uebungsgruppe| {
                                    ::yew::html! {
                                        tr { class: if uebungsgruppe.active { "table-primary" } else { "" },
                                            th { scope: "row",
                                                Link { to: Route::CourseDetails { course: uebungsgruppe.url.clone() },
                                                    { &uebungsgruppe.name }
                                                }
                                            }
                                            td {
                                                { uebungsgruppe.date_range.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { &uebungsgruppe.uebungsleiter }
                                            }
                                        }
                                    }
                                })
                                .collect::<Html>()
                        }
                    }
                }
                h2 {
                    { "Anmeldefristen" }
                }
                table { class: "table",
                    thead {
                        tr {
                            th { scope: "col",
                                { "Phase" }
                            }
                            th { scope: "col",
                                { "Block" }
                            }
                            th { scope: "col",
                                { "Start" }
                            }
                            th { scope: "col",
                                { "Ende Anmeldung" }
                            }
                            th { scope: "col",
                                { "Ende Abmeldung" }
                            }
                            th { scope: "col",
                                { "Ende Hörer" }
                            }
                        }
                    }
                    tbody {
                        {
                            course
                                .course_anmeldefristen
                                .iter()
                                .map(|anmeldefrist| {
                                    ::yew::html! {
                                        tr {
                                            td {
                                                { &anmeldefrist.zulassungstyp }
                                            }
                                            td {
                                                { &anmeldefrist.block_type }
                                            }
                                            td {
                                                { anmeldefrist.start.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { &anmeldefrist.ende_anmeldung.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { &anmeldefrist.ende_abmeldung.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { &anmeldefrist.ende_hoerer.clone().unwrap_or_default() }
                                            }
                                        }
                                    }
                                })
                                .collect::<Html>()
                        }
                    }
                }
                if !course.termine_kleingruppe.is_empty() {
                    h2 {
                        { "Termine Kleingruppe" }
                    }
                    table { class: "table",
                        thead {
                            tr {
                                th { scope: "col",
                                    { "Datum" }
                                }
                                th { scope: "col",
                                    { "Start" }
                                }
                                th { scope: "col",
                                    { "Ende" }
                                }
                                th { scope: "col",
                                    { "Kursleitende" }
                                }
                                th { scope: "col",
                                    { "Räume" }
                                }
                            }
                        }
                        tbody {
                            {
                                course
                                    .termine_kleingruppe
                                    .iter()
                                    .map(|termin| {
                                        ::yew::html! {
                                            tr {
                                                td {
                                                    { &termin.date }
                                                }
                                                td {
                                                    { &termin.time_start }
                                                }
                                                td {
                                                    { &termin.time_end }
                                                }
                                                td {
                                                    { &termin.instructors.clone().unwrap_or_default() }
                                                }
                                                td {
                                                    ul {
                                                        {
                                                            termin
                                                                .rooms
                                                                .iter()
                                                                .map(|room| {
                                                                    ::yew::html! {
                                                                        li {
                                                                            { &room.name }
                                                                        }
                                                                    }
                                                                })
                                                                .collect::<Html>()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    })
                                    .collect::<Html>()
                            }
                        }
                    }
                }
                h2 {
                    { "Termine Plenumsveranstaltung" }
                }
                table { class: "table",
                    thead {
                        tr {
                            th { scope: "col",
                                { "Datum" }
                            }
                            th { scope: "col",
                                { "Start" }
                            }
                            th { scope: "col",
                                { "Ende" }
                            }
                            th { scope: "col",
                                { "Kursleitende" }
                            }
                            th { scope: "col",
                                { "Räume" }
                            }
                        }
                    }
                    tbody {
                        {
                            course
                                .termine
                                .iter()
                                .map(|termin| {
                                    ::yew::html! {
                                        tr {
                                            td {
                                                { &termin.date }
                                            }
                                            td {
                                                { &termin.time_start }
                                            }
                                            td {
                                                { &termin.time_end }
                                            }
                                            td {
                                                { &termin.instructors.clone().unwrap_or_default() }
                                            }
                                            td {
                                                ul {
                                                    {
                                                        termin
                                                            .rooms
                                                            .iter()
                                                            .map(|room| {
                                                                ::yew::html! {
                                                                    li {
                                                                        { &room.name }
                                                                    }
                                                                }
                                                            })
                                                            .collect::<Html>()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                                .collect::<Html>()
                        }
                    }
                }
                h2 {
                    { "Beschreibung" }
                }
                // TODO FIXME this is dangerous

                { Html::from_html_unchecked(course.description.join("\n").into()) }
                h2 {
                    { "Sonstige Informationen" }
                }
                div {
                    { format!("Sprache: {}", course.language) }
                }
                div {
                    { format!("SWS: {}", course.sws.map(|v| v.to_string()).unwrap_or_default()) }
                }
                if let Some(anzeige_im_stundenplan) = &course.anzeige_im_stundenplan {
                    div {
                        { format!("Anzeige im Stundenplan: {}", anzeige_im_stundenplan) }
                    }
                }
                div {
                    { format!("Kurslevel: {}", course.courselevel) }
                }
                h2 {
                    { "Enhalten in Modulen" }
                }
                ul {
                    {
                        course
                            .enhalten_in_modulen
                            .iter()
                            .map(|modul| {
                                ::yew::html! {
                                    li {
                                        { modul }
                                    }
                                }
                            })
                            .collect::<Html>()
                    }
                }
            }
        }
    })
}
