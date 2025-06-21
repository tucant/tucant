use std::rc::Rc;

use dioxus::prelude::*;
use tucant_types::{DynTucan, LoginResponse, Tucan};

#[component]
pub fn LogoutComponent() -> Element {
    let tucan: Rc<DynTucan> = use_context();

    let current_session_handle = use_context::<Signal<Option<LoginResponse>>>();

    // https://github.com/DioxusLabs/dioxus/issues/4303
    let on_submit = move |e: FormEvent| {
        let mut current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();

        async move {
            if let Some(current_session) = current_session_handle() {
                tucan.logout(&current_session).await.unwrap();

                current_session_handle.set(None);
            }
        }
    };

    rsx! {
        form { onsubmit: on_submit, class: "d-flex",
            button { id: "logout-button", class: "btn btn-outline-success", type: "submit", { "Logout" } }
        }
    }
}
