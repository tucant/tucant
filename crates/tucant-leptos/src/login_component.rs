use std::marker::PhantomData;

use leptos::{html::Input, prelude::*};
use tucant_types::{LoginRequest, Tucan};

use crate::rc_tucan_type::RcTucanType;

#[component]
pub fn LoginComponent() -> impl IntoView {
    let username_ref = NodeRef::<Input>::new();
    let password_ref = NodeRef::<Input>::new();

    let add_todo_action = Action::new(|(username, password): &(String, String)| async move {
        let response = tucan.0.login(LoginRequest { username: username.to_owned(), password: password.to_owned() }).await.unwrap();

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
    });

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let username = username_ref.get().unwrap();
            let password = password_ref.get().unwrap();
            add_todo_action.dispatch(username.value());
        } class="d-flex">
            <input node_ref=username_ref id="login-username" required=true class="form-control me-2" type="username" placeholder="TU-ID" aria-label="TU-ID" autocomplete="current-username" />
            <input node_ref=password_ref id="login-password" required=true class="form-control me-2" type="password" placeholder="Password" aria-label="Password" autocomplete="current-password" />
            <button class="btn btn-outline-success" type="submit" id="login-button">{ "Login" }</button>
        </form>
    }
}
