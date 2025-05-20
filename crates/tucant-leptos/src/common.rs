use leptos::prelude::*;
use log::info;
use std::ops::Deref;
use std::sync::Arc;
use tucant_types::Tucan;
use tucant_types::{LoginResponse, RevalidationStrategy, TucanError};
use web_sys::MouseEvent;

use crate::api_server::ApiServerTucan;

pub fn use_authenticated_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(handler: impl AsyncFn(Arc<ApiServerTucan>, LoginResponse, RevalidationStrategy, I) -> Result<O, TucanError> + Copy + 'static, request: I, cache_age_seconds: i64, max_stale_age_seconds: i64, render: impl Fn(O, Callback<MouseEvent>) -> AnyView + Send + 'static) -> AnyView {
    use_data_loader(true, async move |tucan: Arc<ApiServerTucan>, current_session: Option<LoginResponse>, revalidation_strategy, additional| handler(tucan, current_session.unwrap(), revalidation_strategy, additional).await, request, cache_age_seconds, max_stale_age_seconds, render)
}

pub fn use_unauthenticated_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(handler: impl AsyncFn(Arc<ApiServerTucan>, Option<LoginResponse>, RevalidationStrategy, I) -> Result<O, TucanError> + Copy + 'static, request: I, cache_age_seconds: i64, max_stale_age_seconds: i64, render: impl Fn(O, Callback<MouseEvent>) -> AnyView + Send + 'static) -> AnyView {
    use_data_loader(false, handler, request, cache_age_seconds, max_stale_age_seconds, render)
}

fn use_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(authentication_required: bool, handler: impl AsyncFn(Arc<ApiServerTucan>, Option<LoginResponse>, RevalidationStrategy, I) -> Result<O, TucanError> + Copy + 'static, request: I, cache_age_seconds: i64, max_stale_age_seconds: i64, render: impl Fn(O, Callback<MouseEvent>) -> AnyView + Send + 'static) -> AnyView {
    use reqwest::StatusCode;

    let tucan = use_context::<Arc<ApiServerTucan>>().unwrap();
    let session = use_context::<ReadSignal<Option<LoginResponse>>>().unwrap();
    /*if authentication_required && (**current_session_handle).is_none() {
        data.set(Err("Not logged in".to_owned()));
        return;
    } */

    // https://docs.rs/leptos/latest/leptos/prelude/trait.FromStream.html
    // https://crates.io/crates/leptos-fetch
    let data = {
        LocalResource::new(move || {
            let tucan = tucan.clone();
            let request = request.clone();
            async move {
                match handler(tucan.clone(), session.get(), RevalidationStrategy { max_age: cache_age_seconds, invalidate_dependents: Some(true) }, request.clone()).await {
                    Ok(response) => {
                        return Ok(response);

                        /*match handler(tucan.clone(), session.clone(), RevalidationStrategy { max_age: max_stale_age_seconds, invalidate_dependents: Some(true) }, request).await {
                            Ok(response) => data.set(Ok(Some(response))),
                            Err(error) => {
                                info!("ignoring error when refetching: {}", error)
                            }
                        }*/
                    }
                    Err(error) => {
                        log::error!("{}", error);
                        match error {
                            TucanError::Http(ref req) if req.status() == Some(StatusCode::UNAUTHORIZED) => {
                                //current_session_handle.set(None);
                                return Err("Unauthorized".to_owned());
                            }
                            TucanError::Timeout | TucanError::AccessDenied => {
                                //current_session_handle.set(None);
                                return Err("Unauthorized".to_owned());
                            }
                            _ => {
                                return Err(error.to_string());
                            }
                        }
                    }
                }
            }
        })
    };

    let reload = Callback::new(move |ev: MouseEvent| {});
    /*
    let reload = {
        let course_details = request.clone();
        let data = data.clone();
        let loading = loading.clone();
        let tucan = tucan.clone();
        Callback::from(move |_e: MouseEvent| {
            if authentication_required && (*current_session_handle).is_none() {
                data.set(Err("Not logged in".to_owned()));
                return;
            }
            loading.set(true);
            let course_details = course_details.clone();
            let data = data.clone();
            let tucan = tucan.clone();
            let loading = loading.clone();
            let current_session_handle = current_session_handle.clone();
            spawn_local(async move {
                match handler(tucan.clone(), (*current_session_handle).clone(), RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }, course_details.clone()).await {
                    Ok(response) => {
                        data.set(Ok(Some(response)));
                        loading.set(false);
                    }
                    Err(error) => {
                        log::error!("{}", error);
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
                        loading.set(false);
                    }
                }
            })
        })
    };*/

    view! {
            {move || match data.get() {
            None => view! {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">
                            { "Loading..." }
                        </span>
                    </div>
                </div>
            }
            .into_any(),
            Some(Ok(data)) => view! {
                <div class="container">
                    { render(data, reload) }
                </div>
            }
            .into_any(),
            Some(Err(error)) => {
                return view! {
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
                }
                .into_any();
            }}
        }
    }
    .into_any()
}
