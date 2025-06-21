use std::rc::Rc;

use tucant_types::{vv::ActionRequest, DynTucan, Tucan};
use dioxus::prelude::*;

use crate::{Route, common::use_unauthenticated_data_loader};

#[component]
pub fn VorlesungsverzeichnisComponent(vv: ActionRequest) -> Element {
    let handler = async |tucan: Rc<DynTucan>, current_session: Option<tucant_types::LoginResponse>, revalidation_strategy, additional| tucan.vv(current_session.as_ref(), revalidation_strategy, additional).await;

    use_unauthenticated_data_loader(handler, vv.to_owned(), 28 * 24 * 60 * 60, 24 * 60 * 60, |data, reload| {
        rsx! {
            div { class: "container",
                h2 { class: "text-center",
                    { data.title.clone() }
                    button { onclick: reload, type: "button", class: "btn btn-light",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg { xmlns: "http://www.w3.org/2000/svg", width: "16", height: "16", fill: "currentColor", class: "bi bi-arrow-clockwise", view_box: "0 0 16 16",
                            path { "fill-rule": "evenodd", d: "M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" }
                            path { d: "M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" }
                        }
                    }
                }
                nav { style: "min-height: 5.5rem", "aria-label": "breadcrumb",
                    ol { class: "breadcrumb",
                        {
                            data.path
                                .iter()
                                .map(|entry| {
                                    rsx! {
                                        li { class: "breadcrumb-item",
                                            Link { to: Route::Vorlesungsverzeichnis { vv: entry.1.clone() },
                                                { entry.0.clone() }
                                            }
                                        }
                                    }
                                })
                                
                        }
                    }
                }
                // TODO FIXME this is dangerous

                div { dangerous_inner_html: data.description.join("\n") }
                h2 { class: "text-center",
                    { "Submenus" }
                }
                ul { class: "list-group",
                    {
                        data.entries
                            .iter()
                            .map(|entry| {
                                rsx! {
                                    Link { to: Route::Vorlesungsverzeichnis { vv: entry.1.clone() }, class: "list-group-item list-group-item-action",
                                        { format!("{}", entry.0) }
                                    }
                                }
                            })
                            
                    }
                }
                h2 { class: "text-center",
                    { "Modules and courses" }
                }
                ul { class: "list-group",
                    {
                        data.veranstaltungen_or_module
                            .iter()
                            .map(|entry| {
                                rsx! {
                                    li { class: "list-group-item",
                                        div { class: "d-flex w-100 justify-content-between",
                                            h5 { class: "mb-1",
                                                Link { to: Route::CourseDetails { course: entry.coursedetails_url.clone() },
                                                    { format!("Kurs {}", entry.title) }
                                                }
                                            }
                                        }
                                        div { class: "d-flex w-100 justify-content-between",
                                            h6 { class: "mb-1",
                                                { entry.lecturer_name.clone().unwrap_or_default().to_string() }
                                            }
                                        }
                                        h6 { class: "mb-1",
                                            { entry.date_range.clone().unwrap_or_default().to_string() }
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
