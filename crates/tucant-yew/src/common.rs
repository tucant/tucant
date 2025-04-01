use log::info;
use tucant_types::{LoginResponse, RevalidationStrategy, Tucan};
use yew::{hook, platform::spawn_local, use_context, use_effect_with, use_state, Callback, MouseEvent, UseStateHandle};
use std::rc::Rc;
use crate::RcTucanType;
use tucant_types::{mlsstart::MlsStart, TucanError};

pub struct DataLoaderReturn<T> {
    pub data: UseStateHandle<Result<Option<T>, String>>,
    pub loading: UseStateHandle<bool>,
    pub reload: Callback<MouseEvent>
}

pub trait WhatTheHell<TucanType: Tucan + 'static, T> {
    fn execute(tucan: &TucanType, request: &LoginResponse, revalidation_strategy: RevalidationStrategy) -> impl std::future::Future<Output = Result<T, TucanError>>;
}

#[hook]
pub fn use_data_loader<TucanType: Tucan + 'static, T: 'static, W: WhatTheHell<TucanType, T>>() -> DataLoaderReturn<T> {
    let tucan: RcTucanType<TucanType> = use_context().expect("no ctx found");

    let data = use_state(|| Ok(None));
    let loading = use_state(|| false);
    let current_session_handle = use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session_handle = current_session_handle.clone();
        let tucan = tucan.clone();
        use_effect_with((), move |()| {
            if let Some(current_session) = (*current_session_handle).to_owned() {
                loading.set(true);
                let data = data.clone();
                let tucan = tucan.clone();
                spawn_local(async move {
                    match W::execute(&tucan.0, &current_session, RevalidationStrategy { max_age: 14 * 24 * 60 * 60, invalidate_dependents: Some(true) }).await {
                        Ok(response) => {
                            data.set(Ok(Some(response)));
                            loading.set(false);

                            match W::execute(&tucan.0, &current_session, RevalidationStrategy { max_age: 4 * 24 * 60 * 60, invalidate_dependents: Some(true) }).await {
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
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        let tucan = tucan.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(current_session) = (*current_session).to_owned() {
                loading.set(true);
                let data = data.clone();
                let tucan = tucan.clone();
                let loading = loading.clone();
                spawn_local(async move {
                    match W::execute(&tucan.0, &current_session, RevalidationStrategy { max_age: 0, invalidate_dependents: Some(true) }).await {
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