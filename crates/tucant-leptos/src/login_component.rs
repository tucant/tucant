use std::{marker::PhantomData, sync::Arc};

use leptos::{html::Input, prelude::*};
use tucant_types::{LoginRequest, LoginResponse, Tucan};

use crate::{api_server::ApiServerTucan, rc_tucan_type::RcTucanType};

#[component]
pub fn LoginComponent(set_session: WriteSignal<Option<LoginResponse>>) -> impl IntoView {
    let username_ref = NodeRef::<Input>::new();
    let password_ref = NodeRef::<Input>::new();
    let tucan = use_context::<Arc<ApiServerTucan>>().unwrap();

    let login_action = Action::new_local(move |(username, password): &(String, String)| {
        let username = username.to_owned();
        let password = password.to_owned();
        let tucan = tucan.clone();
        async move {
            let response = tucan.login(LoginRequest { username, password }).await.unwrap();

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

            set_session.set(Some(response));
        }
    });

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                let username = username_ref.get().unwrap();
                let password = password_ref.get().unwrap();
                login_action.dispatch((username.value(), password.value()));
            }
            class="d-flex"
        >
            <input
                node_ref=username_ref
                id="login-username"
                required=true
                class="form-control me-2"
                type="username"
                placeholder="TU-ID"
                aria-label="TU-ID"
                autocomplete="current-username"
            />
            <input
                node_ref=password_ref
                id="login-password"
                required=true
                class="form-control me-2"
                type="password"
                placeholder="Password"
                aria-label="Password"
                autocomplete="current-password"
            />
            <button class="btn btn-outline-success" type="submit" id="login-button">
                {"Login"}
            </button>
        </form>
    }
}
