use std::rc::Rc;

use dioxus::prelude::*;
use tucant_dioxus::{api_server::ApiServerTucan, navbar::Navbar, rc_tucan_type::RcTucanType, Route};
use tucant_types::{DynTucan, LoginRequest, LoginResponse, Tucan};

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut session = use_signal(|| None::<LoginResponse>);

    use_context_provider(|| session);
    use_context_provider(|| DynTucan::new_rc(ApiServerTucan::new()));

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
    }
}
