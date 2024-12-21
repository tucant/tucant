use key_value_database::Database;
use std::rc::Rc;
use tucant_types::{
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
    LoginRequest, LoginResponse,
};
use url::Url;

use log::info;
use serde::{Deserialize, Serialize};

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    js_sys::{Function, JsString},
    HtmlInputElement, Node,
};
use yew::{
    prelude::*,
    suspense::{self, SuspensionResult},
};
use yew_router::{
    hooks::{use_location, use_navigator, use_route},
    prelude::Link,
    BrowserRouter, Routable, Switch,
};

// http://localhost:1420/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N218653534694253,-N000311,-A

#[hook]
fn use_login_response() -> LoginResponse {
    let location = use_location().unwrap();
    let test: URLFormat = location.query::<URLFormat>().unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    // here

    LoginResponse {
        id: test
            .ARGUMENTS
            .split_once(',')
            .unwrap()
            .0
            .trim_start_matches("-N")
            .parse()
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "cnsc" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })
            .unwrap(),
    }
}

#[derive(Properties, PartialEq)]
pub struct AnmeldungRequestProps {
    anmeldung_request: AnmeldungRequest,
}

#[function_component(Content)]
fn content() -> HtmlResult {
    let location = use_location().unwrap();
    let test: URLFormat = location.query::<URLFormat>().unwrap();
    let anmeldung_request = AnmeldungRequest {
        arguments: ",".to_owned() + test.ARGUMENTS.split_once(',').unwrap().1,
    };

    let login_response = use_login_response();

    let data = use_state(|| AnmeldungResponse {
        path: vec![],
        submenus: vec![],
        entries: vec![],
        additional_information: vec![],
    });
    let loading = use_state(|| false);
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(anmeldung_request.clone(), move |anmeldung_request| {
            loading.set(true);
            let anmeldung_request = anmeldung_request.clone();
            let data = data.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();
                let mut url = Url::parse("http://localhost:1420/api/v1/registration").unwrap();
                url.path_segments_mut()
                    .unwrap()
                    .push(&anmeldung_request.arguments);
                let response: AnmeldungResponse =
                    client.get(url).send().await.unwrap().json().await.unwrap();
                data.set(response);
                loading.set(false);
            })
        });
    }

    let login_response = use_login_response();

    Ok(html! {
        <>

            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    {
                        data.path.iter().map(|entry| {
                            html!{<li class="breadcrumb-item"><Link<Route, URLFormat> to={Route::Home} query={URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: format!("-N{:015}{}", login_response.id, entry.1.arguments.clone())}}>{entry.0.clone()}</Link<Route, URLFormat>></li>}
                        }).collect::<Html>()
                    }
                </ol>
            </nav>

            <h2 class="text-center">{"Submenus"}</h2>

            <ul class="list-group">
                {
                    data.submenus.iter().map(|entry| {
                        html!{<Link<Route, URLFormat> to={Route::Home} query={URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: format!("-N{:015}{}", login_response.id, entry.1.arguments.clone())}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route, URLFormat>>}
                    }).collect::<Html>()
                }
            </ul>

            <h2 class="text-center">{"Modules and courses"}</h2>

            <ul class="list-group">
                {
                    for data.entries.iter().map(|entry| {
                        let module = entry.module.as_ref();
                        html!{
                            <li class="list-group-item">
                                <div class="d-flex w-100 justify-content-between">
                                    <h5 class="mb-1"><a href={ module.map(|module| module.url.clone().arguments).unwrap_or("/notfound".to_owned())}>{ format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default())}</a></h5>
                                    <small class="text-body-secondary">{ format!("Anmeldung bis {}", module.map(|module| module.date.clone()).unwrap_or_default()) }</small>
                                </div>
                                <div class="d-flex w-100 justify-content-between">
                                    <h6 class="mb-1">{ format!("{}", module.map(|module| module.lecturer.clone().unwrap_or_default()).unwrap_or_default()) }</h6>
                                    <small class="text-body-secondary">{ module.map(|module| "Teilnehmerlimit ".to_owned() + &module.limit_and_size).unwrap_or_default() }</small>
                                </div>

                                {
                                    module.map(|module| {
                                        match &module.registration_button_link {
                                            RegistrationState::Unknown => html! { },
                                            RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={unregister_link.clone()}>{"Vom Modul abmelden"}</a> },
                                            RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={register_link.clone()}>{"Zum Modul anmelden"}</a> },
                                        }
                                    })
                                }

                                <ul class="list-group">
                                {
                                    for entry.courses.iter().map(|course| {
                                        html! {
                                            <li class="list-group-item">
                                                <div class="d-flex w-100 justify-content-between">
                                                    <h5 class="mb-1"><a href={ course.1.url.clone() }>{ format!("Kurs {} {}", course.1.id, course.1.name) }</a></h5>
                                                    <small class="text-body-secondary">{ format!("Anmeldung bis {}", course.1.registration_until) }</small>
                                                </div>

                                                <div class="d-flex w-100 justify-content-between">
                                                    <h6 class="mb-1">{ format!("{}", course.1.lecturers.clone().unwrap_or_default()) }</h6>
                                                    <small class="text-body-secondary">{ ("Teilnehmerlimit ".to_owned() + &course.1.limit_and_size) }</small>
                                                </div>

                                                <h6 class="mb-1">{ format!("{}", course.1.begin_and_end.clone().unwrap_or_default()) }</h6>

                                                {
                                                    match &course.1.registration_button_link {
                                                        RegistrationState::Unknown => html! { },
                                                        RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={unregister_link.clone()}>{"Vom Kurs abmelden"}</a> },
                                                        RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={register_link.clone()}>{"Zum Kurs anmelden"}</a> },
                                                    }
                                                }
                                            </li>
                                        }
                                    })
                                }
                                </ul>
                            </li>
                        }
                    })
                }
            </ul>

            if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            }
        </>
    })
}

