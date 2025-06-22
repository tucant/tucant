use std::rc::Rc;

use dioxus::prelude::*;
use tucant_types::{DynTucan, LoginRequest, LoginResponse, Tucan};

use crate::RcTucanType;

#[component]
pub fn LoginComponent() -> Element {
    let tucan: RcTucanType = use_context();

    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut current_session = use_context::<Signal<Option<LoginResponse>>>();

    let on_submit = move |e: FormEvent| {
        let tucan = tucan.clone();
        async move  {
        let tucan = tucan.clone();

        
        let password_string = password();
        password.set("".to_owned());

        // TODO don't unwrap here but handle error
        let response = tucan.login(LoginRequest { username: username(), password: password_string}).await.unwrap();

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
