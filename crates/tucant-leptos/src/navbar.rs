use leptos::prelude::*;
use log::error;
use std::ops::Deref;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan, TucanError};

use crate::{login_component::LoginComponent, navbar_logged_out::NavbarLoggedOut};

#[component]
pub fn Navbar(set_session: WriteSignal<Option<LoginResponse>>) -> impl IntoView {
    view! {
        <>
            <nav class="navbar navbar-expand-xl bg-body-tertiary">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#/">
                        {"TUCaN't"}
                    </a>
                    <button
                        class="navbar-toggler"
                        type="button"
                        data-bs-toggle="collapse"
                        data-bs-target="#navbarSupportedContent"
                        aria-controls="navbarSupportedContent"
                        aria-expanded="false"
                        aria-label="Toggle navigation"
                    >
                        <span class="navbar-toggler-icon" />
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-xl-0">

                            <NavbarLoggedOut />
                        // }
                        </ul>
                        <LoginComponent />

                    </div>
                </div>
            </nav>
        </>
    }
}
