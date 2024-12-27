use tucant_yew::App;

fn main() {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    //console_log::init().unwrap();

    yew::Renderer::<App>::new().render();
}
