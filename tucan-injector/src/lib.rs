use std::{ops::Deref, rc::Rc};

use log::info;
use tucan_connector::{
    login::LoginResponse,
    registration::index::{anmeldung, AnmeldungRequest, AnmeldungResponse},
    Tucan,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast as _};
use yew::{
    prelude::*,
    suspense::{self, Suspension, SuspensionResult},
};

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
                            .split_once(",")
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
        evil_stuff((&*anmeldung_request).clone())
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
                                move |event| {
                                    anmeldung_request_state.set((&*entry_link).clone());
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
                            move |event| {
                                anmeldung_request_state.set((&*entry_link).clone());
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

#[function_component]
fn App() -> HtmlResult {
    let anmeldung_request: UseStateHandle<AnmeldungRequest> = use_state(|| AnmeldungRequest::new());

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
                    <Content anmeldung_request={(&*anmeldung_request).clone()} anmeldung_request_setter={anmeldung_request.setter()} />
                </Suspense>
            </div>
        </>
    })
}

#[wasm_bindgen(start)]
fn start() {
    console_log::init().unwrap();

    // cargo build --target=wasm32-unknown-unknown
    // wasm-bindgen --out-dir=wasm-bindgen ../target/wasm32-unknown-unknown/debug/tucan_injector.wasm
    // npm run build
    yew::Renderer::<App>::new().render();
}
