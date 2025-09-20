use log::info;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[wasm_bindgen(main)]
fn main() {
    console_log::init().unwrap();

    let global = js_sys::global().unchecked_into::<web_sys::DedicatedWorkerGlobalScope>();
    let closure: Closure<dyn Fn(MessageEvent)> = Closure::new(|event: MessageEvent| {
        info!("Got message");
    });
    global
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .unwrap();
}
