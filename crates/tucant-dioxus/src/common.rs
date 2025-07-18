use dioxus::prelude::*;
use log::info;
use tucant_types::{LoginResponse, RevalidationStrategy, TucanError};

use crate::RcTucanType;

pub fn use_authenticated_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(
    handler: impl AsyncFn(RcTucanType, LoginResponse, RevalidationStrategy, I) -> Result<O, TucanError>
        + Copy
        + 'static,
    request: ReadOnlySignal<I>,
    cache_age_seconds: i64,
    max_stale_age_seconds: i64,
    render: impl Fn(O, Callback<MouseEvent>) -> Element,
) -> Element {
    use_data_loader(
        true,
        async move |tucan: RcTucanType,
                    current_session: Option<LoginResponse>,
                    revalidation_strategy,
                    additional| {
            handler(
                tucan,
                current_session.unwrap(),
                revalidation_strategy,
                additional,
            )
            .await
        },
        request,
        cache_age_seconds,
        max_stale_age_seconds,
        render,
    )
}

pub fn use_unauthenticated_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(
    handler: impl AsyncFn(
            RcTucanType,
            Option<LoginResponse>,
            RevalidationStrategy,
            I,
        ) -> Result<O, TucanError>
        + Copy
        + 'static,
    request: ReadOnlySignal<I>,
    cache_age_seconds: i64,
    max_stale_age_seconds: i64,
    render: impl Fn(O, Callback<MouseEvent>) -> Element,
) -> Element {
    use_data_loader(
        false,
        handler,
        request,
        cache_age_seconds,
        max_stale_age_seconds,
        render,
    )
}

fn use_data_loader<I: Clone + PartialEq + 'static, O: Clone + 'static>(
    authentication_required: bool,
    handler: impl AsyncFn(
            RcTucanType,
            Option<LoginResponse>,
            RevalidationStrategy,
            I,
        ) -> Result<O, TucanError>
        + Copy
        + 'static,
    request: ReadOnlySignal<I>,
    cache_age_seconds: i64,
    max_stale_age_seconds: i64,
    render: impl Fn(O, Callback<MouseEvent>) -> Element,
) -> Element {
    use reqwest::StatusCode;

    let tucan: RcTucanType = use_context();

    let mut data = use_signal(|| Ok(None));
    let mut loading = use_signal(|| false);
    let mut current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    {
        let tucan = tucan.clone();
        let _ = use_resource(move || {
            let request = request;
            let mut data = data;
            let tucan = tucan.clone();
            let mut current_session_handle = current_session_handle.to_owned();
            async move {
                if authentication_required && current_session_handle().is_none() {
                    data.set(Err("Not logged in".to_owned()));
                    return;
                }
                loading.set(true);
                match handler(
                    tucan.clone(),
                    current_session_handle(),
                    RevalidationStrategy {
                        max_age: cache_age_seconds,
                        invalidate_dependents: Some(true),
                    },
                    request(),
                )
                .await
                {
                    Ok(response) => {
                        data.set(Ok(Some(response)));
                        loading.set(false);

                        match handler(
                            tucan.clone(),
                            current_session_handle(),
                            RevalidationStrategy {
                                max_age: max_stale_age_seconds,
                                invalidate_dependents: Some(true),
                            },
                            request(),
                        )
                        .await
                        {
                            Ok(response) => data.set(Ok(Some(response))),
                            Err(error) => {
                                info!("ignoring error when refetching: {}", error)
                            }
                        }
                    }
                    Err(error) => {
                        log::error!("{}", error);
                        match error {
                            TucanError::Http(ref req)
                                if req.status() == Some(StatusCode::UNAUTHORIZED) =>
                            {
                                current_session_handle.set(None);
                                data.set(Err("Unauthorized".to_owned()))
                            }
                            TucanError::Timeout | TucanError::AccessDenied => {
                                #[cfg(feature = "direct")]
                                web_extensions_sys::chrome()
                                    .cookies()
                                    .set(web_extensions_sys::SetCookieDetails {
                                        name: Some("id".to_owned()),
                                        partition_key: None,
                                        store_id: None,
                                        url: "https://www.tucan.tu-darmstadt.de".to_owned(),
                                        domain: None,
                                        path: Some("/scripts".to_owned()),
                                        value: None,
                                        expiration_date: Some(0),
                                        http_only: None,
                                        secure: Some(true),
                                        same_site: None,
                                    })
                                    .await;

                                #[cfg(feature = "direct")]
                                web_extensions_sys::chrome()
                                    .cookies()
                                    .set(web_extensions_sys::SetCookieDetails {
                                        name: Some("cnsc".to_owned()),
                                        partition_key: None,
                                        store_id: None,
                                        url: "https://www.tucan.tu-darmstadt.de".to_owned(),
                                        domain: None,
                                        path: Some("/scripts".to_owned()),
                                        value: None,
                                        expiration_date: Some(0),
                                        http_only: None,
                                        secure: Some(true),
                                        same_site: None,
                                    })
                                    .await;

                                if current_session_handle().is_some() {
                                    current_session_handle.set(None);
                                } else {
                                    // some vv urls are not available without authentication
                                    data.set(Err(
                                        "Not accessible without authentication".to_owned()
                                    ));
                                }
                            }
                            _ => {
                                data.set(Err(error.to_string()));
                            }
                        }
                        loading.set(false);
                    }
                }
            }
        });
    }

    let reload = {
        let tucan = tucan.clone();
        Callback::new(move |_e: MouseEvent| {
            if authentication_required && current_session_handle().is_none() {
                data.set(Err("Not logged in".to_owned()));
                return;
            }
            loading.set(true);
            let tucan = tucan.clone();
            spawn(async move {
                match handler(
                    tucan.clone(),
                    current_session_handle(),
                    RevalidationStrategy {
                        max_age: 0,
                        invalidate_dependents: Some(true),
                    },
                    request(),
                )
                .await
                {
                    Ok(response) => {
                        data.set(Ok(Some(response)));
                        loading.set(false);
                    }
                    Err(error) => {
                        log::error!("{}", error);
                        match error {
                            TucanError::Http(ref req)
                                if req.status() == Some(StatusCode::UNAUTHORIZED) =>
                            {
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
            });
        })
    };

    let data = match data() {
        Ok(data) => data,
        Err(error) => {
            return rsx! {
                div { class: "container",
                    div {
                        class: "alert alert-danger d-flex align-items-center mt-2",
                        role: "alert",
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "bi bi-exclamation-triangle-fill flex-shrink-0 me-2",
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            role: "img",
                            "aria-label": "Error:",
                            path { d: "M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" }
                        }
                        div {
                            {"Data loading error: "}
                            {error}
                        }
                    }
                }
            };
        }
    };

    rsx! {
        div { class: "container",
            if loading() {
                div {
                    style: "z-index: 10000",
                    class: "position-fixed top-50 start-50 translate-middle",
                    div { class: "spinner-grow", role: "status",
                        span { class: "visually-hidden", "Loading..." }
                    }
                }
            }
            if let Some(course) = data {
                {render(course.to_owned(), reload)}
            }
        }
    }
}
