use dioxus::prelude::*;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan};

use crate::{
    RcTucanType, Route, common::handle_error, login_component::LoginComponent,
    logout_component::LogoutComponent, navbar_logged_in::NavbarLoggedIn,
    navbar_logged_out::NavbarLoggedOut,
};

//use crate::{LoginComponent, LogoutComponent, RcTucanType,
// navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut};

// https://github.com/marc2332/dioxus-query

#[component]
pub fn Navbar() -> Element {
    let tucan: RcTucanType = use_context();

    let current_session = use_context::<Signal<Option<LoginResponse>>>();

    let data = use_resource(move || {
        let tucan = tucan.clone();
        async move {
            if let Some(the_current_session) = current_session() {
                match tucan
                    .after_login(&the_current_session, RevalidationStrategy::cache())
                    .await
                {
                    Ok(response) => Ok(Some(response)),
                    Err(error) => handle_error(current_session, error, true).await,
                }
            } else {
                Ok(None)
            }
        }
    });

    let back_button = if cfg!(feature = "mobile") {
        rsx! {
            button {
                r#type: "button",
                class: "btn",
                onclick: |_| {
                    navigator().go_back();
                },
                svg {
                    class: "bi bi-arrow-left",
                    fill: "currentColor",
                    height: "16",
                    view_box: "0 0 16 16",
                    width: "16",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 \
                         0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8",
                        fill_rule: "evenodd",
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        nav {
            class: "navbar navbar-expand-xl bg-body-tertiary",
            div {
                class: "container-fluid",
                { back_button }
                a {
                    class: "navbar-brand",
                    href: "#/",
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
                    span {
                        class: "navbar-toggler-icon",
                    }
                }
                div {
                    class: "collapse navbar-collapse",
                    id: "navbarSupportedContent",
                    ul {
                        class: "navbar-nav me-auto mb-2 mb-xl-0",
                        if let Some(current_session) = current_session() {
                            if let Some(Ok(data)) = data() {
                                NavbarLoggedIn {
                                    current_session,
                                    data,
                                }
                            } else {
                                NavbarLoggedOut {
                                }
                            }
                        } else {
                            NavbarLoggedOut {
                            }
                        }
                    }
                    if let Some(_current_session) = current_session() {
                        LogoutComponent {
                        }
                    } else {
                        LoginComponent {
                        }
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
