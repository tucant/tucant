use key_value_database::Database;
use std::rc::Rc;
use tucan_connector::registration::index::RegistrationState;

use log::info;
use serde::{Deserialize, Serialize};
use tucan_connector::{
    login::LoginResponse,
    registration::index::{anmeldung, AnmeldungRequest, AnmeldungResponse},
    Tucan, TucanError,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    js_sys::{Function, JsString},
    Node,
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

async fn evil_stuff(
    login_response: LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> AnmeldungResponse {
    let mut database = Database::new().await;

    let key = anmeldung_request.arguments.clone();
    if let Some(anmeldung_response) = database.get(&key).await {
        return anmeldung_response;
    }

    let tucan = Tucan::new().await.unwrap();

    let key = anmeldung_request.arguments.clone();
    let anmeldung_response = anmeldung(&tucan, &login_response, anmeldung_request)
        .await
        .unwrap();

    database.put(&key, &anmeldung_response).await;

    info!("{:?}", anmeldung_response);
    anmeldung_response
}

#[hook]
fn use_login_response() -> LoginResponse {
    let location = use_location().unwrap();
    let test: URLFormat = location.query::<URLFormat>().unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

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
                let s = evil_stuff(login_response, anmeldung_request).await;
                data.set(s);
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
                                    <h5 class="mb-1"><a href={ module.map(|module| module.url.clone()).unwrap_or("/notfound".to_owned())}>{ format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default())}</a></h5>
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

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/scripts/mgrqispi.dll")]
    Home,
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
            <style>
                {include_str!("./bootstrap.min.css")}
            </style>
            <script>
                {include_str!("./bootstrap.bundle.min.js")}
            </script>
            <div class="container">
                <h2 class="text-center">{"Registration"}</h2>

                <Content />
            </div>
        </>
    })
}

#[function_component(App)]
fn app() -> HtmlResult {
    Ok(html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    })
}

fn inject() {
    info!("Supported URL detected, injecting");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let closure = Closure::<dyn Fn(Node)>::new(|element: Node| {
        element
            .parent_node()
            .unwrap()
            .remove_child(&element)
            .unwrap();
    });
    document
        .query_selector_all(r#"link[rel="stylesheet"]"#)
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r#"style"#)
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r#"script"#)
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r#"[style]"#)
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();

    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(start)]
fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log::init().unwrap();

    info!("hi");

    let window = web_sys::window().unwrap();
    let prgname = url::Url::parse(&window.location().href().unwrap())
        .unwrap()
        .query_pairs()
        .find_map(|p| {
            if p.0 == "PRGNAME" {
                Some(p.1.to_string())
            } else {
                None
            }
        });
    let prgname = prgname.as_deref();

    match prgname {
        None => {}
        Some("REGISTRATION") => {
            inject();
        }
        Some(_) => {}
    }
}
