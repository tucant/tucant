use std::ops::Deref as _;

use tucant_types::{
    coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, LoginResponse, Tucan,
};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_context, use_effect_with, use_state, Html, HtmlResult,
    Properties, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetailsRequest,
}

#[function_component(CourseDetails)]
pub fn course_details<TucanType: Tucan>(
    CourseDetailsProps { course_details }: &CourseDetailsProps,
) -> HtmlResult {
    let data = use_state(|| None);
    let loading = use_state(|| false);
    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with(course_details.to_owned(), move |request| {
            loading.set(true);
            let request = request.clone();
            let data = data.clone();
            spawn_local(async move {
                let response =
                    TucanType::course_details(&current_session.deref().clone().unwrap(), request)
                        .await
                        .unwrap();
                data.set(Some(response));
                loading.set(false);
            })
        });
    }

    Ok(html! {
        <div class="container">
            { data.as_ref().map(|course| {
                html!{
                    <div>


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
