use tucant_yew::{login_response, App, AppProps, CurrentSession};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(main)]
async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

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
