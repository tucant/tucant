use dioxus::prelude::*;

use crate::navbar::Navbar;

pub mod navbar;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Navbar {}
    }
}
