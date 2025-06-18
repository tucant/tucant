use dioxus::prelude::*;
use tucant_dioxus::{navbar::Navbar, Route};
use tucant_types::Tucan;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/bootstrap.bundle.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App<TucanType: Tucan + 'static>(input: TucanType) -> Element {
    use_context_provider(|| input);

    rsx! {
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
        script { src: BOOTSTRAP_JS }
    }
}
