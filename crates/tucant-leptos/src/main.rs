pub mod navbar_logged_out;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use log::Level;
use navbar_logged_out::NavbarLoggedOut;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=NavbarLoggedOut />
                <Route path=path!("/users") view=|| view! { <h1>"Not Found"</h1> }/>
                <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
