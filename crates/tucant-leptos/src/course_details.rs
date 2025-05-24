use std::sync::Arc;

use crate::{Route, api_server::ApiServerTucan, common::use_authenticated_data_loader};
use leptos::{ev::Targeted, prelude::*};
use leptos_router::hooks::use_params_map;
use tucant_types::{Tucan, coursedetails::CourseDetailsRequest};

#[component]
pub fn course_details() -> impl IntoView {
    let params = use_params_map();
    let course_details = move || CourseDetailsRequest::parse(&params.read().get("course-details").unwrap_or_default());

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: CourseDetailsRequest| tucan.course_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, Signal::derive(course_details), 14 * 24 * 60 * 60, 60 * 60, |course, reload| {
        view! {
            <div>
                <h1>
                    { course.name }
                    {move ||
                        if let Some(credits) = course.credits {
                            view! {
                                { " " }
                                <span class="badge text-bg-secondary">
                                    { format!("{} CP", credits) }
                                </span>
                            }.into_any()
                        } else {
                            view!{}.into_any()
                        }
                    }
                    { " " }
                    <button /*onclick={reload}*/ type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h1>
                {move ||
                    if !course.instructors.is_empty() {
                        view! {
                            <h2>
                                { "Lehrende" }
                            </h2>
                            <ul>
                                {
                                    course
                                        .instructors
                                        .iter()
                                        .map(|instructor| {
                                            view! {
                                                <li>
                                                    { instructor.0.clone() }
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
                            </ul>
                        }.into_any()
                    } else {
                        view!{}.into_any()
                    }
                }
                <div>
                    { format!("Typ: {}", course.r#type) }
                </div>
                <div>
                    { format!("Fachbereich: {}", course.fachbereich) }
                </div>
                {
                    match (course.teilnehmer_min, course.teilnehmer_max) {
                        (None, None) => view! {}.into_any(),
                        (None, Some(max)) => view! {
                            <div>
                                { format!("Maximal {max} Teilnehmende") }
                            </div>
                        }.into_any(),
                        (Some(min), None) => view! {
                            <div>
                                { format!("Mindestens {min} Teilnehmende",) }
                            </div>
                        }.into_any(),
                        (Some(min), Some(max)) => view! {
                            <div>
                                { format!("{min} - {max} Teilnehmende",) }
                            </div>
                        }.into_any(),
                    }
                }
                <h2>
                    { "Übungsgruppen" }
                </h2>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Zeitraum" }
                            </th>
                            <th scope="col">
                                { "Uebungsleitende" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        { move ||
                            if let Some(plenumsveranstaltung) = &course.plenumsveranstaltung_url {
                                view! {
                                    <tr>
                                        <th scope="row">
                                            <a href=format!("/course-details/{}", plenumsveranstaltung)>
                                                { "Plenumsveranstaltung" }
                                            </a>
                                        </th>
                                        <td>
                                        </td>
                                        <td>
                                        </td>
                                    </tr>
                                }.into_any()
                            } else {
                                view!{}.into_any()
                            }
                        }
                        {
                            course
                                .uebungsgruppen
                                .iter()
                                .map(|uebungsgruppe| {
                                    view! {
                                        <tr class={if uebungsgruppe.active { "table-primary" } else { "" }}>
                                            <th scope="row">
                                                <a href=format!("/course-details/{}", uebungsgruppe.url.clone())>
                                                    { uebungsgruppe.name.clone() }
                                                </a>
                                            </th>
                                            <td>
                                                { uebungsgruppe.date_range.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { uebungsgruppe.uebungsleiter.clone() }
                                            </td>
                                        </tr>
                                    }.into_any()
                                })
                                .collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
                <h2>
                    { "Anmeldefristen" }
                </h2>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Phase" }
                            </th>
                            <th scope="col">
                                { "Block" }
                            </th>
                            <th scope="col">
                                { "Start" }
                            </th>
                            <th scope="col">
                                { "Ende Anmeldung" }
                            </th>
                            <th scope="col">
                                { "Ende Abmeldung" }
                            </th>
                            <th scope="col">
                                { "Ende Hörer" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            course
                                .course_anmeldefristen
                                .iter()
                                .map(|anmeldefrist| {
                                    view! {
                                        <tr>
                                            <td>
                                                { anmeldefrist.zulassungstyp.clone() }
                                            </td>
                                            <td>
                                                { anmeldefrist.block_type.clone() }
                                            </td>
                                            <td>
                                                { anmeldefrist.start.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { anmeldefrist.ende_anmeldung.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { anmeldefrist.ende_abmeldung.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { anmeldefrist.ende_hoerer.clone().unwrap_or_default() }
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
                {move ||
                if !course.termine_kleingruppe.is_empty() {
                    view! {
                        <h2>
                            { "Termine Kleingruppe" }
                        </h2>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th scope="col">
                                        { "Datum" }
                                    </th>
                                    <th scope="col">
                                        { "Start" }
                                    </th>
                                    <th scope="col">
                                        { "Ende" }
                                    </th>
                                    <th scope="col">
                                        { "Kursleitende" }
                                    </th>
                                    <th scope="col">
                                        { "Räume" }
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    course
                                        .termine_kleingruppe
                                        .iter()
                                        .map(|termin| {
                                            view! {
                                                <tr>
                                                    <td>
                                                        { termin.date.clone() }
                                                    </td>
                                                    <td>
                                                        { termin.time_start.clone() }
                                                    </td>
                                                    <td>
                                                        { termin.time_end.clone() }
                                                    </td>
                                                    <td>
                                                        { termin.instructors.clone().unwrap_or_default() }
                                                    </td>
                                                    <td>
                                                        <ul>
                                                            {
                                                                termin
                                                                    .rooms
                                                                    .iter()
                                                                    .map(|room| {
                                                                        view! {
                                                                            <li>
                                                                                { room.name.clone() }
                                                                            </li>
                                                                        }
                                                                    })
                                                                    .collect::<Vec<_>>()
                                                            }
                                                        </ul>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }
                            </tbody>
                        </table>
                        }.into_any()
                    } else {
                        view!{}.into_any()
                    }
                }
                <h2>
                    { "Termine Plenumsveranstaltung" }
                </h2>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Datum" }
                            </th>
                            <th scope="col">
                                { "Start" }
                            </th>
                            <th scope="col">
                                { "Ende" }
                            </th>
                            <th scope="col">
                                { "Kursleitende" }
                            </th>
                            <th scope="col">
                                { "Räume" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            course
                                .termine
                                .iter()
                                .map(|termin| {
                                    view! {
                                        <tr>
                                            <td>
                                                { termin.date.clone() }
                                            </td>
                                            <td>
                                                { termin.time_start.clone() }
                                            </td>
                                            <td>
                                                { termin.time_end.clone() }
                                            </td>
                                            <td>
                                                { termin.instructors.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                <ul>
                                                    {
                                                        termin
                                                            .rooms
                                                            .iter()
                                                            .map(|room| {
                                                                view! {
                                                                    <li>
                                                                        { room.name.clone() }
                                                                    </li>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()
                                                    }
                                                </ul>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
                <h2>
                    { "Beschreibung" }
                </h2>
                // TODO FIXME this is dangerous
                <div inner_html=course.description.join("\n") />
                <h2>
                    { "Sonstige Informationen" }
                </h2>
                <div>
                    { format!("Sprache: {}", course.language) }
                </div>
                <div>
                    { format!("SWS: {}", course.sws.map(|v| v.to_string()).unwrap_or_default()) }
                </div>
                {move ||
                    if let Some(anzeige_im_stundenplan) = &course.anzeige_im_stundenplan {
                        view! {
                            <div>
                                { format!("Anzeige im Stundenplan: {}", anzeige_im_stundenplan) }
                            </div>
                        }.into_any()
                    } else {
                        view!{}.into_any()
                    }
                }
                <div>
                    { format!("Kurslevel: {}", course.courselevel) }
                </div>
                <h2>
                    { "Enhalten in Modulen" }
                </h2>
                <ul>
                    {
                        course
                            .enhalten_in_modulen
                            .iter()
                            .map(|modul| {
                                view! {
                                    <li>
                                        { modul.clone() }
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        }
        .into_any()
    })
}
