use std::{cell::RefCell, time::Duration};

use log::info;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

#[wasm_bindgen(main)]
pub async fn main() {
    // From https://github.com/rustwasm/console_error_panic_hook, licensed under MIT and Apache 2.0
    std::panic::set_hook(Box::new(|info| {
        let mut msg = "Service Worker ".to_string();
        msg.push('\n');
        msg.push_str(&info.to_string());
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        error(msg.clone());
    }));
    console_log::init().unwrap();

    log::error!("service worker");

    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    
    let closure: Closure<dyn Fn(MessageEvent)> = Closure::new(move |event: MessageEvent| {
        //info!("Got message at worker {:?}", event.data());
        let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();

        let afewe: () = serde_wasm_bindgen::from_value(event.data()).unwrap();
        //info!("Got result at worker {:?}", result);
        global.post_message(&"hi".into()).unwrap();
    });
    global
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();

    global.post_message(&JsValue::from_str("ready")).unwrap();
}
