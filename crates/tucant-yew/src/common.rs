use yew::{hook, Callback, MouseEvent, UseStateHandle};
use tucant_types::Tucan;
use crate::RcTucanType;
use yew::use_context;
use tucant_types::{coursedetails::{CourseDetailsRequest, CourseDetailsResponse}, LoginResponse, RevalidationStrategy, TucanError};
use log::info;
use yew::{platform::spawn_local, use_effect_with, use_state};

pub struct DataLoaderReturn<O> {
    pub data: UseStateHandle<Result<Option<O>, String>>,
    pub loading: UseStateHandle<bool>,
    pub reload: Callback<MouseEvent>
}

#[hook]
pub fn use_data_loader<TucanType: Tucan + 'static, I: Clone + PartialEq + 'static, O: 'static>(handler: impl AsyncFn(RcTucanType<TucanType>, LoginResponse, RevalidationStrategy, I) -> Result<O, TucanError> + Copy + 'static, request: I) -> DataLoaderReturn<O> {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| Ok(None));
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        use_effect_with(request.to_owned(), move |request| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let request = request.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                spawn_local(async move {
                    match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: 14 * 24 * 60 * 60, invalidate_dependents: Some(true) }, request.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);

                            match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: 4 * 24 * 60 * 60, invalidate_dependents: Some(true) }, request).await {
                                Ok(response) => data.set(Ok(Some(response))),
                                Err(error) => {
                                    info!("ignoring error when refetching: {}", error)
                                }
                            }
                        }
                        Err(error) => {
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        });
    }

    let reload = {
        let current_session = current_session_handle.clone();
        let course_details = request.clone();
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        let tucan = tucan.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(current_session) = (*current_session).to_owned() {
                loading.set(true);
                let course_details = course_details.clone();
                let data = data.clone();
                let tucan = tucan.clone();
                let loading = loading.clone();
                spawn_local(async move {
                    match handler(tucan.clone(), current_session.clone(), RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }, course_details.clone()).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);
                        }
                        Err(error) => {
                            data.set(Err(error.to_string()));
                            loading.set(false);
                        }
                    }
                })
            } else {
                data.set(Err("Not logged in".to_owned()));
            }
        })
    };

    DataLoaderReturn { data, loading, reload }
}
