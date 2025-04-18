use crate::RcTucanType;
use log::info;
use std::ops::Deref;
use tucant_types::Tucan;
use tucant_types::{LoginResponse, RevalidationStrategy, TucanError};
use yew::Html;
use yew::html;
use yew::use_context;
use yew::{Callback, MouseEvent, UseStateHandle, hook};
use yew::{platform::spawn_local, use_effect_with, use_state};

#[hook]
pub fn use_data_loader<TucanType: Tucan + 'static, I: Clone + PartialEq + 'static, O: Clone + 'static>(handler: impl AsyncFn(RcTucanType<TucanType>, LoginResponse, RevalidationStrategy, I) -> Result<O, TucanError> + Copy + 'static, request: I, cache_age_seconds: i64, max_stale_age_seconds: i64, render: impl Fn(O, Callback<MouseEvent>) -> Html) -> Html {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| Ok(None));
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        use_effect_with((request.to_owned(), current_session_handle.clone()), move |(request, current_session_handle)| {
            if let Some(current_session) = (**current_session_handle).to_owned() {
                loading.set(true);
                let request = request.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                let current_session_handle = current_session_handle.to_owned();
                spawn_local(async move {
                    match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: cache_age_seconds, invalidate_dependents: Some(true) }, request.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);

                            match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: max_stale_age_seconds, invalidate_dependents: Some(true) }, request).await {
                                Ok(response) => data.set(Ok(Some(response))),
                                Err(error) => {
                                    info!("ignoring error when refetching: {}", error)
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("{}", error);
                            if let TucanError::Timeout | TucanError::AccessDenied = error {
                                current_session_handle.set(None);
                            }
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        });
    }

    let reload = {
        let current_session_handle = current_session_handle.clone();
        let course_details = request.clone();
        let data = data.clone();
        let loading = loading.clone();
        let tucan = tucan.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let course_details = course_details.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                let loading = loading.clone();
                let current_session_handle = current_session_handle.clone();
                spawn_local(async move {
                    match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }, course_details.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);
                        }
                        Err(error) => {
                            log::error!("{}", error);
                            if let TucanError::Timeout | TucanError::AccessDenied = error {
                                current_session_handle.set(None);
                            }
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        })
    };

    let data = match data.deref() {
        Ok(data) => data,
        Err(error) => {
            return ::yew::html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
                        </svg>
                        <div>
                            { "Data loading error: " }
                            { error }
                        </div>
                    </div>
                </div>
            };
        }
    };

    ::yew::html! {
        <div class="container">
            if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">
                            { "Loading..." }
                        </span>
                    </div>
                </div>
            }
            if let Some(course) = data {
                { render(course.to_owned(), reload) }
            }
        </div>
    }
}
