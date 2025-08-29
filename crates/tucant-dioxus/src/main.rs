use std::panic;

use dioxus::prelude::*;
use tucant_dioxus::{Anonymize, Route};
use tucant_types::LoginResponse;
use wasm_bindgen::prelude::*;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");
const BOOTSTRAP_PATCH_JS: Asset = asset!("/assets/bootstrap.patch.js");

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
        let mut msg = info.to_string();
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

    #[cfg(feature = "web")]
    let launcher = launcher.with_cfg(
        dioxus::web::Config::new().history(std::rc::Rc::new(dioxus::web::HashHistory::new(false))),
    );

    let login_response = tucant_dioxus::login_response().await;
    let launcher = launcher.with_context(login_response);

    #[cfg(feature = "api")]
    let launcher = launcher.with_context(tucant_dioxus::RcTucanType(
        tucant_types::DynTucan::new_arc(tucant_dioxus::api_server::ApiServerTucan::new()),
    ));

    #[cfg(any(feature = "direct", feature = "desktop", feature = "mobile"))]
    let launcher = launcher.with_context(tucant_dioxus::RcTucanType(
        tucant_types::DynTucan::new_arc(tucan_connector::TucanConnector::new().await.unwrap()),
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
        // TODO move this into index.html to prevent flash of unstyled content
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
        script { src: BOOTSTRAP_PATCH_JS }
    }
}
