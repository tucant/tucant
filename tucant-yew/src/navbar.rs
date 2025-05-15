use std::ops::Deref;

use log::error;
use reqwest::StatusCode;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan, TucanError};
use wasm_bindgen_futures::spawn_local;
use yew::{Html, UseStateHandle, function_component, use_context, use_effect_with, use_state};

use crate::{LoginComponent, LogoutComponent, RcTucanType, navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut};

#[function_component(Navbar)]
pub fn navbar<TucanType: Tucan + 'static>() -> Html {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let data = use_state(|| Ok(None));

    {
        let data = data.clone();
        use_effect_with(current_session_handle.clone(), move |current_session_handle| {
            if let Some(current_session) = (&**current_session_handle).to_owned() {
                let current_session_handle = current_session_handle.clone();
                spawn_local(async move {
                    match tucan.0.after_login(&current_session, RevalidationStrategy::cache()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                        }
                        Err(error) => {
                            // TODO pass through tucanerror from server
                            error!("{}", error);
                            match error {
                                TucanError::Http(ref req) if req.status() == Some(StatusCode::UNAUTHORIZED) => {
                                    current_session_handle.set(None);
                                    data.set(Err("Unauthorized".to_owned()))
                                }
                                TucanError::Timeout | TucanError::AccessDenied => {
                                    current_session_handle.set(None);
                                }
                                _ => {
                                    data.set(Err(error.to_string()));
                                }
                            }
                        }
                    }
                })
            }
        });
    }

    ::yew::html! {
        <>
            <nav class="navbar navbar-expand-xl bg-body-tertiary">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#/">
                        { "TUCaN't" }
                    </a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon" />
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-xl-0">
                            if let Some(current_session) = &*current_session_handle {
                                if let Ok(data) = &*data {
                                    <NavbarLoggedIn current_session={current_session.clone()} data={data.clone()} />
                                }
                            } else {
                                <NavbarLoggedOut />
                            }
                        </ul>
                        if !current_session_handle.is_some() {
                            <LoginComponent<TucanType> />
                        } else {
                            <LogoutComponent<TucanType> />
                        }
                    </div>
                </div>
            </nav>
            {
                match data.deref() {
                    Ok(_data) => yew::html! {},
                    Err(error) => {
                        ::yew::html! {
                            <div class="container">
                                <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                                    // https://github.com/twbs/icons
                                    // The MIT License (MIT)
                                    // Copyright (c) 2019-2024 The Bootstrap Authors

                                    <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                                        <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
                                    </svg>
                                    <div>
                                        { "Navigation bar error: " }
                                        { error.to_string() }
                                    </div>
                                </div>
                            </div>
                        }
                    }
                }
            }</>
    }
}
