use std::{ops::Deref, rc::Rc};

use log::info;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan, coursedetails::CourseDetailsRequest};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, hook, html, use_context, use_effect_with, use_state, Callback, Html, HtmlResult, MouseEvent, Properties, UseStateHandle};
use crate::{common::{use_data_loader, DataLoaderReturn}, RcTucanType};
use tucant_types::{coursedetails::CourseDetailsResponse, TucanError};

#[derive(Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetailsRequest,
}

#[function_component(CourseDetails)]
pub fn course_details<TucanType: Tucan + 'static>(CourseDetailsProps { course_details }: &CourseDetailsProps) -> HtmlResult {

    let handler =
        async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| {
            tucan.0.course_details(&current_session, revalidation_strategy, additional).await
        };

    let DataLoaderReturn { data, loading, reload } = use_data_loader(handler, course_details.to_owned());

    let data = match data.deref() {
        Ok(data) => data,
        Err(error) => {
            return Ok(html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors
                        <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
                        </svg>
                        <div>{ error }</div>
                    </div>
                </div>
            });
        }
    };

    Ok(html! {
        <div class="container">
            if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            }

            if let Some(course) = data {
                    <div>

                    <h1>
                        { &course.name }
                        if let Some(credits) = course.credits {
                            {" "}<span class="badge text-bg-secondary">{ format!("{} CP", credits) }</span>
                        }{" "}<button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"/>
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466"/>
                        </svg>
                    </button>
                    </h1>

                    <h2>{"Lehrende"}</h2>
                    <ul>
                    {
                        course.instructors.iter().map(|instructor| {
                            html!{
                                <li>{ &instructor.0 }</li>
                            }
                        }).collect::<Html>()
                    }
                    </ul>

                    <div>{ format!("Typ: {}", course.r#type) }</div>

                    <div>{ format!("Fachbereich: {}", course.fachbereich) }</div>

                    {
                        match (course.teilnehmer_min, course.teilnehmer_max) {
                            (None, None) => html! {
                            },
                            (None, Some(max)) => html! {
                                <div>{ format!("Maximal {max} Teilnehmende") }</div>
                            },
                            (Some(min), None) => html! {
                                <div>{ format!("Mindestens {min} Teilnehmende", ) }</div>
                            },
                            (Some(min), Some(max)) => html! {
                                <div>{ format!("{min} - {max} Teilnehmende", ) }</div>
                            }
                        }
                    }

                    <h2>{"Übungsgruppen"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Name"}</th>
                        <th scope="col">{"Zeitraum"}</th>
                        <th scope="col">{"Uebungsleitende"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.uebungsgruppen.iter().map(|uebungsgruppe| {
                            html!{
                                <tr>
                                    <th scope="row">{&uebungsgruppe.name}</th>
                                    <td>{uebungsgruppe.date_range.clone().unwrap_or_default()}</td>
                                    <td>{&uebungsgruppe.uebungsleiter}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Anmeldefristen"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Phase"}</th>
                        <th scope="col">{"Block"}</th>
                        <th scope="col">{"Start"}</th>
                        <th scope="col">{"Ende Anmeldung"}</th>
                        <th scope="col">{"Ende Abmeldung"}</th>
                        <th scope="col">{"Ende Hörer"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.course_anmeldefristen.iter().map(|anmeldefrist| {
                            html!{
                                <tr>
                                    <td>{&anmeldefrist.zulassungstyp}</td>
                                    <td>{&anmeldefrist.block_type}</td>
                                    <td>{anmeldefrist.start.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_anmeldung.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_abmeldung.clone().unwrap_or_default()}</td>
                                    <td>{&anmeldefrist.ende_hoerer.clone().unwrap_or_default()}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Termine"}</h2>
                    <table class="table">
                    <thead>
                    <tr>
                        <th scope="col">{"Datum"}</th>
                        <th scope="col">{"Start"}</th>
                        <th scope="col">{"Ende"}</th>
                        <th scope="col">{"Kursleitende"}</th>
                        <th scope="col">{"Räume"}</th>
                    </tr>
                    </thead>
                    <tbody>
                    {
                        course.termine.iter().map(|termin| {
                            html!{
                                <tr>
                                    <td>{&termin.date}</td>
                                    <td>{&termin.time_start}</td>
                                    <td>{&termin.time_end}</td>
                                    <td>{&termin.instructors.clone().unwrap_or_default()}</td>
                                    <td><ul>
                                    {
                                        termin.rooms.iter().map(|room| {
                                            html!{
                                                <li>{&room.name}</li>
                                            }
                                        }).collect::<Html>()
                                    }
                                    </ul></td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                    </table>

                    <h2>{"Beschreibung"}</h2>

                    // TODO FIXME this is dangerous
                    { Html::from_html_unchecked(course.description.join("\n").into()) }

                    <h2>{"Sonstige Informationen"}</h2>

                    <div>{ format!("Sprache: {}", course.language) }</div>

                    <div>{ format!("SWS: {}", course.sws.map(|v| v.to_string()).unwrap_or_default()) }</div>

                    if let Some(anzeige_im_stundenplan) = &course.anzeige_im_stundenplan {
                        <div>{ format!("Anzeige im Stundenplan: {}", anzeige_im_stundenplan) }</div>
                    }

                    <div>{ format!("Kurslevel: {}", course.courselevel) }</div>

                    <h2>{"Enhalten in Modulen"}</h2>
                    <ul>
                    {
                        course.enhalten_in_modulen.iter().map(|modul| {
                            html!{
                                <li>{modul}</li>
                            }
                        }).collect::<Html>()
                    }
                    </ul>

                    </div>
                }
        </div>
    })
}
