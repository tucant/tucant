use std::sync::Arc;

use leptos::{ev::Targeted, html::Input, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_params_map};
use tucant_types::{
    LoginResponse, Tucan,
    student_result::{StudentResultLevel, StudentResultResponse},
};
use web_sys::{Event, HtmlSelectElement};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[component]
pub fn StudentResult() -> impl IntoView {
    let params = use_params_map();
    let course_of_study = move || params.read().get("course_of_study").unwrap_or_default();

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: u64| tucan.student_result(&current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, Signal::derive(move || if course_of_study() == "default" { 0 } else { course_of_study().parse().unwrap() }), 14 * 24 * 60 * 60, 60 * 60, move |student_result: StudentResultResponse, reload| {
        let navigate = navigate.clone();
        let on_course_of_study_change = move |e: Targeted<Event, HtmlSelectElement>| {
            let value = e.target().value();
            navigate(&format!("/student-result/{}", value), NavigateOptions::default());
        };
        view! {
            <>
                <h1>
                    {"Leistungsspiegel"} {" "} <button type="button" class="btn btn-light">
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
                    on:change:target=on_course_of_study_change
                    class="form-select mb-1"
                    aria-label="Select course of study"
                >
                    {student_result
                        .course_of_study
                        .iter()
                        .map(|course_of_study| {
                            view! {
                                <option
                                    selected=course_of_study.selected
                                    value=course_of_study.value.clone()
                                >
                                    {course_of_study.name.clone()}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
                <StudentResultLevelComponent level=student_result.level0 path=Vec::new() />
                <div>{format!("Gesamt-GPA: {}", student_result.total_gpa)}</div>
                <div>{format!("Hauptfach-GPA: {}", student_result.main_gpa)}</div>
            </>
        }
        .into_any()
    })
}

#[component]
pub fn StudentResultLevelComponent(level: StudentResultLevel, path: Vec<String>) -> impl IntoView {
    view! {
        <>
            {
                let level_name_clone = level.name.clone();
                let path_clone = path.clone();
                move || {
                    if !level.entries.is_empty() {
                        view! {
                            <h5>
                                <nav aria-label="breadcrumb">
                                    <ol class="breadcrumb">
                                        {path_clone
                                            .iter()
                                            .map(|item| {
                                                view! { <li class="breadcrumb-item">{item.clone()}</li> }
                                            })
                                            .collect::<Vec<_>>()}
                                        <li class="breadcrumb-item">{level_name_clone.clone()}</li>
                                    </ol>
                                </nav>
                            </h5>
                            <table class="table table-sm">
                                <thead>
                                    <tr>
                                        <th scope="col">{"Name"}</th>
                                        <th scope="col" class="col-1">
                                            {"CP"}
                                        </th>
                                        <th scope="col" class="col-1">
                                            {"eCP"}
                                        </th>
                                        <th scope="col" class="col-1">
                                            {"Note"}
                                        </th>
                                        <th scope="col" class="col-1">
                                            {"Status"}
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {level
                                        .entries
                                        .iter()
                                        .map(|entry| {
                                            view! {
                                                <tr>
                                                    <td>{entry.name.clone()}</td>
                                                    <td>{entry.cp.clone().unwrap_or_default()}</td>
                                                    <td>{entry.used_cp.clone().unwrap_or_default()}</td>
                                                    <td>{entry.grade.clone().unwrap_or_default()}</td>
                                                    <td>{entry.state.clone()}</td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        }
                            .into_any()
                    } else {
                        view! {}.into_any()
                    }
                }
            }
            {level
                .children
                .iter()
                .map(|child| {
                    view! {
                        <StudentResultLevelComponent
                            level=child.clone()
                            path=path
                                .iter()
                                .cloned()
                                .chain(std::iter::once(level.name.clone()))
                                .collect::<Vec<_>>()
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </>
    }
    .into_any()
}
