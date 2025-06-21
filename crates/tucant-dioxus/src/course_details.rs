use std::rc::Rc;

use crate::{ Route, common::use_authenticated_data_loader};
use tucant_types::{coursedetails::CourseDetailsRequest, DynTucan, Tucan};
use dioxus::prelude::*;


#[component]
pub fn CourseDetails(course: ReadOnlySignal<CourseDetailsRequest>) -> Element {
    let handler = async |tucan: Rc<DynTucan>, current_session, revalidation_strategy, additional| tucan.course_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, course.to_owned(), 14 * 24 * 60 * 60, 60 * 60, |course, reload| {
        rsx! {
            div {
                h1 {
                    { course.name.clone() }
                    if let Some(credits) = course.credits {
                        { " " }
                        span { class: "badge text-bg-secondary",
                            { format!("{} CP", credits) }
                        }
                    }
                    { " " }
                    button { onclick: reload, type: "button", class: "btn btn-light",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", fill: "currentColor", class: "bi bi-arrow-clockwise", view_box: "0 0 16 16",
                            path { "fill-rule": "evenodd", d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" }
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
                                    rsx! {
                                        li {
                                            { instructor.0.clone() }
                                        }
                                    }
                                })
                                
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
                        (None, None) => rsx! {},
                        (None, Some(max)) => rsx! {
                            div {
                                { format!("Maximal {max} Teilnehmende") }
                            }
                        },
                        (Some(min), None) => rsx! {
                            div {
                                { format!("Mindestens {min} Teilnehmende",) }
                            }
                        },
                        (Some(min), Some(max)) => rsx! {
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
                                    rsx! {
                                        tr { class: if uebungsgruppe.active { "table-primary" } else { "" },
                                            th { scope: "row",
                                                Link { to: Route::CourseDetails { course: uebungsgruppe.url.clone() },
                                                    { uebungsgruppe.name.clone() }
                                                }
                                            }
                                            td {
                                                { uebungsgruppe.date_range.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { uebungsgruppe.uebungsleiter.clone() }
                                            }
                                        }
                                    }
                                })
                                
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
                                    rsx! {
                                        tr {
                                            td {
                                                { anmeldefrist.zulassungstyp.clone() }
                                            }
                                            td {
                                                { anmeldefrist.block_type.clone() }
                                            }
                                            td {
                                                { anmeldefrist.start.clone().unwrap_or_default() }
                                            }
                                            td {
                                                { anmeldefrist.ende_anmeldung.clone().unwrap_or_default().clone() }
                                            }
                                            td {
                                                { anmeldefrist.ende_abmeldung.clone().unwrap_or_default().clone() }
                                            }
                                            td {
                                                { anmeldefrist.ende_hoerer.clone().unwrap_or_default().clone() }
                                            }
                                        }
                                    }
                                })
                                
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
                                        rsx! {
                                            tr {
                                                td {
                                                    { termin.date.clone() }
                                                }
                                                td {
                                                    { termin.time_start.clone() }
                                                }
                                                td {
                                                    { termin.time_end.clone() }
                                                }
                                                td {
                                                    { termin.instructors.clone().unwrap_or_default().clone() }
                                                }
                                                td {
                                                    ul {
                                                        {
                                                            termin
                                                                .rooms
                                                                .iter()
                                                                .map(|room| {
                                                                    rsx! {
                                                                        li {
                                                                            { room.name.clone() }
                                                                        }
                                                                    }
                                                                })
                                                                
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    })
                                    
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
                                    rsx! {
                                        tr {
                                            td {
                                                { termin.date.clone() }
                                            }
                                            td {
                                                { termin.time_start.clone() }
                                            }
                                            td {
                                                { termin.time_end.clone() }
                                            }
                                            td {
                                                { termin.instructors.clone().unwrap_or_default().clone() }
                                            }
                                            td {
                                                ul {
                                                    {
                                                        termin
                                                            .rooms
                                                            .iter()
                                                            .map(|room| {
                                                                rsx! {
                                                                    li {
                                                                        { room.name.clone() }
                                                                    }
                                                                }
                                                            })
                                                            
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })
                                
                        }
                    }
                }
                h2 {
                    { "Beschreibung" }
                }
                // TODO FIXME this is dangerous

                div { dangerous_inner_html: course.description.join("\n") }
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
                                rsx! {
                                    li {
                                        { modul.clone() }
                                    }
                                }
                            })
                            
                    }
                }
            }
        }
    })
}
