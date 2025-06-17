use dioxus::prelude::*;

use crate::navbar::Navbar;

pub mod navbar;
pub mod navbar_logged_out;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Navbar {}
        script { src: BOOTSTRAP_JS }
    }
}
