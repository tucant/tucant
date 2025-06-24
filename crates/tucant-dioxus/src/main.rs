use std::panic;

use dioxus::{prelude::*};
use log::warn;
use tucant_dioxus::{RcTucanType, Route};
use tucant_types::LoginResponse;
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
        use std::rc::Rc;

        use dioxus::web::{Config, HashHistory};
        use tucant_types::DynTucan;

        let history_provider: Rc<dyn History> = Rc::new(HashHistory::default());
        let login_response = tucant_dioxus::direct_login_response().await;
        let connector = RcTucanType(DynTucan::new_rc(tucan_connector::TucanConnector::new().await.unwrap()));

        let vdom = VirtualDom::new_with_props(App, AppProps { login_response, connector });
        vdom.provide_root_context(history_provider);
        dioxus::web::launch::launch_virtual_dom(vdom, Config::new());
    }
    #[cfg(feature = "api")]
    {
        use std::rc::Rc;

        use dioxus::web::{Config, HashHistory};
        use tucant_types::DynTucan;

        let history_provider: Rc<dyn History> = Rc::new(HashHistory::default());
        let login_response = tucant_dioxus::api_login_response().await;
        let connector = RcTucanType(DynTucan::new_rc(tucant_dioxus::api_server::ApiServerTucan::new()));

        let vdom = VirtualDom::new_with_props(App, AppProps { login_response, connector });
        vdom.provide_root_context(history_provider);
        dioxus::web::launch::launch_virtual_dom(vdom, Config::new());
    }
    #[cfg(not(any(feature = "direct", feature = "api")))]
    panic!("must activate at least feature `direct` or `api`");
}

#[component]
fn App(login_response: Option<LoginResponse>, connector: RcTucanType) -> Element {
    let session = use_signal(|| login_response);
    provide_context(session);
    provide_context(connector);

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
    }
}
