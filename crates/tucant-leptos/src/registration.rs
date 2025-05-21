use std::sync::Arc;

use leptos::{html::Input, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_params_map};
use tucant_types::{
    Tucan,
    registration::{AnmeldungRequest, RegistrationState},
};

use crate::{Route, api_server::ApiServerTucan, common::use_authenticated_data_loader};

#[component]
pub fn Registration() -> impl IntoView {
    let params = use_params_map();
    let registration = move || AnmeldungRequest::parse(&params.read().get("registration").unwrap_or_default());

    let handler = async |tucan: Arc<ApiServerTucan>, current_session, revalidation_strategy, additional| tucan.anmeldung(current_session, revalidation_strategy, additional).await;

    let navigate = leptos_router::hooks::use_navigate();

    use_authenticated_data_loader(handler, registration(), 28 * 24 * 60 * 60, 24 * 60 * 60, |data, reload| {
        if data.submenus.len() == 1 && data.additional_information.is_empty() && data.entries.is_empty() {
            navigate(&format!("/registration/{}", data.submenus[0].1.clone()), NavigateOptions::default());
            return view! {
                <></>
            }
            .into_any();
        }
        view! {
            <div class="container">
                <h2 class="text-center">
                    { "Registration " }
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
                                            <a href=format!("/registration/{}", entry.1)>
                                                { entry.0.clone() }
                                            </a>>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </ol>
                </nav>
                // TODO FIXME this is dangerous
                <div inner_html=data.additional_information.join("\n") />
                <h2 class="text-center">
                    { "Submenus" }
                </h2>
                <ul class="list-group">
                    {
                        data.submenus
                            .iter()
                            .map(|entry| {
                                view! {
                                    <a href=format!("/registration/{}", entry.1.clone()) class="list-group-item list-group-item-action">
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
                        data.entries
                            .iter()
                            .map(|entry| {
                                let module = entry.module.as_ref();
                                view! {
                                    <li class="list-group-item">
                                        <div class="d-flex w-100 justify-content-between">
                                            <h5 class="mb-1">
                                                <a href=format!("/module-details/{}", module.unwrap().url.clone())>
                                                    { format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default()) }
                                                </a>
                                            </h5>
                                            {move ||
                                                if let Some(module) = module {
                                                    if let Some(date) = &module.date {
                                                        view! {
                                                            <small class="text-body-secondary">
                                                                { format!("Anmeldung bis {}", date) }
                                                            </small>
                                                        }.into_any()
                                                    } else {
                                                        view!{}.into_any()
                                                    }
                                                } else {
                                                    view!{}.into_any()
                                                }
                                            }
                                        </div>
                                        <div class="d-flex w-100 justify-content-between">
                                            <h6 class="mb-1">
                                                { format!("{}", module.map(|module| module.lecturer.clone().unwrap_or_default()).unwrap_or_default()) }
                                            </h6>
                                            {move ||
                                                if let Some(module) = module {
                                                    if let Some(limit_and_size) = &module.limit_and_size {
                                                        view! {
                                                            <small class="text-body-secondary">
                                                                { ("Teilnehmerlimit ".to_owned() + limit_and_size) }
                                                            </small>
                                                        }.into_any()
                                                    } else {
                                                        view!{}.into_any()
                                                    }
                                                } else {
                                                    view!{}.into_any()
                                                }
                                            }
                                        </div>
                                        {
                                            module.map(|module| match &module.registration_state {
                                                RegistrationState::Unknown => view! {}.into_any(),
                                                RegistrationState::Registered { unregister_link } => view! {
                                                    <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", unregister_link.clone())}>
                                                        { "Vom Modul abmelden" }
                                                    </a>
                                                }.into_any(),
                                                RegistrationState::NotRegistered { register_link } => view! {
                                                    <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone())}>
                                                        { "Zum Modul anmelden" }
                                                    </a>
                                                }.into_any(),
                                            })
                                        }
                                        <ul class="list-group">
                                            {
                                                entry
                                                    .courses
                                                    .iter()
                                                    .map(|course| {
                                                        view! {
                                                            <li class="list-group-item">
                                                                <div class="d-flex w-100 justify-content-between">
                                                                    <h5 class="mb-1">
                                                                        <a href=format!("/course-details/{}", course.1.url.clone())>
                                                                            { format!("Kurs {} {}", course.1.id, course.1.name) }
                                                                        </a>
                                                                    </h5>
                                                                    {move ||
                                                                        if let Some(registration_until) = &course.1.registration_until {
                                                                            view! {
                                                                                <small class="text-body-secondary">
                                                                                    { format!("Anmeldung bis {}", registration_until) }
                                                                                </small>
                                                                            }.into_any()
                                                                        } else {
                                                                            view!{}.into_any()
                                                                        }
                                                                    }
                                                                </div>
                                                                <div class="d-flex w-100 justify-content-between">
                                                                    <h6 class="mb-1">
                                                                        { format!("{}", course.1.lecturers.clone().unwrap_or_default()) }
                                                                    </h6>
                                                                    // needing the parentheses is a yew bug

                                                                    {move ||
                                                                        if let Some(limit_and_size) = &course.1.limit_and_size {
                                                                            view! {
                                                                                <small class="text-body-secondary">
                                                                                    { ("Teilnehmerlimit ".to_owned() + limit_and_size) }
                                                                                </small>
                                                                            }.into_any()
                                                                        } else {
                                                                            view!{}.into_any()
                                                                        }
                                                                    }
                                                                </div>
                                                                <h6 class="mb-1">
                                                                    { format!("{}", course.1.begin_and_end.clone().unwrap_or_default()) }
                                                                </h6>
                                                                {
                                                                    match &course.1.registration_button_link {
                                                                        RegistrationState::Unknown => view! {}.into_any(),
                                                                        RegistrationState::Registered { unregister_link } => view! {
                                                                            <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", unregister_link.clone())}>
                                                                                { "Vom Kurs abmelden" }
                                                                            </a>
                                                                        }.into_any(),
                                                                        RegistrationState::NotRegistered { register_link } => view! {
                                                                            <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone())}>
                                                                                { "Zum Kurs anmelden" }
                                                                            </a>
                                                                        }.into_any(),
                                                                    }
                                                                }
                                                            </li>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        </ul>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        }
    })
}
