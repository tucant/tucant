use std::rc::Rc;

use tucant_types::{LoggedInHead, LoginResponse, Tucan};
use wasm_bindgen_futures::spawn_local;
use yew::{
    classes, function_component, html, use_context, use_effect_with, use_state, Html,
    UseStateHandle,
};

use crate::{
    navbar_logged_in::NavbarLoggedIn, navbar_logged_out::NavbarLoggedOut, LoginComponent,
    LogoutComponent, RcTucanType,
};

#[function_component(Navbar)]
pub fn navbar<TucanType: Tucan + 'static>() -> Html {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect_with((&*current_session).clone(), move |current_session| {
            if let Some(current_session) = current_session.to_owned() {
                spawn_local(async move {
                    match tucan.0.after_login(&current_session).await {
                        Ok(response) => {
                            data.set(Some(response));
                        }
                        Err(error) => {
                            panic!("{}", error)
                        }
                    }
                })
            }
        });
    }

    html! {
        <nav class="navbar navbar-expand-xl bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="#/">{ "TUCaN't" }</a>
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
                        if let Some(current_session) = &*current_session {
                            <NavbarLoggedIn
                                current_session={current_session.clone()}
                                data={(&*data).clone()}
                            />
                        } else {
                            <NavbarLoggedOut />
                        }
                    </ul>
                    if !current_session.is_some() {
                        <LoginComponent<TucanType> />
                    } else {
                        <LogoutComponent<TucanType> />
                    }
                </div>
            </div>
        </nav>
    }
}
