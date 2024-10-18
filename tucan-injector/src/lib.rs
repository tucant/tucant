use tucan_connector::{
    login::LoginResponse,
    registration::index::{anmeldung, AnmeldungRequest},
    Tucan,
};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
};

async fn evil_stuff() {
    let tucan = Tucan::new().await.unwrap();

    let result = LoginResponse {
        id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
        cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
    };

    let anmeldung_response = anmeldung(&tucan.client, &result, AnmeldungRequest::new())
        .await
        .unwrap();

    println!("{:?}", anmeldung_response);
}

#[function_component]
fn App() -> Html {
    let data = Suspension::from_future(evil_stuff());
    let counter = use_state(|| 3);
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
            {data}
        </div>
    }
}

#[wasm_bindgen(start)]
fn start() {
    // cargo build --target=wasm32-unknown-unknown
    // wasm-bindgen --out-dir=wasm-bindgen ../target/wasm32-unknown-unknown/debug/tucan_injector.wasm
    // npm run build
    yew::Renderer::<App>::new().render();
}
