use api_server::ApiServerTucan;
use key_value_database::Database;
use std::{ops::Deref, rc::Rc};
use tauri::TauriTucan;
use tucant_types::{
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
    LoginRequest, LoginResponse, Tucan,
};
use url::Url;
use web_extensions_sys::CookieDetails;
use yew_autoprops::autoprops;

use log::info;
use serde::{Deserialize, Serialize};

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _, JsValue,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    js_sys::{Function, JsString, Reflect},
    HtmlInputElement, Node,
};
use yew::{
    prelude::*,
    suspense::{self, SuspensionResult},
};
use yew_router::{
    hooks::{use_location, use_navigator, use_route},
    prelude::{Link, Redirect},
    BrowserRouter, HashRouter, Routable, Switch,
};
mod api_server;
#[cfg(feature = "direct")]
mod direct;
mod tauri;

// TODO FIXME don't do it this way because of feature unification
#[cfg(feature = "tauri")]
type TucanType = TauriTucan;
#[cfg(feature = "direct")]
type TucanType = direct::DirectTucan;
#[cfg(feature = "api")]
type TucanType = ApiServerTucan;

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentSession {
    pub id: String,
    pub cnsc: String,
}

#[cfg(feature = "direct")]
pub async fn login_response() -> LoginResponse {
    {
        let session_id = web_extensions_sys::chrome()
            .storage()
            .local()
            .get(&JsValue::from_str("sessionId"))
            .await
            .unwrap();

        info!("session_id: {:?}", session_id);
        let session_id =
            js_sys::Reflect::get(&session_id, &JsValue::from_str("sessionId")).unwrap();
        info!("session_id: {:?}", session_id);
        let session_id = session_id.as_string().unwrap();
        info!("session_id: {:?}", session_id);

        let cnsc = web_extensions_sys::chrome()
            .cookies()
            .get(CookieDetails {
                name: "cnsc".to_owned(),
                url: "https://www.tucan.tu-darmstadt.de/scripts".to_owned(),
                partition_key: None,
                store_id: None,
            })
            .await
            .unwrap();

        LoginResponse {
            id: session_id.parse().unwrap(),
            cookie_cnsc: cnsc.value,
        }
    }
}

#[cfg(feature = "api")]
pub async fn login_response() -> LoginResponse {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    LoginResponse {
        id: cookie::Cookie::split_parse(&cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "id" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })
            .unwrap()
            .parse()
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(&cookie)
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
    registration: AnmeldungRequest,
}

#[function_component(Registration)]
fn registration(AnmeldungRequestProps { registration }: &AnmeldungRequestProps) -> HtmlResult {
    let data = use_state(|| {
        Ok(AnmeldungResponse {
            path: vec![],
            submenus: vec![],
            entries: vec![],
            additional_information: vec![],
        })
    });
    let loading = use_state(|| false);
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(registration.clone(), move |anmeldung_request| {
            loading.set(true);
            let anmeldung_request = anmeldung_request.clone();
            let data = data.clone();
            spawn_local(async move {
                match TucanType::anmeldung(login_response().await, anmeldung_request).await {
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
        });
    }
    let current_session = use_context::<Option<CurrentSession>>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    let data = match data.deref() {
        Ok(data) => data,
        Err(error) => {
            return Ok(html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2" width="16" height="16" viewBox="0 0 16 16" role="img" aria-label="Error:">
                        <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
                        </svg>
                        <div>
                            {error}
                        </div>
                    </div>
                </div>
            })
        }
    };

    if (data.submenus.len() == 1
        && data.additional_information.is_empty()
        && data.entries.is_empty()
        && !*loading)
    {
        navigator.replace(&Route::Registration {
            registration: format!("{}", data.submenus[0].1.arguments.clone()),
        });
        return Ok(html! { <></> });
    }

    Ok(html! {
        <div class="container">
            <h2 class="text-center">{ "Registration" }</h2>
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    { data.path.iter().map(|entry| {
                            html!{<li class="breadcrumb-item"><Link<Route> to={Route::Registration { registration: format!("{}", entry.1.arguments.clone())}}>{entry.0.clone()}</Link<Route>></li>}
                        }).collect::<Html>() }
                </ol>
            </nav>
            <h2 class="text-center">{ "Submenus" }</h2>
            <ul class="list-group">
                { data.submenus.iter().map(|entry| {
                        html!{<Link<Route> to={Route::Registration { registration: format!("{}", entry.1.arguments.clone())}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route>>}
                    }).collect::<Html>() }
            </ul>
            <h2 class="text-center">{ "Modules and courses" }</h2>
            <ul class="list-group">
                { for data.entries.iter().map(|entry| {
                        let module = entry.module.as_ref();
                        html!{
                            <li class="list-group-item">
                                <div class="d-flex w-100 justify-content-between">
                                    <h5 class="mb-1"><Link<Route> to={Route::ModuleDetails { module: module.map(|module| module.url.clone().arguments).unwrap_or("/notfound".to_owned())}}>{ format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default())}</Link<Route>></h5>
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
                                            RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Vom Modul abmelden"}</a> },
                                            RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone())}>{"Zum Modul anmelden"}</a> },
                                        }
                                    })
                                }
                                <ul class="list-group">
                                {
                                    for entry.courses.iter().map(|course| {
                                        html! {
                                            <li class="list-group-item">
                                                <div class="d-flex w-100 justify-content-between">
                                                    <h5 class="mb-1"><a href={ format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N{:015}{}",  current_session.as_ref().map(|s| s.id.as_str()).unwrap_or("1"), course.1.url.clone()) }>{ format!("Kurs {} {}", course.1.id, course.1.name) }</a></h5>
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
                                                        RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Vom Kurs abmelden"}</a> },
                                                        RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",register_link.clone())}>{"Zum Kurs anmelden"}</a> },
                                                    }
                                                }
                                            </li>
                                        }
                                    })
                                }
                                </ul>
                            </li>
                        }
                    }) }
            </ul>
            if *loading {
                <div
                    style="z-index: 10000"
                    class="position-fixed top-50 start-50 translate-middle"
                >
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{ "Loading..." }</span>
                    </div>
                </div>
            }
        </div>
    })
}

