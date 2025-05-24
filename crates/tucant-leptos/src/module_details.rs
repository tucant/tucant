use std::sync::Arc;

use leptos::{ev::Targeted, prelude::*};
use leptos_router::hooks::use_params_map;
use tucant_types::{Tucan, moduledetails::ModuleDetailsRequest};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[allow(clippy::too_many_lines)]
#[component]
pub fn ModuleDetails() -> impl IntoView {
    let params = use_params_map();
    let module_details = move || ModuleDetailsRequest::parse(&params.read().get("module-details").unwrap_or_default());

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: ModuleDetailsRequest| tucan.module_details(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, Signal::derive(module_details), 14 * 24 * 60 * 60, 60 * 60, |module, reload| {
        view! {
            <div>
                <h1>
                    {module.module_id}
                    {move || {
                        if let Some(credits) = &module.credits {
                            view! {
                                {" "}
                                <span class="badge text-bg-secondary">
                                    {format!("{credits} CP")}
                                </span>
                            }
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}
                    {move || {
                        if module.registered {
                            view! {
                                {" "}
                                <span class="badge text-bg-secondary">{"Angemeldet"}</span>
                            }
                                .into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }} {" "} <button type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            class="bi bi-arrow-clockwise"
                            viewBox="0 0 16 16"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"
                            />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h1>
                <h2>{"Modulverantwortliche"}</h2>
                <ul>
                    {module
                        .modulverantwortliche
                        .iter()
                        .map(|modulverantwortliche| {
                            view! { <li>{modulverantwortliche.0.clone()}</li> }
                        })
                        .collect::<Vec<_>>()}
                </ul>
                <h2>{"Kurse"}</h2>
                {module
                    .kurskategorien
                    .clone()
                    .into_iter()
                    .map(|kurskategorie| {
                        view! {
                            <>
                                <h3>
                                    {kurskategorie.course_no.clone()} {" "}
                                    {kurskategorie.name.clone()}
                                    {move || {
                                        if kurskategorie.credits != 0.0 {
                                            view! {
                                                {" "}
                                                <span class="badge text-bg-secondary">
                                                    {format!("{} CP", kurskategorie.credits)}
                                                </span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                    {move || {
                                        if kurskategorie.mandatory {
                                            view! {
                                                {" "}
                                                <span class="badge text-bg-secondary">{"Pflicht"}</span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                    {move || {
                                        if let Some(semester) = kurskategorie.semester {
                                            if semester != 1 {
                                                view! {
                                                    {" "}
                                                    <span class="badge text-bg-secondary">
                                                        {format!("{semester} Semester")}
                                                    </span>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                </h3>
                                <table class="table">
                                    <thead>
                                        <tr>
                                            <th scope="col">{"Nummer"}</th>
                                            <th scope="col">{"Name"}</th>
                                            <th scope="col">{"Semester"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {kurskategorie
                                            .kurse
                                            .iter()
                                            .map(|kurs| {
                                                view! {
                                                    <tr>
                                                        <th scope="row">{kurs.course_id.clone()}</th>
                                                        <td>{kurs.name.clone()}</td>
                                                        <td>{kurs.semester.clone()}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </>
                        }
                    })
                    .collect::<Vec<_>>()}
                <h2>{"Leistungen"}</h2>
                {module
                    .leistungen
                    .clone()
                    .into_iter()
                    .map(|leistung| {
                        view! {
                            <>
                                <h3>
                                    {leistung.name.clone()}
                                    {move || {
                                        if leistung.compulsory {
                                            view! {
                                                {" "}
                                                <span class="badge text-bg-secondary">{"Pflicht"}</span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }} {" "}
                                    <span class="badge text-bg-secondary">
                                        {format!("{} Gewichtung", leistung.weight)}
                                    </span>
                                    {move || {
                                        if let Some(weight_more) = &leistung.weight_more {
                                            view! {
                                                {" "}
                                                <span class="badge text-bg-secondary">
                                                    {format!("Zusatzinfo {weight_more}")}
                                                </span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                </h3>
                            </>
                        }
                    })
                    .collect::<Vec<_>>()}
                <h2>{"Pruefungen"}</h2>
                {module
                    .pruefungen
                    .clone()
                    .into_iter()
                    .map(|pruefung| {
                        view! {
                            <>
                                <h3>
                                    {pruefung.name.clone()}
                                    {move || {
                                        if pruefung.compulsory {
                                            view! {
                                                " "
                                                <span class="badge text-bg-secondary">{"Pflicht"}</span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {}.into_any()
                                        }
                                    }}
                                </h3>
                                <table class="table">
                                    <thead>
                                        <tr>
                                            <th scope="col">{"Name"}</th>
                                            <th scope="col">{"Datum"}</th>
                                            <th scope="col">{"Prüfende"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {pruefung
                                            .termine
                                            .iter()
                                            .map(|termin| {
                                                view! {
                                                    <tr>
                                                        <th scope="row">{termin.subname.clone()}</th>
                                                        <td>{termin.date.clone()}</td>
                                                        <td>{termin.examiner.clone()}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </>
                        }
                    })
                    .collect::<Vec<_>>()}
                <h2>{"Beschreibung"}</h2>
                // TODO FIXME this is dangerous
                <div inner_html=module.description.join("\n") />
                <h2>{"Sonstige Informationen"}</h2>
                {move || {
                    if module.abweichende_credits {
                        view! {
                            <div class="alert alert-warning" role="alert">
                                {"Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein."}
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                {move || {
                    if let Some(anmeldefristen) = &module.anmeldefristen {
                        view! {
                            <div>
                                {format!("Anmeldefrist: {}", anmeldefristen.registration_range)}
                            </div>
                            <div>
                                {format!("Abmeldefrist: {}", anmeldefristen.unregistration_range)}
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                <div>{format!("Startsemester: {}", module.start_semester)}</div>
                {move || {
                    if let Some(display_in_timetable) = &module.display_in_timetable {
                        view! {
                            <div>{format!("Display in timetable: {}", display_in_timetable)}</div>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
                <div>{format!("Dauer: {}", module.duration)}</div>
                <div>{format!("Anzahl Wahlkurse: {}", module.count_elective_courses)}</div>
                <br />
            </div>
        }
        .into_any()
    })
}
