use std::rc::Rc;

use dioxus::prelude::*;
use tucant_dioxus::{api_server::ApiServerTucan, navbar::Navbar, rc_tucan_type::RcTucanType, Route};
use tucant_types::Tucan;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| RcTucanType(Rc::new(ApiServerTucan::new())));

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
    }
}
