use std::rc::Rc;

use log::info;
use serde::{Deserialize, Serialize};
use tucan_connector::{
    login::LoginResponse,
    registration::index::{anmeldung, AnmeldungRequest, AnmeldungResponse},
    Tucan,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _,
};
use web_sys::{js_sys::Function, Node};
use yew::{
    prelude::*,
    suspense::{self, SuspensionResult},
};
use yew_router::{
    hooks::{use_location, use_navigator, use_route},
    BrowserRouter, Routable, Switch,
};

async fn evil_stuff(
    login_response: LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> AnmeldungResponse {
    let tucan = Tucan::new().await.unwrap();

    let anmeldung_response = anmeldung(&tucan.client, &login_response, anmeldung_request)
        .await
        .unwrap();

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

#[hook]
fn use_anmeldung(anmeldung_request: AnmeldungRequest) -> SuspensionResult<AnmeldungResponse> {
    let login_response = use_login_response();

    let s = suspense::use_future_with(anmeldung_request, |anmeldung_request| {
        evil_stuff(login_response, (*anmeldung_request).clone())
    })?;
    Ok((*s).clone())
}

#[derive(Properties, PartialEq)]
pub struct AnmeldungRequestProps {
    anmeldung_request: AnmeldungRequest,
}

#[function_component(Content)]
fn content(props: &AnmeldungRequestProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let data = use_anmeldung(props.anmeldung_request.clone())?;

    Ok(html! {
        <>
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    {
                        data.path.into_iter().map(|entry| {
                            let anmeldung_request_cb = Callback::from({
                                let navigator = navigator.clone();
                                let entry_link = Rc::new(entry.1.clone());
                                move |_event| {
                                    // TODO add id
                                    navigator.push_with_query(&Route::Home, &URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: entry_link.arguments.clone() }).unwrap();
                                }
                            });
                            html!{<li class="breadcrumb-item"><a href="#" onclick={anmeldung_request_cb}>{entry.0}</a></li>}
                        }).collect::<Html>()
                    }
                </ol>
            </nav>

            <h2 class="text-center">{"Submenus"}</h2>

            <ul class="list-group">
                {
                    data.submenus.into_iter().map(|entry| {
                        let anmeldung_request_cb = Callback::from({
                            let navigator = navigator.clone();
                            let entry_link = Rc::new(entry.1.clone());
                            move |_event| {
                                // TODO add id
                                navigator.push_with_query(&Route::Home, &URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: entry_link.arguments.clone() }).unwrap();
                            }
                        });
                        html!{<a href="#" onclick={anmeldung_request_cb} class="list-group-item list-group-item-action">{ format!("{}", entry.0) }</a>}
                    }).collect::<Html>()
                }
            </ul>

            <h2 class="text-center">{"Modules and courses"}</h2>

            <ul class="list-group">
                {
                    data.entries.into_iter().map(|entry| {
                        html!{<li class="list-group-item">{ format!("{}", entry.module.map(|module| module.name).unwrap_or_default()) }</li>}
                    }).collect::<Html>()
                }
            </ul>

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
        "REGISTRATION" => {
            let anmeldung_request = AnmeldungRequest {
                arguments: ",".to_owned() + test.ARGUMENTS.split_once(',').unwrap().1,
            };
            Ok(html! { <Registration {anmeldung_request} /> })
        }
        _ => Ok(html! { <div>{"unknown"}</div> }),
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <SwitchInner></SwitchInner> },
        Route::NotFound => html! { <div>{"404"}</div> },
    }
}

#[derive(Serialize, Deserialize)]
struct URLFormat {
    APPNAME: String,
    PRGNAME: String,
    ARGUMENTS: String,
}

#[function_component(Registration)]
fn registration(props: &AnmeldungRequestProps) -> HtmlResult {
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

                <Suspense {fallback}>
                    <Content anmeldung_request={(props.anmeldung_request).clone()} />
                </Suspense>
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
