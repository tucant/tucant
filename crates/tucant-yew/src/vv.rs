use std::ops::Deref as _;

use log::info;
use tucant_types::{
    LoginResponse, RevalidationStrategy, Tucan,
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
    vv::{ActionRequest, Vorlesungsverzeichnis},
};
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Html, HtmlResult, MouseEvent, Properties, UseStateHandle, function_component, html, use_context, use_effect_with, use_state};
use yew_router::{hooks::use_navigator, prelude::Link};

use crate::{RcTucanType, Route, vv};

#[derive(Properties, PartialEq)]
pub struct VorlesungsverzeichnisProps {
    pub vv: ActionRequest,
}

#[function_component(VorlesungsverzeichnisComponent)]
pub fn vorlesungsverzeichnis<TucanType: Tucan + 'static>(VorlesungsverzeichnisProps { vv }: &VorlesungsverzeichnisProps) -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    // TODO add invalidation if you press the register button (I think we need to add this to revalidation strategy)

    let data = use_state(|| {
        Ok(Vorlesungsverzeichnis {
            title: "Wird geladen...".to_owned(),
            entries: Vec::new(),
            path: Vec::new(),
            description: Vec::new(),
            veranstaltungen_or_module: Vec::new(),
        })
    });
    let loading = use_state(|| false);
    let current_session = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        let tucan = tucan.clone();
        use_effect_with(vv.clone(), move |vv| {
            if let Some(current_session) = (*current_session).to_owned() {
                loading.set(true);
                let vv = vv.clone();
                let data = data.clone();
                spawn_local(async move {
                    match tucan.0.vv(Some(&current_session), RevalidationStrategy { max_age: 14 * 24 * 60 * 60, invalidate_dependents: Some(true) }, vv.clone()).await {
                        Ok(response) => {
                            data.set(Ok(response));
                            loading.set(false);

                            match tucan.0.vv(Some(&current_session), RevalidationStrategy { max_age: 3 * 24 * 60 * 60, invalidate_dependents: Some(true) }, vv).await {
                                Ok(response) => data.set(Ok(response)),
                                Err(error) => {
                                    info!("ignoring error when refetching: {}", error)
                                }
                            }
                        }
                        Err(error) => {
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
        let current_session = current_session.clone();
        let vv = vv.clone();
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        let tucan = tucan.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(current_session) = (*current_session).to_owned() {
                loading.set(true);
                let vv = vv.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                let loading = loading.clone();
                spawn_local(async move {
                    match tucan.0.vv(Some(&current_session), RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }, vv.clone()).await {
                        Ok(response) => {
                            data.set(Ok(response));
                            loading.set(false);
                        }
                        Err(error) => {
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
            return Ok(html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors
                        <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" />
                        </svg>
                        <div>{ error }</div>
                    </div>
                </div>
            });
        }
    };

    #[expect(unused_parens)]
    Ok(html! {
        <div class="container">
            <h2 class="text-center">{ &data.title }<button onclick={reload} type="button" class="btn btn-light">
                // https://github.com/twbs/icons
                // The MIT License (MIT)
                // Copyright (c) 2019-2024 The Bootstrap Authors
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"/>
                    <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466"/>
                </svg>
            </button></h2>
            <nav style="min-height: 5.5rem" aria-label="breadcrumb">
                <ol class="breadcrumb">
                    { data.path.iter().map(|entry| {
                            html!{<li class="breadcrumb-item"><Link<Route> to={Route::Vorlesungsverzeichnis { vv: entry.1.clone()}}>{entry.0.clone()}</Link<Route>></li>}
                        }).collect::<Html>() }
                </ol>
            </nav>
            // TODO FIXME this is dangerous
            { Html::from_html_unchecked(data.description.join("\n").into()) }
            <h2 class="text-center">{ "Submenus" }</h2>
            <ul class="list-group">
                { data.entries.iter().map(|entry| {
                        html!{<Link<Route> to={Route::Vorlesungsverzeichnis { vv: entry.1.clone()}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route>>}
                    }).collect::<Html>() }
            </ul>
            <h2 class="text-center">{ "Modules and courses" }</h2>
            <ul class="list-group">
                { for data.veranstaltungen_or_module.iter().map(|entry| {
                    html!{
                        <li class="list-group-item">
                            <div class="d-flex w-100 justify-content-between">
                                <h5 class="mb-1"><Link<Route> to={Route::CourseDetails { course: entry.coursedetails_url.clone() }}>{ format!("Kurs {}", entry.title) }</Link<Route>></h5>
                            </div>

                            <div class="d-flex w-100 justify-content-between">
                                <h6 class="mb-1">{ format!("{}", entry.lecturer_name.clone().unwrap_or_default()) }</h6>
                            </div>

                            <h6 class="mb-1">{ format!("{}", entry.date_range.clone().unwrap_or_default()) }</h6>
                        </li>
                    } })
                }
            </ul>
            if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{ "Loading..." }</span>
                    </div>
                </div>
            }
        </div>
    })
}
