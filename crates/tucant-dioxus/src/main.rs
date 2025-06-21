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

#[wasm_bindgen(main)]
async fn main() {
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
        let login_response = tucant_dioxus::api_login_response().await;

        dioxus::LaunchBuilder::new()
            .with_context_provider(move || Box::new(login_response.clone()))
            .with_context_provider(|| Box::new(DynTucan::new_rc(ApiServerTucan::new())))
            .launch(App);
    }
    #[cfg(not(any(feature = "direct", feature = "api")))]
    panic!("must activate at least feature `direct` or `api`");
}

#[component]
fn App() -> Element {
    let session = use_context::<Option<LoginResponse>>();
    let mut session = use_signal(|| session);
    provide_context(session);

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router:: {}
        script { src: BOOTSTRAP_JS }
    }
}
