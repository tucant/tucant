use std::{str::FromStr, sync::Arc};

use leptos::{ev::Targeted, html::Input, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_params_map};
use tucant_types::{SemesterId, Tucan, myexams::MyExamsResponse};
use web_sys::{Event, HtmlSelectElement};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[component]
pub fn MyExams() -> impl IntoView {
    let params = use_params_map();
    let semester = move || SemesterId::from_str(&params.read().get("semester").unwrap_or_default()).unwrap();

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: SemesterId| tucan.my_exams(&current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, Signal::derive(semester), 14 * 24 * 60 * 60, 60 * 60, move |exams: MyExamsResponse, reload| {
        let navigate = navigate.clone();
        let on_semester_change = move |e: Targeted<Event, HtmlSelectElement>| {
            let value = e.target().value();
            navigate(&format!("/my-exams/{}", SemesterId::from_str(&value).unwrap()), NavigateOptions::default());
        };
        view! {
            <div>
                <h1>
                    {"Prüfungen"} {" "} <button type="button" class="btn btn-light">
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
                    {exams
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
                            <th scope="col">{"NR"}</th>
                            <th scope="col">{"Name"}</th>
                            <th scope="col">{"Prüfungsart"}</th>
                            <th scope="col">{"Termin"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {exams
                            .exams
                            .clone()
                            .into_iter()
                            .map(|exam| {
                                let exam_name = exam.name.clone();
                                view! {
                                    <tr>
                                        <th scope="row">{exam.id.clone()}</th>
                                        <td>
                                            {move || {
                                                if let Some(coursedetails_url) = &exam.coursedetails_url {
                                                    view! {
                                                        <a href=format!(
                                                            "/course-details/{}",
                                                            coursedetails_url,
                                                        )>{exam_name.clone()}</a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }}
                                            {move || {
                                                if let Some(moduledetails_url) = &exam.moduledetails_url {
                                                    view! {
                                                        <a href=format!(
                                                            "/module-details/{}",
                                                            moduledetails_url,
                                                        )>{exam.name.clone()}</a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }
                                            }}
                                        </td>
                                        <td>
                                            <a href=format!(
                                                "https://www.tucan.tu-darmstadt.de{}",
                                                exam.examdetail_url,
                                            )>{exam.pruefungsart.clone()}</a>
                                        </td>
                                        <td>
                                            {move || {
                                                if let Some(courseprep_url) = &exam.courseprep_url {
                                                    view! {
                                                        <a href=format!(
                                                            "https://www.tucan.tu-darmstadt.de{}",
                                                            courseprep_url,
                                                        )>{exam.date.clone()}</a>
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! { {exam.date.clone()} }.into_any()
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
