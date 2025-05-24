use std::sync::Arc;

use leptos::{ev::Targeted, prelude::*};
use leptos_router::hooks::use_params_map;
use tucant_types::{Tucan, vv::ActionRequest};

use crate::{Route, api_server::ApiServerTucan, common::use_unauthenticated_data_loader};

#[component]
pub fn VorlesungsverzeichnisComponent() -> impl IntoView {
    let params = use_params_map();
    let vv = move || ActionRequest::parse(&params.read().get("vv").unwrap_or_default());

    let handler = async |tucan: Arc<ApiServerTucan>, current_session: Option<tucant_types::LoginResponse>, revalidation_strategy, additional: Signal<ActionRequest>| tucan.vv(current_session.as_ref(), revalidation_strategy, additional.get()).await;

    use_unauthenticated_data_loader(handler, Signal::derive(vv).into(), 28 * 24 * 60 * 60, 24 * 60 * 60, |data, reload| {
        view! {
            <div class="container">
                <h2 class="text-center">
                    { data.title }
                    <button /*onclick={reload}*/ type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h2>
                <nav style="min-height: 5.5rem" aria-label="breadcrumb">
                    <ol class="breadcrumb">
                        {
                            data.path
                                .iter()
                                .map(|entry| {
                                    view! {
                                        <li class="breadcrumb-item">
                                            <a href=format!("/vv/{}", entry.1.clone())>
                                                { entry.0.clone() }
                                            </a>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </ol>
                </nav>
                // TODO FIXME this is dangerous
                <div inner_html=data.description.join("\n") />
                <h2 class="text-center">
                    { "Submenus" }
                </h2>
                <ul class="list-group">
                    {
                        data.entries
                            .iter()
                            .map(|entry| {
                                view! {
                                    <a href=format!("/vv/{}", entry.1.clone()) class="list-group-item list-group-item-action">
                                        { format!("{}", entry.0) }
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
                <h2 class="text-center">
                    { "Modules and courses" }
                </h2>
                <ul class="list-group">
                    {
                        data.veranstaltungen_or_module
                            .iter()
                            .map(|entry| {
                                view! {
                                    <li class="list-group-item">
                                        <div class="d-flex w-100 justify-content-between">
                                            <h5 class="mb-1">
                                                <a href=format!("/course-details/{}", entry.coursedetails_url)>
                                                    { format!("Kurs {}", entry.title) }
                                                </a>
                                            </h5>
                                        </div>
                                        <div class="d-flex w-100 justify-content-between">
                                            <h6 class="mb-1">
                                                { format!("{}", entry.lecturer_name.clone().unwrap_or_default()) }
                                            </h6>
                                        </div>
                                        <h6 class="mb-1">
                                            { format!("{}", entry.date_range.clone().unwrap_or_default()) }
                                        </h6>
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
