use std::{str::FromStr, sync::Arc};

use leptos::{ev::Targeted, html::Input, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_params_map};
use tucant_types::{SemesterId, Tucan, courseresults::ModuleResultsResponse};
use web_sys::{Event, HtmlSelectElement};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[component]
pub fn CourseResults() -> impl IntoView {
    let params = use_params_map();
    let semester = move || SemesterId::from_str(&params.read().get("semester").unwrap_or_default()).unwrap();

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: SemesterId| tucan.course_results(&current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, Signal::derive(semester), 14 * 24 * 60 * 60, 60 * 60, move |course_results: ModuleResultsResponse, reload| {
        let navigate = navigate.clone();
        let on_semester_change = move |e: Targeted<Event, HtmlSelectElement>| {
            let value = e.target().value();
            navigate(&format!("/course-results/{}", SemesterId::from_str(&value).unwrap()), NavigateOptions::default());
        };
        view! {
            <div>
                <h1>
                    {"Modulergebnisse"} {" "} <button type="button" class="btn btn-light">
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
                <select
                    on:change:target=on_semester_change
                    class="form-select mb-1"
                    aria-label="Select semester"
                >
                    {course_results
                        .semester
                        .iter()
                        .map(|semester| {
                            view! {
                                <option
                                    selected=semester.selected
                                    value=semester.value.inner().clone()
                                >
                                    {semester.name.clone()}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">{"Nr"}</th>
                            <th scope="col">{"Name"}</th>
                            <th scope="col">{"Credits"}</th>
                            <th scope="col">{"Note"}</th>
                            <th scope="col">{"Status"}</th>
                            <th scope="col">{"Prüfungen"}</th>
                            <th scope="col">{"Ø"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {course_results
                            .results
                            .clone()
                            .into_iter()
                            .map(|exam| {
                                view! {
                                    <tr>
                                        <th scope="row">{exam.nr.clone()}</th>
                                        <td>{exam.name.clone()}</td>
                                        <td>{exam.credits.clone()}</td>
                                        <td>
                                            {exam.grade.clone().unwrap_or_else(|| "-".to_owned())}
                                        </td>
                                        <td>{exam.status.clone().unwrap_or_default()}</td>
                                        <td>
                                            {move || {
                                                if let Some(pruefungen_url) = &exam.pruefungen_url {
                                                    view! {
                                                        <a href=format!(
                                                            "https://www.tucan.tu-darmstadt.de{}",
                                                            pruefungen_url,
                                                        )>{"Prüfungen"}</a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }}
                                        </td>
                                        <td>
                                            {move || {
                                                if let Some(average_url) = &exam.average_url {
                                                    view! {
                                                        <a href=format!(
                                                            "https://www.tucan.tu-darmstadt.de{}",
                                                            average_url,
                                                        )>{"Ø"}</a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        }
        .into_any()
    })
}
