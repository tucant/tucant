use std::{str::FromStr, sync::Arc};

use leptos::{ev::Targeted, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_params_map};
use tucant_types::{SemesterId, Tucan, mymodules::MyModulesResponse};
use web_sys::{Event, HtmlSelectElement};

use crate::{api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[component]
pub fn MyModules() -> impl IntoView {
    let params = use_params_map();
    let semester = move || SemesterId::from_str(&params.read().get("semester").unwrap_or_default()).unwrap();

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional: SemesterId| tucan.my_modules(&current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, Signal::derive(semester), 14 * 24 * 60 * 60, 60 * 60, move |my_modules: MyModulesResponse, reload| {
        let navigate = navigate.clone();
        let on_semester_change = move |e: Targeted<Event, HtmlSelectElement>| {
            let value = e.target().value();
            navigate(&format!("my-modules/{}", SemesterId::from_str(&value).unwrap()), NavigateOptions::default());
        };
        view! {
            <div>
                <h1>
                    {"Meine Module"} {" "} <button type="button" class="btn btn-light">
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
                    {my_modules
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
                            <th scope="col">{"Verantwortliche Person"}</th>
                            <th scope="col">{"Credits"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {my_modules
                            .modules
                            .iter()
                            .map(|module| {
                                view! {
                                    <tr>
                                        <th scope="row">{module.nr.clone()}</th>
                                        <td>
                                            <a href=format!(
                                                "/module-details/{}",
                                                module.url.clone(),
                                            )>{module.title.clone()}</a>
                                        </td>
                                        <td>{module.lecturer.clone()}</td>
                                        <td>
                                            {module.credits.clone().unwrap_or_else(|| "-".to_owned())}
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
