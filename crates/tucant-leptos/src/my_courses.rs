use std::{str::FromStr, sync::Arc};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};
use leptos::{ev::Targeted, prelude::*};
use leptos_router::NavigateOptions;
use tucant_types::{SemesterId, Tucan, mycourses::MyCoursesResponse};
use web_sys::{Event, HtmlSelectElement};

#[component]
pub fn MyCourses(semester: SemesterId) -> impl IntoView {
    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional| tucan.my_courses(&current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, semester.clone(), 14 * 24 * 60 * 60, 60 * 60, move |my_modules: MyCoursesResponse, reload| {
        let navigate = navigate.clone();
        let on_semester_change = move |e: Targeted<Event, HtmlSelectElement>| {
            let value = e.target().value();
            navigate(&format!("my-courses/{}", SemesterId::from_str(&value).unwrap()), NavigateOptions::default());
        };
        view! {
            <div>
                <h1>
                    { "Meine Veranstaltungen" }
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
                <select on:change:target=on_semester_change class="form-select mb-1" aria-label="Select semester">
                    {
                        my_modules
                            .semester
                            .iter()
                            .map(|semester| {
                                view! {
                                    <option selected={semester.selected} value={semester.value.inner().clone()}>
                                        { semester.name.clone() }
                                    </option>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </select>
                {
                    my_modules
                        .sections
                        .iter()
                        .map(|section| {
                            view! {
                                <>
                                    <h2>
                                        { section.0.clone() }
                                    </h2>
                                    <table class="table">
                                        <thead>
                                            <tr>
                                                <th scope="col">
                                                    { "NR" }
                                                </th>
                                                <th scope="col">
                                                    { "Name" }
                                                </th>
                                                <th scope="col">
                                                    { "Zeitraum" }
                                                </th>
                                                <th scope="col">
                                                    { "Standort" }
                                                </th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {
                                                section
                                                    .1
                                                    .iter()
                                                    .map(|course| {
                                                        view! {
                                                            <tr>
                                                                <th scope="row">
                                                                    { course.nr.clone() }
                                                                </th>
                                                                <td>
                                                                    <a href=format!("/course-details/{}", course.url.clone())>
                                                                        { course.title.clone() }
                                                                    </a>
                                                                </td>
                                                                <td>
                                                                    { course.date_range.clone() }
                                                                </td>
                                                                <td>
                                                                    { course.location.clone() }
                                                                </td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        </tbody>
                                    </table></>
                            }
                        })
                        .collect::<Vec<_>>()
                }
            </div>
        }
        .into_any()
    })
}
