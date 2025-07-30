use dioxus::prelude::*;
use tucant_types::{LoginRequest, LoginResponse, Tucan};

use crate::RcTucanType;

#[component]
pub fn LoginComponent() -> Element {
    let tucan: RcTucanType = use_context();

    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let mut current_session = use_context::<Signal<Option<LoginResponse>>>();

    let on_submit = move |_: FormEvent| {
        let tucan = tucan.clone();
        async move {
            let tucan = tucan.clone();

            let password_string = password();
            password.set("".to_owned());

            loading.set(true);
            match tucan
                .login(LoginRequest {
                    username: username(),
                    password: password_string,
                })
                .await
            {
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

                    #[cfg(any(feature = "desktop", feature = "mobile"))]
                    keyring::Entry::new("tucant", "session")
                        .unwrap()
                        .set_password(&serde_json::to_string(&response).unwrap())
                        .unwrap();

                    current_session.set(Some(response.clone()));
                    error_message.set(None);
                }
                Err(e) => {
                    tracing::error!("{e}");
                    error_message.set(Some(e.to_string()));
                }
            };
            loading.set(false);
        }
    };

    let is_invalid = if error_message().is_some() {
        "is-invalid"
    } else {
        ""
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
                disabled: loading(),
            }
            div { class: "align-self-start input-group has-validation",
                input {
                    id: "login-password",
                    value: "{password}",
                    oninput: move |event| password.set(event.value()),
                    required: true,
                    class: "form-control me-2 {is_invalid}",
                    r#type: "password",
                    placeholder: "Password",
                    "aria-label": "Password",
                    "aria-describedby": "password-feedback",
                    autocomplete: "current-password",
                    disabled: loading(),
                }
                if let Some(error_message) = error_message() {
                    div { id: "password-feedback", class: "invalid-feedback", "{error_message}" }
                }
            }
            button {
                class: "align-self-start btn btn-outline-success",
                r#type: "submit",
                id: "login-button",
                disabled: loading(),
                "Login"
            }
        }
    }
}
