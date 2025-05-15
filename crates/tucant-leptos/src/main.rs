use leptos::prelude::*;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
