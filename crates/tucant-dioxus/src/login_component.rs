use dioxus::prelude::*;
use tucant_types::{LoginRequest, LoginResponse, Tucan};

use crate::RcTucanType;

#[component]
pub fn LoginComponent() -> Element {
    let tucan: RcTucanType = use_context();

    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut current_session = use_context::<Signal<Option<LoginResponse>>>();

    let on_submit = move |_: FormEvent| {
        let tucan = tucan.clone();
        async move {
            let tucan = tucan.clone();

            let password_string = password();
            password.set("".to_owned());

            match tucan.login(LoginRequest { username: username(), password: password_string }).await {
                Ok(response) => {
                    #[cfg(feature = "direct")]
                    web_extensions_sys::chrome()
                        .cookies()
                        .set(web_extensions_sys::SetCookieDetails {
                            name: Some("id".to_owned()),
                            partition_key: None,
                            store_id: None,
                            url: "https://www.tucan.tu-darmstadt.de".to_owned(),
                            domain: None,
                            path: Some("/scripts".to_owned()),
                            value: Some(response.id.to_string()),
                            expiration_date: None,
                            http_only: None,
                            secure: Some(true),
                            same_site: None,
                        })
                        .await;

                    current_session.set(Some(response.clone()));
                }
                Err(e) => {
                    tracing::error!("{e}");
                }
            };
        }
    };

    rsx! {
        form { onsubmit: on_submit, class: "d-flex",
            input {
                id: "login-username",
                value: "{username}",
                oninput: move |event| username.set(event.value()),
                required: true,
                class: "align-self-start form-control me-2",
                r#type: "username",
                placeholder: "TU-ID",
                "aria-label": "TU-ID",
                autocomplete: "current-username",
            }
            div {
                class: "align-self-start input-group has-validation",
                input {
                    id: "login-password",
                    value: "{password}",
                    oninput: move |event| password.set(event.value()),
                    required: true,
                    class: "form-control me-2 is-invalid",
                    r#type: "password",
                    placeholder: "Password",
                    "aria-label": "Password",
                    autocomplete: "current-password",
                }
                div {
                    class: "invalid-feedback",
                    "Wrong password"
                }
            }
            button {
                class: "align-self-start btn btn-outline-success",
                r#type: "submit",
                id: "login-button",
                "Login"
            }
        }
    }
}
