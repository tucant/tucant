use std::{ops::Deref, rc::Rc};

use log::error;
use reqwest::StatusCode;
use tucant_types::{DynTucan, LoginResponse, RevalidationStrategy, Tucan, TucanError};
use dioxus::prelude::*;

use crate::{login_component::LoginComponent, logout_component::LogoutComponent, navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut, RcTucanType, Route};

//use crate::{LoginComponent, LogoutComponent, RcTucanType, navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut};

// https://github.com/marc2332/dioxus-query

#[component]
pub fn Navbar() -> Element {
    let tucan: RcTucanType = use_context();

    let mut current_session = use_context::<Signal<Option<LoginResponse>>>();
    
    let data = use_resource(move || {
        let tucan = tucan.clone();
        async move {
        if let Some(the_current_session) = current_session() {
            match tucan.after_login(&the_current_session, RevalidationStrategy::cache()).await {
                Ok(response) => {
                    return Ok(Some(response));
                }
                Err(error) => {
                    // TODO pass through tucanerror from server
                    error!("{}", error);
                    match error {
                        TucanError::Http(ref req) if req.status() == Some(StatusCode::UNAUTHORIZED) => {
                            current_session.set(None);
                            return Err("Unauthorized".to_owned())
                        }
                        TucanError::Timeout | TucanError::AccessDenied => {
                            current_session.set(None);
                            return Ok(None); // TODO FIXME
                        }
                        _ => {
                            return Err(error.to_string());
                        }
                    }
                }
            }
        } else {
            return Ok(None);
        }
    }});
             
    rsx! {
        nav { class: "navbar navbar-expand-xl bg-body-tertiary",
            div { class: "container-fluid",
                a { class: "navbar-brand", href: "#/",
                    "TUCaN't"
                }
                button {
                    aria_controls: "navbarSupportedContent",
                    aria_expanded: "false",
                    aria_label: "Toggle navigation",
                    class: "navbar-toggler",
                    "data-bs-target": "#navbarSupportedContent",
                    "data-bs-toggle": "collapse",
                    r#type: "button",
                    span { class: "navbar-toggler-icon" }
                }
                div {
                    class: "collapse navbar-collapse",
                    id: "navbarSupportedContent",
                    ul {
                        class: "navbar-nav me-auto mb-2 mb-xl-0",
                        if let Some(current_session) = current_session() {
                            if let Some(Ok(data)) = data() {
                                NavbarLoggedIn { current_session: current_session, data: data }
                            }
                        } else {
                            NavbarLoggedOut {}
                        }
                    }
                    if let Some(current_session) = current_session() {
                        LogoutComponent {}
                    } else {
                        LoginComponent {}
                    }
                }
            }
        }
            /*{
                match data.deref() {
                    Ok(_data) => yew::html! {},
                    Err(error) => {
                        rsx! {
                            div { class: "container",
                                div { class: "alert alert-danger d-flex align-items-center mt-2" role: "alert",
                                    // https://github.com/twbs/icons
                                    // The MIT License (MIT)
                                    // Copyright (c) 2019-2024 The Bootstrap Authors

                                    svg { xmlns: "http://www.w3.org/2000/svg" class: "bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width: "16" height: "16" viewBox: "0 0 16 16" role: "img" aria-label: "Error:",
                                        path { d: "M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" }
                                    }
                                    div {
                                        { "Navigation bar error: " }
                                        { error.to_string() }
                                    }
                                }
                            }
                        }
                    }
                }
            }*/
        Outlet::<Route> {}
    }
}
