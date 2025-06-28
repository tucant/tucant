use dioxus::prelude::*;
use log::error;
use reqwest::StatusCode;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan, TucanError};

use crate::{RcTucanType, Route, login_component::LoginComponent, logout_component::LogoutComponent, navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut};

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
                    Ok(response) => Ok(Some(response)),
                    Err(error) => {
                        // TODO pass through tucanerror from server
                        error!("{}", error);
                        match error {
                            TucanError::Http(ref req) if req.status() == Some(StatusCode::UNAUTHORIZED) => {
                                current_session.set(None);
                                Err("Unauthorized".to_owned())
                            }
                            TucanError::Timeout | TucanError::AccessDenied => {
                                current_session.set(None);
                                Ok(None) // TODO FIXME
                            }
                            _ => Err(error.to_string()),
                        }
                    }
                }
            } else {
                Ok(None)
            }
        }
    });

    rsx! {
        nav { class: "navbar navbar-expand-xl bg-body-tertiary",
            div { class: "container-fluid",
                a { class: "navbar-brand", href: "#/", "TUCaN't" }
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
                    ul { class: "navbar-nav me-auto mb-2 mb-xl-0",
                        if let Some(current_session) = current_session() {
                            if let Some(Ok(data)) = data() {
                                NavbarLoggedIn { current_session, data }
                            }
                        } else {
                            NavbarLoggedOut {}
                        }
                    }
                    if let Some(_current_session) = current_session() {
                        LogoutComponent {}
                    } else {
                        LoginComponent {}
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
