#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc, clippy::multiple_crate_versions)]

mod app;
mod module;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
