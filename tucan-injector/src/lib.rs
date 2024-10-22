use std::rc::Rc;

use indexed_db::Factory;
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
    BrowserRouter, Routable, Switch,
};

async fn evil_stuff(
    login_response: LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> AnmeldungResponse {
    // https://github.com/rustwasm/wasm-bindgen/issues/3798

    let factory = Factory::<TucanError>::get().unwrap();

    let db = factory
        .open("database", 1, |evt| async move {
            let db = evt.database();
            let store = db.build_object_store("store").create()?;

            // You can also add objects from this callback
            store
                .add_kv(&JsString::from("foo"), &JsString::from("foo"))
                .await?;

            Ok(())
        })
        .await
        .unwrap();

    let key = anmeldung_request.arguments.clone();
    let result = db
        .transaction(&["store"])
        .run(|t| async move {
            let store = t.object_store("store")?;
            let value = store.get(&key.into()).await.unwrap();

            Ok(value)
        })
        .await
        .unwrap();

    if let Some(result) = result {
        return serde_wasm_bindgen::from_value(result).unwrap();
    }

    let tucan = Tucan::new().await.unwrap();

    let key = anmeldung_request.arguments.clone();
    let anmeldung_response = anmeldung(&tucan.client, &login_response, anmeldung_request)
        .await
        .unwrap();

    let value = serde_wasm_bindgen::to_value(&anmeldung_response).unwrap();
    db.transaction(&["store"])
        .rw()
        .run(|t| async move {
            let store = t.object_store("store")?;
            let value = store.put_kv(&key.into(), &value).await.unwrap();

            Ok(value)
        })
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

#[derive(Properties, PartialEq)]
pub struct AnmeldungRequestProps {
    anmeldung_request: AnmeldungRequest,
}

#[function_component(Content)]
fn content(props: &AnmeldungRequestProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let login_response = use_login_response();

    let data = use_state(|| AnmeldungResponse {
        path: vec![],
        submenus: vec![],
        entries: vec![],
        additional_information: vec![],
    });
    {
        let data = data.clone();
        use_effect_with(props.anmeldung_request.clone(), move |anmeldung_request| {
            let anmeldung_request = anmeldung_request.clone();
            let data = data.clone();
            spawn_local(async move {
                let s = evil_stuff(login_response, anmeldung_request).await;
                data.set(s);
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
                            let anmeldung_request_cb = Callback::from({
                                let navigator = navigator.clone();
                                let entry_link = Rc::new(entry.1.clone());
                                move |_event| {
                                    navigator.push_with_query(&Route::Home, &URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: format!("-N{:015}{}", login_response.id, entry_link.arguments.clone())}).unwrap();
                                }
                            });
                            html!{<li class="breadcrumb-item"><a href="#" onclick={anmeldung_request_cb}>{entry.0.clone()}</a></li>}
                        }).collect::<Html>()
                    }
                </ol>
            </nav>

            <h2 class="text-center">{"Submenus"}</h2>

            <ul class="list-group">
                {
                    data.submenus.iter().map(|entry| {
                        let anmeldung_request_cb = Callback::from({
                            let navigator = navigator.clone();
                            let entry_link = Rc::new(entry.1.clone());
                            move |_event| {
                                navigator.push_with_query(&Route::Home, &URLFormat { APPNAME: "CampusNet".to_owned(), PRGNAME: "REGISTRATION".to_owned(), ARGUMENTS: format!("-N{:015}{}", login_response.id, entry_link.arguments.clone())}).unwrap();
                            }
                        });
                        html!{<a href="#" onclick={anmeldung_request_cb} class="list-group-item list-group-item-action">{ format!("{}", entry.0) }</a>}
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

                                <span class="text-body-secondary"><a class="btn btn-primary mb-1" role="button" href={ format!("{}", module.map(|module| module.registration_button_link.clone().unwrap_or_default()).unwrap_or_default()) }>{"Zum Modul anmelden"}</a></span>

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

                                                <span class="text-body-secondary"><a class="btn btn-primary mb-1" role="button" href={ format!("{}", course.1.registration_button_link.clone().unwrap_or_default()) }>{"Zum Kurs anmelden"}</a></span>
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

                <Content anmeldung_request={(props.anmeldung_request).clone()} />
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
