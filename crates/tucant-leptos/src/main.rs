pub mod api_server;
pub mod login_component;
pub mod navbar;
pub mod navbar_logged_in;
pub mod navbar_logged_out;
pub mod rc_tucan_type;

use std::sync::Arc;

use api_server::ApiServerTucan;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use log::Level;
use navbar::Navbar;
use navbar_logged_out::NavbarLoggedOut;
use tucant_types::LoginResponse;

#[component]
fn App() -> impl IntoView {
    provide_context(Arc::new(ApiServerTucan::new()));

    let (session, set_session) = signal(None::<LoginResponse>);
    provide_context(session);

    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=move || view! { <Navbar set_session=set_session /> } />
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
