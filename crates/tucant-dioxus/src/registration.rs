use dioxus::prelude::*;
use tucant_types::{
    Tucan,
    registration::{AnmeldungRequest, RegistrationState},
};

use crate::{RcTucanType, Route, common::use_authenticated_data_loader};

#[component]
pub fn Registration(registration: ReadSignal<AnmeldungRequest>) -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, additional| {
        tucan
            .anmeldung(current_session, revalidation_strategy, additional)
            .await
    };

    let navigator = use_navigator();

    use_authenticated_data_loader(
        handler,
        registration.to_owned(),
        28 * 24 * 60 * 60,
        24 * 60 * 60,
        |data, reload| {
            if data.submenus.len() == 1
                && data.additional_information.is_empty()
                && data.entries.is_empty()
            {
                navigator.replace(Route::Registration {
                    registration: data.submenus[0].1.clone(),
                });
                return rsx! {};
            }
            let on_course_of_study_change = {
                Callback::new(move |e: Event<FormData>| {
                    let value = e.value();
                    navigator.push(Route::Registration {
                        registration: AnmeldungRequest::parse(&value),
                    });
                })
            };
            rsx! {
            div { class: "container",
                h2 { class: "text-center",
                    {"Registration "}
                    button {
                        onclick: reload,
                        r#type: "button",
                        class: "btn btn-secondary",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            fill: "currentColor",
                            class: "bi bi-arrow-clockwise",
                            view_box: "0 0 16 16",
                            path {
                                "fill-rule": "evenodd",
                                d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z",
                            }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                select {
                    onchange: on_course_of_study_change,
                    class: "form-select mb-1",
                    "aria-label": "Select course of study",
                    {
                        data.studiumsauswahl
                            .iter()
                            .map(|course_of_study| {
                                rsx! {
                                    option {
                                        selected: course_of_study.selected,
                                        value: course_of_study.value.to_string(),
                                        {course_of_study.name.clone()}
                                    }
                                }
                            })
                    }
                }
                nav {
                    class: "mt-2",
                    style: "min-height: 5.5rem",
                    "aria-label": "breadcrumb",
                    ol { class: "breadcrumb",
                        {
                            data.path
                                .iter()
                                .map(|entry| {
                                    rsx! {
                                        li { class: "breadcrumb-item",
                                            Link {
                                                to: Route::Registration {
                                                    registration: entry.1.clone(),
                                                },
                                                {entry.0.clone()}
                                            }
                                        }
                                    }
                                })
                        }
                    }
                }
                // TODO FIXME this is dangerous

                div { dangerous_inner_html: data.additional_information.join("\n") }
                h2 { class: "text-center", {"Submenus"} }
                ul { class: "list-group",
                    {
                        data.submenus
                            .iter()
                            .map(|entry| {
                                rsx! {
                                    Link {
                                        to: Route::Registration {
                                            registration: entry.1.clone(),
                                        },
                                        class: "list-group-item list-group-item-action",
                                        {entry.0.to_string()}
                                    }
                                }
                            })
                    }
                }
                h2 { class: "text-center", {"Modules and courses"} }
                ul { class: "list-group",
                    {
                        data.entries
                            .iter()
                            .map(|entry| {
                                let module = entry.module.as_ref();
                                rsx! {
                                    li { class: "list-group-item",
                                        div { class: "d-flex w-100 justify-content-between",
                                            h5 { class: "mb-1",
                                                Link {
                                                    to: Route::ModuleDetails {
                                                        module: module.unwrap().url.clone(),
                                                    },
                                                    {
                                                        format!(
                                                            "Modul {} {}",
                                                            module.map(|module| module.id.clone()).unwrap_or_default(),
                                                            module.map(|module| module.name.clone()).unwrap_or_default(),
                                                        )
                                                    }
                                                }
                                            }
                                            if let Some(module) = module {
                                                if let Some(date) = &module.date {
                                                    small { class: "text-body-secondary", {format!("Anmeldung bis {date}")} }
                                                }
                                            }
                                        }
                                        div { class: "d-flex w-100 justify-content-between",
                                            h6 { class: "mb-1",
                                                {
                                                    module
                                                        .map(|module| module.lecturer.clone().unwrap_or_default())
                                                        .unwrap_or_default()
                                                        .to_string()
                                                }
                                            }
                                            if let Some(module) = module {
                                                if let Some(limit_and_size) = &module.limit_and_size {
                                                    small { class: "text-body-secondary", {"Teilnehmerlimit ".to_owned() + limit_and_size} }
                                                }
                                            }
                                        }
                                        {
                                            module
                                                .map(|module| match &module.registration_state {
                                                    RegistrationState::Unknown => rsx! {},
                                                    RegistrationState::Registered { unregister_link } => {
                                                        rsx! {
                                                            a {
                                                                class: "btn btn-danger mb-1",
                                                                role: "button",
                                                                href: format!("https://www.tucan.tu-darmstadt.de{}", unregister_link.clone()),
                                                                {"Vom Modul abmelden"}
                                                            }
                                                        }
                                                    }
                                                    RegistrationState::NotRegistered { register_link } => {
                                                        rsx! {
                                                            a {
                                                                class: "btn btn-outline-success mb-1",
                                                                role: "button",
                                                                href: format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone()),
                                                                {"Zum Modul anmelden"}
                                                            }
                                                        }
                                                    }
                                                })
                                        }
                                        ul { class: "list-group",
                                            {
                                                entry
                                                    .courses
                                                    .iter()
                                                    .map(|course| {
                                                        rsx! {
                                                            li { class: "list-group-item",
                                                                div { class: "d-flex w-100 justify-content-between",
                                                                    h5 { class: "mb-1",
                                                                        Link {
                                                                            to: Route::CourseDetails {
                                                                                course: course.1.url.clone(),
                                                                            },
                                                                            {format!("Kurs {} {}", course.1.id, course.1.name)}
                                                                        }
                                                                    }
                                                                    if let Some(registration_until) = &course.1.registration_until {
                                                                        small { class: "text-body-secondary", {format!("Anmeldung bis {registration_until}")} }
                                                                    }
                                                                }
                                                                div { class: "d-flex w-100 justify-content-between",
                                                                    h6 { class: "mb-1", {course.1.lecturers.clone().unwrap_or_default().to_string()} }
                                                                    if let Some(limit_and_size) = &course.1.limit_and_size {
                                                                        small { class: "text-body-secondary", {"Teilnehmerlimit ".to_owned() + limit_and_size} }
                                                                    }
                                                                }
                                                                h6 { class: "mb-1", {course.1.begin_and_end.clone().unwrap_or_default().to_string()} }
                                                                {
                                                                    match &course.1.registration_button_link {
                                                                        RegistrationState::Unknown => rsx! {},
                                                                        RegistrationState::Registered { unregister_link } => {
                                                                            rsx! {
                                                                                a {
                                                                                    class: "btn btn-danger mb-1",
                                                                                    role: "button",
                                                                                    href: format!("https://www.tucan.tu-darmstadt.de{}", unregister_link.clone()),
                                                                                    {"Vom Kurs abmelden"}
                                                                                }
                                                                            }
                                                                        }
                                                                        RegistrationState::NotRegistered { register_link } => {
                                                                            rsx! {
                                                                                a {
                                                                                    class: "btn btn-outline-success mb-1",
                                                                                    role: "button",
                                                                                    href: format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone()),
                                                                                    {"Zum Kurs anmelden"}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
        }
        },
    )
}
