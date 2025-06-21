use std::rc::Rc;

use tucant_types::{moduledetails::ModuleDetailsRequest, DynTucan, Tucan};
use dioxus::prelude::*;

use crate::{
    common::use_authenticated_data_loader};

#[component]
pub fn ModuleDetails(module_details: ModuleDetailsRequest) -> Element {
    let handler = async |tucan: Rc<DynTucan>, current_session, revalidation_strategy, additional| tucan.module_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, module_details.clone(), 14 * 24 * 60 * 60, 60 * 60, |module, reload| {
        rsx! {
            div {
                h1 {
                    { &module.module_id }
                    if let Some(credits) = &module.credits {
                        { " " }
                        span { class: "badge text-bg-secondary",
                            { format!("{credits} CP",) }
                        }
                    }
                    if module.registered {
                        { " " }
                        span { class: "badge text-bg-secondary",
                            { "Angemeldet" }
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
                h2 {
                    { "Modulverantwortliche" }
                }
                ul {
                    {
                        module
                            .modulverantwortliche
                            .iter()
                            .map(|modulverantwortliche| {
                                rsx! {
                                    li {
                                        { &modulverantwortliche.0 }
                                    }
                                }
                            })
                            
                    }
                }
                h2 {
                    { "Kurse" }
                }
                {
                    module
                        .kurskategorien
                        .iter()
                        .map(|kurskategorie| {
                            rsx! {
                                    h3 {
                                        { &kurskategorie.course_no }
                                        { " " }
                                        { &kurskategorie.name }
                                        if kurskategorie.credits != 0.0 {
                                            { " " }
                                            span { class: "badge text-bg-secondary",
                                                { format!("{} CP", kurskategorie.credits) }
                                            }
                                        }
                                        if kurskategorie.mandatory {
                                            { " " }
                                            span { class: "badge text-bg-secondary",
                                                { "Pflicht" }
                                            }
                                        }
                                        if let Some(semester) = &kurskategorie.semester {
                                            if *semester != 1 {
                                                { " " }
                                                span { class: "badge text-bg-secondary",
                                                    { format!("{semester} Semester") }
                                                }
                                            }
                                        }
                                    }
                                    table { class: "table",
                                        thead {
                                            tr {
                                                th { scope: "col",
                                                    { "Nummer" }
                                                }
                                                th { scope: "col",
                                                    { "Name" }
                                                }
                                                th { scope: "col",
                                                    { "Semester" }
                                                }
                                            }
                                        }
                                        tbody {
                                            {
                                                kurskategorie
                                                    .kurse
                                                    .iter()
                                                    .map(|kurs| {
                                                        rsx! {
                                                            tr {
                                                                th { scope: "row",
                                                                    { &kurs.course_id }
                                                                }
                                                                td {
                                                                    { &kurs.name }
                                                                }
                                                                td {
                                                                    { &kurs.semester }
                                                                }
                                                            }
                                                        }
                                                    })
                                                    
                                            }
                                        }
                                    }
                            }
                        })
                        
                }
                h2 {
                    { "Leistungen" }
                }
                {
                    module
                        .leistungen
                        .iter()
                        .map(|leistung| {
                            rsx! {
                                    h3 {
                                        { &leistung.name }
                                        if leistung.compulsory {
                                            { " " }
                                            span { class: "badge text-bg-secondary",
                                                { "Pflicht" }
                                            }
                                        }
                                        { " " }
                                        span { class: "badge text-bg-secondary",
                                            { format!("{} Gewichtung", leistung.weight) }
                                        }
                                        if let Some(weight_more) = &leistung.weight_more {
                                            { " " }
                                            span { class: "badge text-bg-secondary",
                                                { format!("Zusatzinfo {weight_more}") }
                                            }
                                        }
                                    }
                            }
                        })
                        
                }
                h2 {
                    { "Pruefungen" }
                }
                {
                    module
                        .pruefungen
                        .iter()
                        .map(|pruefung| {
                            rsx! {
                                    h3 {
                                        { &pruefung.name }
                                        if pruefung.compulsory {
                                            { " " }
                                            span { class: "badge text-bg-secondary",
                                                { "Pflicht" }
                                            }
                                        }
                                    }
                                    table { class: "table",
                                        thead {
                                            tr {
                                                th { scope: "col",
                                                    { "Name" }
                                                }
                                                th { scope: "col",
                                                    { "Datum" }
                                                }
                                                th { scope: "col",
                                                    { "Prüfende" }
                                                }
                                            }
                                        }
                                        tbody {
                                            {
                                                pruefung
                                                    .termine
                                                    .iter()
                                                    .map(|termin| {
                                                        rsx! {
                                                            tr {
                                                                th { scope: "row",
                                                                    { &termin.subname }
                                                                }
                                                                td {
                                                                    { &termin.date }
                                                                }
                                                                td {
                                                                    { &termin.examiner }
                                                                }
                                                            }
                                                        }
                                                    })
                                                    
                                            }
                                        }
                                    }
                            }
                        })
                        
                }
                h2 {
                    { "Beschreibung" }
                }
                // TODO FIXME this is dangerous

                div { dangerous_inner_html: module.description.join("\n") }
                h2 {
                    { "Sonstige Informationen" }
                }
                if module.abweichende_credits {
                    div { class: "alert alert-warning", role: "alert",
                        { "Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein." }
                    }
                }
                if let Some(anmeldefristen) = &module.anmeldefristen {
                    div {
                        { format!("Anmeldefrist: {}", anmeldefristen.registration_range) }
                    }
                    div {
                        { format!("Abmeldefrist: {}", anmeldefristen.unregistration_range) }
                    }
                }
                div {
                    { format!("Startsemester: {}", module.start_semester) }
                }
                if let Some(display_in_timetable) = &module.display_in_timetable {
                    div {
                        { format!("Display in timetable: {}", display_in_timetable) }
                    }
                }
                div {
                    { format!("Dauer: {}", module.duration) }
                }
                div {
                    { format!("Anzahl Wahlkurse: {}", module.count_elective_courses) }
                }
                br { }
            }
        }
    })
}
