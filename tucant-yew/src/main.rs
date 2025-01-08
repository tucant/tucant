use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
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

    #[cfg(feature = "direct")]
    if js_sys::Reflect::get(
        &js_sys::global(),
        &wasm_bindgen::JsValue::from_str("chrome"),
    )
    .is_ok()
    {
        let login_response = tucant_yew::direct_login_response().await;
        yew::Renderer::<tucant_yew::App<tucant_yew::direct::DirectTucan>>::with_props(
            tucant_yew::AppProps {
                initial_session: login_response,
            },
        )
        .render();
    }
    #[cfg(feature = "api")]
    {
        let login_response = tucant_yew::api_login_response().await;
        yew::Renderer::<tucant_yew::App<tucant_yew::api_server::ApiServerTucan>>::with_props(
            tucant_yew::AppProps {
                initial_session: login_response,
            },
        )
        .render();
    }
}
