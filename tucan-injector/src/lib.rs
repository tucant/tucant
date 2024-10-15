use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[wasm_bindgen(start)]
fn start() {
    // cargo build --target=wasm32-unknown-unknown
    // wasm-bindgen --out-dir=dist --target=web --omit-default-module-path ../target/wasm32-unknown-unknown/debug/tucan_injector.wasm
    // npm run build
    yew::Renderer::<App>::new().render();
}
