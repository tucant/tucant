use log::info;
use tucant_yew::App;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log::init().unwrap();

    info!("hi");

    yew::Renderer::<App>::new().render();
}
