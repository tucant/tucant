use log::info;
use tucant_yew::App;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast as _,
};
use web_sys::Node;

fn inject() {
    info!("Supported URL detected, injecting...");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let closure = Closure::<dyn Fn(Node)>::new(|element: Node| {
        element
            .parent_node()
            .unwrap()
            .remove_child(&element)
            .unwrap();
    });
    document
        .query_selector_all(r#"link[rel="stylesheet"]"#)
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r"style")
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r"script")
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .query_selector_all(r"[style]")
        .unwrap()
        .for_each(closure.as_ref().unchecked_ref())
        .unwrap();

    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(start)]
fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_log::init().unwrap();

    info!("hi");

    let window = web_sys::window().unwrap();
    let prgname = url::Url::parse(&window.location().href().unwrap())
        .unwrap()
        .query_pairs()
        .find_map(|p| {
            if p.0 == "PRGNAME" {
                Some(p.1.to_string())
            } else {
                None
            }
        });
    let prgname = prgname.as_deref();

    match prgname {
        None => {}
        Some("REGISTRATION") => {
            inject();
        }
        Some(_) => {}
    }
}
