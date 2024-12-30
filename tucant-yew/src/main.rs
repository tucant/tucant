use tucant_yew::{login_response, App, AppProps, CurrentSession};
use wasm_bindgen::prelude::wasm_bindgen;

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
    std::panic::set_hook(Box::new(|info| {
        let mut msg = info.to_string();
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        //error(msg.clone());
        alert(msg.as_str());
    }));

    console_log::init().unwrap();

    let login_response = login_response().await;

    yew::Renderer::<App>::with_props(AppProps {
        initial_session: Some(CurrentSession {
            id: login_response.id.to_string(),
            cnsc: login_response.cookie_cnsc,
        }),
    })
    .render();
}
