use std::rc::Rc;

use log::info;
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
use yew_router::{BrowserRouter, Routable, Switch};

async fn evil_stuff(anmeldung_request: AnmeldungRequest) -> AnmeldungResponse {
    let tucan = Tucan::new().await.unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    let result = LoginResponse {
        id: url::Url::parse(&window.location().href().unwrap())
            .unwrap()
            .query_pairs()
            .find_map(|param| {
                if param.0 == "ARGUMENTS" {
                    Some(
                        param
                            .1
                            .split_once(',')
                            .unwrap()
                            .0
                            .trim_start_matches("-N")
                            .parse()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
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
    };

    let anmeldung_response = anmeldung(&tucan.client, &result, anmeldung_request)
        .await
        .unwrap();

    info!("{:?}", anmeldung_response);
    anmeldung_response
}

#[hook]
fn use_anmeldung(anmeldung_request: AnmeldungRequest) -> SuspensionResult<AnmeldungResponse> {
    let s = suspense::use_future_with(anmeldung_request, |anmeldung_request| {
        evil_stuff((*anmeldung_request).clone())
    })?;
    Ok((*s).clone())
}

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    anmeldung_request: AnmeldungRequest,
    anmeldung_request_setter: UseStateSetter<AnmeldungRequest>,
}

#[function_component(Content)]
fn content(props: &ContentProps) -> HtmlResult {
    let data = use_anmeldung(props.anmeldung_request.clone())?;
    Ok(html! {
        <>
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    {
                        data.path.into_iter().map(|entry| {
                            let anmeldung_request_cb = Callback::from({
                                let anmeldung_request_state = props.anmeldung_request_setter.clone();
                                let entry_link = Rc::new(entry.1.clone());
                                move |_event| {
                                    anmeldung_request_state.set((*entry_link).clone());
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
                            let anmeldung_request_state = props.anmeldung_request_setter.clone();
                            let entry_link = Rc::new(entry.1.clone());
                            move |_event| {
                                anmeldung_request_state.set((*entry_link).clone());
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
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Registration /> },
    }
}

#[function_component(Registration)]
fn registration() -> HtmlResult {
    let anmeldung_request: UseStateHandle<AnmeldungRequest> = use_state(AnmeldungRequest::new);

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
                    <Content anmeldung_request={(*anmeldung_request).clone()} anmeldung_request_setter={anmeldung_request.setter()} />
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
