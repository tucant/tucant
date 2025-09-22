use std::{panic, sync::Arc};

use dioxus::prelude::*;
use js_sys::Function;
use log::info;
use serde::{Serialize, de::DeserializeOwned};
use tucan_plus_dioxus::{Anonymize, BOOTSTRAP_JS, BOOTSTRAP_PATCH_JS, Route, wait_for_worker};
use tucan_types::LoginResponse;
use wasm_bindgen::prelude::*;
use web_sys::{AddEventListenerOptions, MessageEvent, Worker, WorkerOptions, WorkerType};

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

// https://github.com/tauri-apps/wry
// https://github.com/tauri-apps/tao/blob/5ac00b57ad3f5c5c7135dde626cb90bc1ad469dc/src/platform_impl/android/ndk_glue.rs#L236

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(main))]
#[cfg_attr(not(target_arch = "wasm32"), tokio::main)]
pub async fn main() {
    // From https://github.com/rustwasm/console_error_panic_hook, licensed under MIT and Apache 2.0
    #[cfg(feature = "web")]
    panic::set_hook(Box::new(|info| {
        let mut msg = "Version: ".to_string();
        msg.push_str(git_version::git_version!());
        msg.push('\n');
        msg.push_str(&info.to_string());
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        error(msg.clone());
        alert(msg.as_str());
    }));
    #[cfg(feature = "web")]
    console_log::init().unwrap();

    let anonymize = {
        #[cfg(feature = "direct")]
        {
            // TODO we need to update this when you update the value in the extension
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"anonymize".into(), &false.into()).unwrap();
            let storage = web_extensions_sys::chrome().storage().sync();
            let result = storage.get(&obj).await.unwrap();
            js_sys::Reflect::get(&result, &"anonymize".into())
                .unwrap()
                .as_bool()
                .unwrap()
        }
        #[cfg(not(feature = "direct"))]
        false
    };

    let launcher = dioxus::LaunchBuilder::new();

    let worker = fragile::Fragile::new(wait_for_worker().await);
    //let response: String = send_message(&worker, &"test").await;

    let launcher = launcher.with_context(worker);

    #[cfg(feature = "web")]
    let launcher = launcher.with_cfg(
        dioxus::web::Config::new().history(std::rc::Rc::new(dioxus::web::HashHistory::new(false))),
    );

    let login_response = tucan_plus_dioxus::login_response().await;
    let launcher = launcher.with_context(login_response);

    #[cfg(feature = "api")]
    let launcher = launcher.with_context(tucan_plus_dioxus::RcTucanType::new(
        tucan_types::DynTucan::new_arc(tucan_plus_dioxus::api_server::ApiServerTucan::new()),
    ));

    #[cfg(any(feature = "direct", feature = "desktop", feature = "mobile"))]
    let launcher = launcher.with_context(tucan_plus_dioxus::RcTucanType::new(
        tucan_types::DynTucan::new_arc(tucan_connector::TucanConnector::new().await.unwrap()),
    ));

    let launcher = launcher.with_context(Anonymize(anonymize));

    launcher.launch(App);
}

#[component]
fn App() -> Element {
    let login_response: Option<LoginResponse> = use_context();
    let login_response = use_signal(|| login_response);
    provide_context(login_response);
    rsx! {
        Router::<Route> {
        }
        script {
            src: BOOTSTRAP_JS,
        }
        script {
            src: BOOTSTRAP_PATCH_JS,
        }
    }
}
