pub mod api_server;
pub mod login_component;
pub mod navbar;
pub mod navbar_logged_out;
pub mod rc_tucan_type;

use api_server::ApiServerTucan;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use log::Level;
use navbar::Navbar;
use navbar_logged_out::NavbarLoggedOut;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    provide_context(ApiServerTucan::new());

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Navbar />
                <Route path=path!("/users") view=|| view! { <h1>"Not Found"</h1> } />
                <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> } />
            </Routes>
        </Router>
    }
}

fn main() {
    console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
