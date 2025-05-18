use std::sync::Arc;

use leptos::prelude::*;
use tucant_types::{LoginResponse, Tucan};

use crate::api_server::ApiServerTucan;

#[component]
pub fn LogoutComponent(set_session: WriteSignal<Option<LoginResponse>>) -> impl IntoView {
    let tucan = use_context::<Arc<ApiServerTucan>>().unwrap();
    let session = use_context::<ReadSignal<Option<LoginResponse>>>().unwrap();

    let logout_action = Action::new_local(move |(): &()| {
        let tucan = tucan.clone();
        async move {
            let response = tucan.logout(&session.get().unwrap()).await.unwrap();

            set_session.set(None);
        }
    });

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            logout_action.dispatch(());
        } class="d-flex">
            <button id="logout-button" class="btn btn-outline-success" type="submit">{ "Logout" }</button>
        </form>
    }
}
