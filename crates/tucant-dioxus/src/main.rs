use dioxus::prelude::*;

use crate::navbar::Navbar;

pub mod navbar;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Navbar {}
    }
}
