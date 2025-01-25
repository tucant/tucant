use std::{ops::Deref as _, rc::Rc};

use tucant_types::{moduledetails::ModuleDetailsRequest, LoginResponse, Tucan};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_context, use_effect_with, use_state, Html, HtmlResult,
    Properties, UseStateHandle,
};

use crate::RcTucanType;

#[derive(Properties, PartialEq)]
pub struct ModuleDetailsProps {
    pub module_details: ModuleDetailsRequest,
}

#[function_component(ModuleDetails)]
pub fn module_details<TucanType: Tucan + 'static>(
    ModuleDetailsProps { module_details }: &ModuleDetailsProps,
) -> HtmlResult {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| None);
    let loading = use_state(|| false);
    let current_session_handle =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(module_details.to_owned(), move |request| {
            if let Some(current_session) = (&*current_session_handle).to_owned() {
                loading.set(true);
                let request = request.clone();
                let data = data.clone();
                spawn_local(async move {
                    let response = tucan
                        .0
                        .module_details(&current_session, request)
                        .await
                        .unwrap();
                    data.set(Some(response));
                    loading.set(false);
                })
            }
        });
    }

    Ok(html! {
        <div class="container">
            { data.as_ref().map(|module| {
                html!{
                    <div>
                        <h1>{ &module.module_id }</h1>

                        <div>{ format!("Registered: {}", if module.registered { "Yes" } else { "No" }) }</div>

                        <div>{ format!("Dozenten: {}", module.dozenten) }</div>

                        <div>{ format!("Display in timetable: {}", module.display_in_timetable) }</div>

                        <div>{ format!("Duration: {}", module.duration) }</div>

                        <div>{ format!("Credits: {}", module.credits) }</div>

                        <div>{ format!("Count of elective courses: {}", module.count_elective_courses) }</div>

                        // TODO FIXME this is dangerous
                        { Html::from_html_unchecked(module.description.join("\n").into()) }

                    </div>
                }
            }).unwrap_or_else(|| html! { if *loading {
                <div style="z-index: 10000" class="position-fixed top-50 start-50 translate-middle">
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            } }) }
        </div>
    })
}
