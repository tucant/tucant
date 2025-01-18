use course_details::CourseDetails;
use js_sys::Object;
use module_details::ModuleDetails;
use navbar::Navbar;
use registration::Registration;
use std::ops::Deref;
use tucant_types::{
    coursedetails::CourseDetailsRequest,
    moduledetails::ModuleDetailsRequest,
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
    LoginRequest, LoginResponse, Tucan,
};

use wasm_bindgen::{JsCast as _, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Link, HashRouter, Routable, Switch};

pub mod navbar;
pub mod navbar_logged_in;
pub mod navbar_logged_out;

pub mod api_server;
pub mod course_details;
#[cfg(feature = "direct")]
pub mod direct;
pub mod module_details;
pub mod registration;
pub mod tauri;

#[cfg(feature = "direct")]
pub async fn direct_login_response() -> Option<LoginResponse> {
    let session_id = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "id".to_owned(),
            partition_key: None,
            store_id: None,
            url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
        })
        .await?
        .value;

    let cnsc = web_extensions_sys::chrome()
        .cookies()
        .get(web_extensions_sys::CookieDetails {
            name: "cnsc".to_owned(),
            url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
            partition_key: None,
            store_id: None,
        })
        .await?
        .value;

    Some(LoginResponse {
        id: session_id.parse().unwrap(),
        cookie_cnsc: cnsc,
    })
}

#[cfg(feature = "api")]
pub async fn api_login_response() -> Option<LoginResponse> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    Some(LoginResponse {
        id: cookie::Cookie::split_parse(&cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "id" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })?
            .parse()
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(&cookie).find_map(|cookie| {
            let cookie = cookie.unwrap();
            if cookie.name() == "cnsc" {
                Some(cookie.value().to_string())
            } else {
                None
            }
        })?,
    })
}

#[function_component(LoginComponent)]
fn login<TucanType: Tucan>() -> HtmlResult {
    let navigator = use_navigator().unwrap();

    let username_value_handle = use_state(String::default);

    let on_username_change = {
        let username_value_handle = username_value_handle.clone();

        Callback::from(move |e: Event| {
            username_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let password_value_handle = use_state(String::default);

    let on_password_change = {
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: Event| {
            password_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let on_submit = {
        let username_value_handle = username_value_handle.clone();
        let password_value_handle = password_value_handle.clone();
        let current_session = current_session.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_value_handle).clone();
            let password = (*password_value_handle).clone();
            let current_session = current_session.clone();
            password_value_handle.set("".to_owned());

            let navigator = navigator.clone();

            spawn_local(async move {
                let response = TucanType::login(LoginRequest { username, password })
                    .await
                    .unwrap();

                web_extensions_sys::chrome()
                    .cookies()
                    .set(web_extensions_sys::SetCookieDetails {
                        name: Some("id".to_owned()),
                        partition_key: None,
                        store_id: None,
                        url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
                        domain: None,
                        path: None,
                        value: Some(response.id.to_string()),
                        expiration_date: None,
                        http_only: None,
                        secure: Some(true),
                        same_site: None,
                    })
                    .await;

                current_session.set(Some(response.clone()));

                navigator.push(&Route::Registration {
                    registration: format!("-N{:015},-N000311,-A", response.id),
                });
            })
        })
    };

    Ok(html! {
        <form onsubmit={on_submit} class="d-flex" role="search">
            <input
                onchange={on_username_change}
                value={(*username_value_handle).clone()}
                required=true
                class="form-control me-2"
                type="username"
                placeholder="TU-ID"
                aria-label="TU-ID"
            />
            <input
                onchange={on_password_change}
                value={(*password_value_handle).clone()}
                required=true
                class="form-control me-2"
                type="password"
                placeholder="Password"
                aria-label="Password"
            />
            <button class="btn btn-outline-success" type="submit">{ "Login" }</button>
        </form>
    })
}

#[function_component(LogoutComponent)]
fn logout<TucanType: Tucan>() -> HtmlResult {
    let current_session_handle =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let on_submit = {
        let current_session_handle = current_session_handle.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let current_session_handle = current_session_handle.clone();

            if let Some(current_session) = (&*current_session_handle).to_owned() {
                spawn_local(async move {
                    TucanType::logout(&current_session).await.unwrap();

                    current_session_handle.set(None);
                });
            }
        })
    };

    Ok(html! {
        <form onsubmit={on_submit} class="d-flex">
            <button class="btn btn-outline-success" type="submit">{ "Logout" }</button>
        </form>
    })
}

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/module-details/:module")]
    ModuleDetails { module: String },
    #[at("/course-details/:course")]
    CourseDetails { course: String },
    #[at("/registration/:registration")]
    Registration { registration: String },
}

fn switch<TucanType: Tucan + 'static>(routes: Route) -> Html {
    match routes {
        Route::Registration { registration } => {
            html! {
                <Registration<TucanType> registration={AnmeldungRequest {arguments: registration}} />
            }
        }
        Route::NotFound => html! { <div>{ "404" }</div> },
        Route::Root => html! { <div>{ "TODO" }</div> },
        Route::ModuleDetails { module } => {
            html! {
                <ModuleDetails<TucanType>
                    module_details={ModuleDetailsRequest {
                arguments: module
            }}
                />
            }
        }
        Route::CourseDetails { course } => {
            html! {
                <CourseDetails<TucanType>
                    course_details={CourseDetailsRequest {
                arguments: course
            }}
                />
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub initial_session: Option<LoginResponse>,
}

#[function_component(App)]
pub fn app<TucanType: Tucan + 'static>(AppProps { initial_session }: &AppProps) -> HtmlResult {
    let ctx = use_state(|| initial_session.clone());

    Ok(html! {
        <>
            <style>{ include_str!("./bootstrap.min.css") }</style>
            <ContextProvider<UseStateHandle<Option<LoginResponse>>> context={ctx.clone()}>
                <HashRouter>
                    <Navbar<TucanType> />
                    <Switch<Route> render={switch::<TucanType>} />
                </HashRouter>
            </ContextProvider<UseStateHandle<Option<LoginResponse>>>>
        </>
    })
}
