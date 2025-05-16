use std::marker::PhantomData;

use leptos::prelude::*;
use tucant_types::Tucan;

use crate::rc_tucan_type::RcTucanType;

#[component]
pub fn LoginComponent() -> impl IntoView {
    /*let tucan: RcTucanType<dyn Tucan> = use_context().expect("no ctx found");

    let username_value_handle = use_state(String::default);

    let on_username_change = {
        let username_value_handle = username_value_handle.clone();

        Callback::from(move |e: Event| {
            username_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let password_value_handle = use_state(String::default);

    let on_password_change = {
        let password_value_handle = password_value_handle.clone();

        Callback::from(move |e: Event| {
            password_value_handle.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());
        })
    };

    let current_session = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let on_submit = {
        let username_value_handle = username_value_handle.clone();
        let password_value_handle = password_value_handle.clone();
        let current_session = current_session.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_value_handle).clone();
            let password = (*password_value_handle).clone();
            let current_session = current_session.clone();
            password_value_handle.set("".to_owned());

            let tucan = tucan.clone();

            spawn_local(async move {
                let response = tucan.0.login(LoginRequest { username, password }).await.unwrap();

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
            })
        })
    };*/

    // onsubmit={on_submit}
    // onchange={on_username_change} value={(*username_value_handle).clone()}
    // onchange={on_password_change} value={(*password_value_handle).clone()}
    view! {
        <form class="d-flex">
            <input id="login-username" required=true class="form-control me-2" type="username" placeholder="TU-ID" aria-label="TU-ID" autocomplete="current-username" />
            <input id="login-password" required=true class="form-control me-2" type="password" placeholder="Password" aria-label="Password" autocomplete="current-password" />
            <button class="btn btn-outline-success" type="submit" id="login-button">{ "Login" }</button>
        </form>
    }
}