#[function_component(LoginComponent)]
fn login() -> HtmlResult {
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

    let on_submit = {
        let username_value_handle = username_value_handle.clone();
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_value_handle).clone();
            let password = (*password_value_handle).clone();
            password_value_handle.set("".to_owned());

            let navigator = navigator.clone();

            spawn_local(async move {
                let response = TucanType::login(LoginRequest { username, password })
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

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/module-details/:module")]
    ModuleDetails { module: String },
    #[at("/registration/:registration")]
    Registration { registration: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Registration { registration } => {
            html! { <Registration registration={AnmeldungRequest {arguments: registration}} /> }
        }
        Route::NotFound => html! { <div>{ "404" }</div> },
        Route::Root => html! { <div>{"TODO"}</div> },
        Route::ModuleDetails { module } => {
            html! {
                <ModuleDetails
                    module_details={ModuleDetailsRequest {
                arguments: module
            }}
                />
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ModuleDetailsProps {
    module_details: ModuleDetailsRequest,
}

#[function_component(ModuleDetails)]
fn module_details(ModuleDetailsProps { module_details }: &ModuleDetailsProps) -> HtmlResult {
    let data = use_state(|| None);
    let loading = use_state(|| false);
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(module_details.to_owned(), move |request| {
            loading.set(true);
            let request = request.clone();
            let data = data.clone();
            spawn_local(async move {
                let response = TucanType::module_details(&login_response().await, request)
                    .await
                    .unwrap();
                data.set(Some(response));
                loading.set(false);
            })
        });
    }

    Ok(html! {
        <div class="container">
            { data.as_ref().map(|module| {
                html!{
                    <div>
                        <h1>{ &module.module_id }</h1>

                        <div>{ format!("Registered: {}", if module.registered { "Yes" } else { "No" }) }</div>

                        <div>{ format!("Dozenten: {}", module.dozenten) }</div>

                        <div>{ format!("Display in timetable: {}", module.display_in_timetable) }</div>

                        <div>{ format!("Duration: {}", module.duration) }</div>

                        <div>{ format!("Credits: {}", module.credits) }</div>

                        <div>{ format!("Count of elective courses: {}", module.count_elective_courses) }</div>

                        // TODO FIXME this is dangerous
                        { Html::from_html_unchecked(module.description.join("\n").into()) }

                    </div>
                }
            }).unwrap_or_else(|| html! { if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            } }) }
        </div>
    })
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-xl bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">{ "TUCaN't" }</a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                    aria-controls="navbarSupportedContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon" />
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-xl-0">
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Aktuelles" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Aktuelles" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Nachrichten" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "VV" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Vorlesungsverzeichnis" }
                                    </a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Lehrveranstaltungssuche" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Raumsuche" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Aktuell - Wintersemester 2024/25" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Vorlesungsverzeichnis Gasthörer_innen WiSe 2024/25" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Vorlesungsverzeichnis des SoSe 2024" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Vorlesungsverzeichnis des WiSe 2023/24" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Archiv" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Stundenplan" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Stundenplan" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Tagesansicht" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Wochenansicht" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Monatsansicht" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Export" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Veranstaltungen" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Veranstaltungen" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Module" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Meine Veranstaltungen" }
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Wahlbereiche" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#/registration/,-N000311,-A" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Anmeldung" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                                        { "Mein aktueller Anmeldestatus" }
                                    </a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Prüfungen" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Prüfungen" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Prüfungen" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Mein Prüfungsplan" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Semesterergebnisse" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Leistungsspiegel" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Service" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Service" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Persönliche Daten" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Dokumente" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Anträge" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Sperren" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                { "Bewerbung" }
                            </a>
                            <ul class="dropdown-menu">
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Bewerbung" }</a>
                                </li>
                                <li><hr class="dropdown-divider" /></li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Bewerbung" }</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Meine Dokumente" }</a>
                                </li>
                            </ul>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">{ "Hilfe" }</a>
                        </li>
                    </ul>
                    <LoginComponent />
                </div>
            </div>
        </nav>
    }
}

#[autoprops]
#[function_component(App)]
pub fn app(initial_session: &Option<CurrentSession>) -> HtmlResult {
    let ctx = use_state(|| initial_session.clone());

    Ok(html! {
        <>
            <style>{ include_str!("./bootstrap.min.css") }</style>
            <ContextProvider<Option<CurrentSession>> context={(*ctx).clone()}>
                <HashRouter>
                    <Navbar />
                    <Switch<Route> render={switch} />
                </HashRouter>
            </ContextProvider<Option<CurrentSession>>>
        </>
    })
}
