use crate::{RcTucanType, Route, common::use_authenticated_data_loader};
use tucant_types::{Tucan, coursedetails::CourseDetailsRequest};
use yew::{Html, Properties, function_component};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetailsRequest,
}

#[function_component(CourseDetails)]
pub fn course_details<TucanType: Tucan + 'static>(CourseDetailsProps { course_details }: &CourseDetailsProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.course_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, course_details.to_owned(), 14 * 24 * 60 * 60, 60 * 60, |course, reload| {
        ::yew::html! {
            <div>
                <h1>
                    { &course.name }
                    if let Some(credits) = course.credits {
                        { " " }
                        <span class="badge text-bg-secondary">
                            { format!("{} CP", credits) }
                        </span>
                    }
                    { " " }
                    <button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h1>
                if !course.instructors.is_empty() {
                    <h2>
                        { "Lehrende" }
                    </h2>
                    <ul>
                        {
                            course
                                .instructors
                                .iter()
                                .map(|instructor| {
                                    ::yew::html! {
                                        <li>
                                            { &instructor.0 }
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                }
                <div>
                    { format!("Typ: {}", course.r#type) }
                </div>
                <div>
                    { format!("Fachbereich: {}", course.fachbereich) }
                </div>
                {
                    match (course.teilnehmer_min, course.teilnehmer_max) {
                        (None, None) => yew::html! {},
                        (None, Some(max)) => ::yew::html! {
                            <div>
                                { format!("Maximal {max} Teilnehmende") }
                            </div>
                        },
                        (Some(min), None) => ::yew::html! {
                            <div>
                                { format!("Mindestens {min} Teilnehmende",) }
                            </div>
                        },
                        (Some(min), Some(max)) => ::yew::html! {
                            <div>
                                { format!("{min} - {max} Teilnehmende",) }
                            </div>
                        },
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
                        if let Some(plenumsveranstaltung) = course.plenumsveranstaltung_url {
                            <tr>
                                <th scope="row">
                                    <Link<Route> to={Route::CourseDetails { course: plenumsveranstaltung.clone() }}>
                                        { "Plenumsveranstaltung" }
                                    </Link<Route>>
                                </th>
                                <td>
                                </td>
                                <td>
                                </td>
                            </tr>
                        }
                        {
                            course
                                .uebungsgruppen
                                .iter()
                                .map(|uebungsgruppe| {
                                    ::yew::html! {
                                        <tr class={if uebungsgruppe.active { "table-primary" } else { "" }}>
                                            <th scope="row">
                                                <Link<Route> to={Route::CourseDetails { course: uebungsgruppe.url.clone() }}>
                                                    { &uebungsgruppe.name }
                                                </Link<Route>>
                                            </th>
                                            <td>
                                                { uebungsgruppe.date_range.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { &uebungsgruppe.uebungsleiter }
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
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
                                    ::yew::html! {
                                        <tr>
                                            <td>
                                                { &anmeldefrist.zulassungstyp }
                                            </td>
                                            <td>
                                                { &anmeldefrist.block_type }
                                            </td>
                                            <td>
                                                { anmeldefrist.start.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { &anmeldefrist.ende_anmeldung.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { &anmeldefrist.ende_abmeldung.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                { &anmeldefrist.ende_hoerer.clone().unwrap_or_default() }
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
                if !course.termine_kleingruppe.is_empty() {
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
                                        ::yew::html! {
                                            <tr>
                                                <td>
                                                    { &termin.date }
                                                </td>
                                                <td>
                                                    { &termin.time_start }
                                                </td>
                                                <td>
                                                    { &termin.time_end }
                                                </td>
                                                <td>
                                                    { &termin.instructors.clone().unwrap_or_default() }
                                                </td>
                                                <td>
                                                    <ul>
                                                        {
                                                            termin
                                                                .rooms
                                                                .iter()
                                                                .map(|room| {
                                                                    ::yew::html! {
                                                                        <li>
                                                                            { &room.name }
                                                                        </li>
                                                                    }
                                                                })
                                                                .collect::<Html>()
                                                        }
                                                    </ul>
                                                </td>
                                            </tr>
                                        }
                                    })
                                    .collect::<Html>()
                            }
                        </tbody>
                    </table>
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
                                    ::yew::html! {
                                        <tr>
                                            <td>
                                                { &termin.date }
                                            </td>
                                            <td>
                                                { &termin.time_start }
                                            </td>
                                            <td>
                                                { &termin.time_end }
                                            </td>
                                            <td>
                                                { &termin.instructors.clone().unwrap_or_default() }
                                            </td>
                                            <td>
                                                <ul>
                                                    {
                                                        termin
                                                            .rooms
                                                            .iter()
                                                            .map(|room| {
                                                                ::yew::html! {
                                                                    <li>
                                                                        { &room.name }
                                                                    </li>
                                                                }
                                                            })
                                                            .collect::<Html>()
                                                    }
                                                </ul>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
                <h2>
                    { "Beschreibung" }
                </h2>
                // TODO FIXME this is dangerous

                { Html::from_html_unchecked(course.description.join("\n").into()) }
                <h2>
                    { "Sonstige Informationen" }
                </h2>
                <div>
                    { format!("Sprache: {}", course.language) }
                </div>
                <div>
                    { format!("SWS: {}", course.sws.map(|v| v.to_string()).unwrap_or_default()) }
                </div>
                if let Some(anzeige_im_stundenplan) = &course.anzeige_im_stundenplan {
                    <div>
                        { format!("Anzeige im Stundenplan: {}", anzeige_im_stundenplan) }
                    </div>
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
                                ::yew::html! {
                                    <li>
                                        { modul }
                                    </li>
                                }
                            })
                            .collect::<Html>()
                    }
                </ul>
            </div>
        }
    })
}
