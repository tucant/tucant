use dioxus::prelude::*;
use tucant_types::{LoginRequest, LoginResponse, Tucan};

use crate::rc_tucan_type::RcTucanType;

#[component]
fn LoginComponent<TucanType: Tucan + 'static>() -> Element {
    let tucan: RcTucanType<TucanType> = use_context();

    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let current_session = use_context::<Signal<Option<LoginResponse>>>();

    let on_submit = move |e: Event<FormData>| {
        let tucan = tucan.clone();
        let username = username.clone();
        let password = password.clone();
        let current_session = current_session.clone();
        async move  {
         let tucan = tucan.clone();
        let username = username.clone();
        let mut password = password.clone();
        let mut current_session = current_session.clone();

        e.prevent_default();
        password.set("".to_owned());

        let response = tucan.0.login(LoginRequest { username: username(), password: password() }).await.unwrap();

        #[cfg(feature = "direct")]
        web_extensions_sys::chrome()
            .cookies()
            .set(web_extensions_sys::SetCookieDetails {
                name: Some("id".to_owned()),
                partition_key: None,
                store_id: None,
                url: "https://www.tucan.tu-darmstadt.de/scripts/".to_owned(),
                domain: None,
                path: None,
                value: Some(response.id.to_string()),
                expiration_date: None,
                http_only: None,
                secure: Some(true),
                same_site: None,
            })
            .await;

        current_session.set(Some(response.clone()));
    }};

    rsx! {
        form { onsubmit: on_submit, class: "d-flex",
            input { id:"login-username", value: "{username}", oninput: move |event| username.set(event.value()), required:true, class:"form-control me-2", r#type:"username", placeholder: "TU-ID", "aria-label": "TU-ID", autocomplete:"current-username"}
            input { id:"login-password", value: "{password}", oninput: move |event| password.set(event.value()), required:true, class:"form-control me-2", r#type:"password", placeholder:"Password", "aria-label":"Password", autocomplete:"current-password"}
            button { class:"btn btn-outline-success", r#type:"submit", id: "login-button", "Login" }
        }
    }
}