#[function_component(LoginPage)]
fn login() -> HtmlResult {
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

    let on_submit = {
        let username = (*username_value_handle).clone();
        let password = (*password_value_handle).clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // TODO submit
            info!("logging in {}", username);
            let username = username.clone();
            let password = password.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();

                // maybe abstract this away into an api client crate that can optionally skip the whole server?
                let response: LoginResponse = client
                    .post("http://localhost:1420/api/v1/login")
                    .json(&LoginRequest { username, password })
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                html_document
                    .set_cookie(&format!("id={}; SameSite=Strict", response.id))
                    .unwrap();
                html_document
                    .set_cookie(&format!("cnsc={}; SameSite=Strict", response.cookie_cnsc))
                    .unwrap();
            })
        })
    };

    Ok(html! {
        <div class="container">

    <form onsubmit={on_submit}>
        <h1 class="h3 mb-3 fw-normal">{"Please sign in"}</h1>

        <div class="form-floating">
            <input required=true onchange={on_username_change} value={(*username_value_handle).clone()} type="username" class="form-control" id="floatingInput" placeholder="TU-ID" />
            <label for="floatingInput">{"TU-ID"}</label>
        </div>
        <div class="form-floating">
            <input required=true onchange={on_password_change} value={ (*password_value_handle).clone()} type="password" class="form-control" id="floatingPassword" placeholder="Password" />
            <label for="floatingPassword">{"Password"}</label>
        </div>

        <button class="btn btn-primary w-100 py-2" type="submit">{"Sign in"}</button>
        </form>
        </div>
      })
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/scripts/mgrqispi.dll")]
    Home,
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(SwitchInner)]
fn switch_inner() -> HtmlResult {
    let location = use_location().unwrap();
    let test: URLFormat = location.query::<URLFormat>().unwrap();

    match test.PRGNAME.as_str() {
        "REGISTRATION" => Ok(html! { <Registration /> }),
        _ => Ok(html! { <div>{"unknown"}</div> }),
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <SwitchInner></SwitchInner> },
        Route::NotFound => html! { <div>{"404"}</div> },
        Route::Root => html! { <LoginPage /> },
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct URLFormat {
    APPNAME: String,
    PRGNAME: String,
    ARGUMENTS: String,
}

#[function_component(Registration)]
fn registration() -> HtmlResult {
    let fallback = html! {
        <>
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    <li class="breadcrumb-item"><a href="#" class="placeholder-glow"><span class="placeholder placeholder-xs">{"Some path that is cool"}</span></a></li>
                    <li class="breadcrumb-item"><a href="#" class="placeholder-glow"><span class="placeholder placeholder-xs">{"Some path that is cool"}</span></a></li>
                    <li class="breadcrumb-item"><a href="#" class="placeholder-glow"><span class="placeholder placeholder-xs">{"Some path that is cool"}</span></a></li>
                </ol>
            </nav>

            <h2 class="text-center">{"Submenus"}</h2>

            <ul class="list-group">
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
            </ul>

            <h2 class="text-center">{"Modules and courses"}</h2>

            <ul class="list-group">
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
                <li class="list-group-item placeholder-glow"><span class="placeholder w-100"></span></li>
            </ul>
        </>
    };

    Ok(html! {
        <>
            <div class="container">
                <h2 class="text-center">{"Registration"}</h2>

                <Content />
            </div>
        </>
    })
}

#[function_component(App)]
pub fn app() -> HtmlResult {
    Ok(html! {
        <>
        <style>
            {include_str!("./bootstrap.min.css")}
        </style>
        <script>
            {include_str!("./bootstrap.bundle.min.js")}
        </script>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </>
    })
}
