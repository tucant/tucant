use std::{collections::HashMap, panic};

use dioxus::{prelude::*};
use log::warn;
use tucant_dioxus::{RcTucanType, Route};
use tucant_types::LoginResponse;
use wasm_bindgen::prelude::*;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

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

#[tokio::main]
pub async fn main() {
    // logging in here does not work?
    //dioxus::logger::initialize_default();
    //tracing::error!("start of main");
    // From https://github.com/rustwasm/console_error_panic_hook, licensed under MIT and Apache 2.0
    /*panic::set_hook(Box::new(|info| {
        let mut msg = info.to_string();
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        error(msg.clone());
        alert(msg.as_str());
    }));
*/
    //console_log::init().unwrap();

    // maybe this code panics before?

    let launcher = dioxus::LaunchBuilder::new();

    #[cfg(feature = "web")]
    let launcher = launcher.with_cfg(dioxus::web::Config::new().history(std::rc::Rc::new(dioxus::web::HashHistory::new(false))));

    let login_response = tucant_dioxus::login_response().await;

    launcher
        .with_context(login_response)
        .with_context(RcTucanType(tucant_types::DynTucan::new_arc(tucan_connector::TucanConnector::new().await.unwrap())))
        .launch(Test);

    //let connector = RcTucanType(DynTucan::new_rc(tucant_dioxus::api_server::ApiServerTucan::new()));
}

fn Test() -> Element {
    let mut values = use_signal(HashMap::new);
    let mut submitted_values = use_signal(HashMap::new);

    rsx! {
        div { style: "display: flex",
            div { style: "width: 50%",
                h1 { "Form" }

                if !submitted_values.read().is_empty() {
                    h2 { "Submitted! ✅" }
                }

                // The form element is used to create an HTML form for user input
                // You can attach regular attributes to it
                form {
                    id: "cool-form",
                    style: "display: flex; flex-direction: column;",

                    // You can attach a handler to the entire form
                    oninput: move |ev| {
                        println!("Input event: {:#?}", ev);
                        values.set(ev.values());
                        ev.prevent_default();
                    },

                    // On desktop/liveview, the form will not navigate the page - the expectation is that you handle
                    // The form event.
                    // However, if your form doesn't have a submit handler, it might navigate the page depending on the webview.
                    // We suggest always attaching a submit handler to the form.
                    onsubmit: move |ev| {
                        println!("Submit event: {:#?}", ev);
                        submitted_values.set(ev.values());
                    },

                    // Regular text inputs with handlers
                    label { r#for: "username", "Username" }
                    input {
                        r#type: "text",
                        name: "username",
                        oninput: move |ev| {
                            println!("setting username");
                            values.set(ev.values());
                        }
                    }

                    // And then the various inputs that might exist
                    // Note for a value to be returned in .values(), it must be named!

                    label { r#for: "full-name", "Full Name" }
                    input { r#type: "text", name: "full-name" }
                    input { r#type: "text", name: "full-name" }

                    label { r#for: "email", "Email (matching <name>@example.com)" }
                    input { r#type: "email", pattern: ".+@example\\.com", size: "30", required: "true", id: "email", name: "email" }

                    label { r#for: "password", "Password" }
                    input { r#type: "password", name: "password" }

                    label { r#for: "color", "Color" }
                    input { r#type: "radio", checked: true, name: "color", value: "red" }
                    input { r#type: "radio", name: "color", value: "blue" }
                    input { r#type: "radio", name: "color", value: "green" }

                    // Select multiple comes in as a comma separated list of selected values
                    // You should split them on the comma to get the values manually
                    label { r#for: "country", "Country" }
                    select {
                        name: "country",
                        multiple: true,
                        oninput: move |ev| {
                            println!("Input event: {:#?}", ev);
                            println!("Values: {:#?}", ev.value().split(',').collect::<Vec<_>>());
                        },
                        option { value: "usa",  "USA" }
                        option { value: "canada",  "Canada" }
                        option { value: "mexico",  "Mexico" }
                    }

                    // Safari can be quirky with color inputs on mac.
                    // We recommend always providing a text input for color as a fallback.
                    label { r#for: "color", "Color" }
                    input { r#type: "color", value: "#000002", name: "head", id: "head" }

                    // Dates!
                    input {
                        min: "2018-01-01",
                        value: "2018-07-22",
                        r#type: "date",
                        name: "trip-start",
                        max: "2025-12-31",
                        id: "start"
                    }

                    // CHekcboxes
                    label { r#for: "cbox", "Color" }
                    div {
                        label { r#for: "cbox-red", "red" }
                        input { r#type: "checkbox", checked: true, name: "cbox", value: "red", id: "cbox-red" }
                    }
                    div {
                        label { r#for: "cbox-blue", "blue" }
                        input { r#type: "checkbox", name: "cbox", value: "blue", id: "cbox-blue" }
                    }
                    div {
                        label { r#for: "cbox-green", "green" }
                        input { r#type: "checkbox", name: "cbox", value: "green", id: "cbox-green" }
                    }
                    div {
                        label { r#for: "cbox-yellow", "yellow" }
                        input { r#type: "checkbox", name: "cbox", value: "yellow", id: "cbox-yellow" }
                    }

                    // Buttons will submit your form by default.
                    button { r#type: "submit", value: "Submit", "Submit the form" }
                }
            }
            div { style: "width: 50%",
                h1 { "Oninput Values" }
                pre { "{values:#?}" }
            }
        }
        button {
            onclick: move |_| { // This breaks the onsubmit button
                println!("Values: {:#?}", values.read());
            },
            "Log values"
        }
    }
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
        //script { src: BOOTSTRAP_JS }
    }
}
