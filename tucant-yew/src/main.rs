use js_sys::{global, Reflect};
use log::info;
use tucant_yew::{
    api_login_response, api_server::ApiServerTucan, direct::DirectTucan, direct_login_response,
    App, AppProps,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::window;
use yew::set_custom_panic_hook;

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
    set_custom_panic_hook(Box::new(|info| {
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

    info!("ewfwfwfefwf");

    if Reflect::get(&global(), &JsValue::from_str("chrome")).is_ok() {
        let login_response = direct_login_response().await;
        yew::Renderer::<App<DirectTucan>>::with_props(AppProps {
            initial_session: login_response,
        })
        .render();
    }
    if 1 == 1 {
        let login_response = api_login_response().await;
        yew::Renderer::<App<ApiServerTucan>>::with_props(AppProps {
            initial_session: login_response,
        })
        .render();
    }
}
