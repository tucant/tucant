mod app;
mod module;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
