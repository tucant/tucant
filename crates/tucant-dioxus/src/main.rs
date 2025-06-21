use std::{panic, rc::Rc};

use dioxus::prelude::*;
use log::warn;
use tucant_dioxus::{api_server::ApiServerTucan, navbar::Navbar, rc_tucan_type::RcTucanType, Route};
use tucant_types::{DynTucan, LoginRequest, LoginResponse, Tucan};
use wasm_bindgen::prelude::*;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    fn alert(s: &str);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}


fn main() {
    // From https://github.com/rustwasm/console_error_panic_hook, licensed under MIT and Apache 2.0
    panic::set_hook(Box::new(|info| {
        let mut msg = info.to_string();
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        error(msg.clone());
        alert(msg.as_str());
    }));

    console_log::init().unwrap();

    warn!("main");

    #[cfg(feature = "direct")]
    if js_sys::Reflect::get(&js_sys::global(), &wasm_bindgen::JsValue::from_str("chrome")).is_ok() {
        let login_response = tucant_yew::direct_login_response().await;
        yew::Renderer::<tucant_yew::App<tucan_connector::TucanConnector>>::with_props(tucant_yew::AppProps {
            initial_session: login_response,
            tucan: tucant_yew::RcTucanType(std::rc::Rc::new(tucan_connector::TucanConnector::new().await.unwrap())),
        })
        .render();
    }
    #[cfg(feature = "api")]
    {
        let login_response = tucant_yew::api_login_response().await;
        yew::Renderer::<tucant_yew::App<tucant_yew::api_server::ApiServerTucan>>::with_props(tucant_yew::AppProps {
            initial_session: login_response,
            tucan: tucant_yew::RcTucanType(std::rc::Rc::new(tucant_yew::api_server::ApiServerTucan::new())),
        })
        .render();
    }
    #[cfg(not(any(feature = "direct", feature = "api")))]
    panic!("must activate at least feature `direct` or `api`");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut session = use_signal(|| None::<LoginResponse>);

    use_context_provider(|| session);
    use_context_provider(|| DynTucan::new_rc(ApiServerTucan::new()));

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
    }
}
