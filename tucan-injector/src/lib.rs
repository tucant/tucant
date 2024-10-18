use log::info;
use tucan_connector::{
    login::LoginResponse,
    registration::index::{anmeldung, AnmeldungRequest},
    Tucan,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast as _};
use yew::{
    prelude::*,
    suspense::{Suspension, SuspensionResult},
};

async fn evil_stuff() {
    let tucan = Tucan::new().await.unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();

    let result = LoginResponse {
        id: url::Url::parse(&window.location().href().unwrap())
            .unwrap()
            .query_pairs()
            .find_map(|param| {
                if param.0 == "ARGUMENTS" {
                    Some(
                        param
                            .1
                            .split_once(",")
                            .unwrap()
                            .0
                            .trim_start_matches("-N")
                            .parse()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
            .unwrap(),
        cookie_cnsc: cookie::Cookie::split_parse(cookie)
            .find_map(|cookie| {
                let cookie = cookie.unwrap();
                if cookie.name() == "cnsc" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })
            .unwrap(),
    };

    let anmeldung_response = anmeldung(&tucan.client, &result, AnmeldungRequest::new())
        .await
        .unwrap();

    info!("{:?}", anmeldung_response);
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
    console_log::init().unwrap();

    // cargo build --target=wasm32-unknown-unknown
    // wasm-bindgen --out-dir=wasm-bindgen ../target/wasm32-unknown-unknown/debug/tucan_injector.wasm
    // npm run build
    yew::Renderer::<App>::new().render();
}
