use std::{ops::Add, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use dioxus::prelude::*;
use futures::{FutureExt as _, StreamExt, stream::BoxStream};
use time::{Month, macros::offset};
use tucan_plus_planning::{compress};
use tucan_types::{DynTucan, LoginResponse, RevalidationStrategy, Tucan, TucanError, registration::{AnmeldungRequest, AnmeldungResponse}};

use crate::RcTucanType;

// breath first for progress?
// maybe us a channel?
// atomic for progress?
#[expect(clippy::manual_async_fn)]
pub fn recursive_anmeldung<'a, 'b: 'a>(
    tucan: &'a DynTucan<'static>,
    login_response: &'b LoginResponse,
    mut atomic_current: SyncSignal<usize>,
    mut atomic_total: SyncSignal<usize>,
    anmeldung_request: AnmeldungRequest,
) -> BoxStream<'a, AnmeldungResponse> {
    tucan.anmeldung(
        login_response,
        RevalidationStrategy::cache(),
        anmeldung_request.clone(),
    ).into_stream().flat_map(move |element: Result<AnmeldungResponse, TucanError>| {
        let element = element.unwrap();
        atomic_total += element.submenus.len();
        atomic_current += 1;
        futures::stream::iter(element
            .submenus.clone()
            .into_iter())
            .flat_map(move |entry| {
                recursive_anmeldung(tucan, login_response, atomic_current, atomic_total, entry.1.clone())
            })
    }).boxed()
}

#[component]
pub fn FetchAnmeldung() -> Element {
    let mut result: Signal<Vec<(String, Vec<u8>)>> = use_signal(Vec::new);
    let tucan: RcTucanType = use_context();
    let current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    let mut loading = use_signal(|| false);
    let mut progresses = use_signal(Vec::<(SyncSignal<usize>, SyncSignal<usize>)>::new);

    let onclick = move |_event| {
        let tucan = tucan.clone();
        async move {
            loading.set(true);
            let anmeldung_response = tucan
                .anmeldung(
                    &current_session_handle().unwrap(),
                    RevalidationStrategy::cache(),
                    AnmeldungRequest::default(),
                )
                .await
                .unwrap();
            let datetime = time::OffsetDateTime::now_utc();
            let datetime = datetime.to_offset(offset!(+2));
            let date = datetime.date();
            let registration_sose = Month::March <= date.month() && date.month() <= Month::August;
            let semester = if registration_sose { "sose" } else { "wise" };

            for course_of_study in anmeldung_response.studiumsauswahl {
                log::info!("start");
                let session = current_session_handle().unwrap();
                let atomic_current = use_signal_sync(|| 0);
                let atomic_total = use_signal_sync(|| 1);
                spawn({
                    let mut result = result.clone();
                    let tucan = tucan.clone();
                    let atomic_current = atomic_current.clone();
                    let atomic_total = atomic_total.clone();
                    async move {
                        let atomic_current = atomic_current.clone();
                        let atomic_total = atomic_total.clone();
                        let response = recursive_anmeldung(
                            &tucan.0,
                            &session,
                            atomic_current,
                            atomic_total,
                            course_of_study.value.clone(),
                        );
                        let response = response.collect::<Vec<AnmeldungResponse>>().await;

                        log::info!("downloaded done");
                        let content = serde_json::to_string(&response).unwrap();
                        result.push((
                            format!(
                                "registration{}_{}.{semester}.v1.tucan",
                                course_of_study.value, course_of_study.name
                            ),
                            compress(content.as_bytes()).await.unwrap(),
                        ));
                    }
                });
                progresses.push((atomic_current, atomic_total));
                // now extract the modules in there?

            }
            loading.set(false);
        }
    };

    rsx! {
        div {
            class: "container",
            if loading() {
                div {
                    style: "z-index: 10000",
                    class: "position-fixed top-50 start-50 translate-middle",
                    div {
                        class: "spinner-grow",
                        role: "status",
                        span {
                            class: "visually-hidden",
                            "Loading..."
                        }
                    }
                }
            }
            h1 {
                class: "text-center",
                "Anmeldungsexporte"
            }
            p {
                "Das Laden könnte etwas länger dauern (5-10 Minuten). Außerdem macht es ca. 1500 \
                 Anfragen an TUCaN und benötigt ca. 30MB Datenvolumen."
            }
            button {
                onclick,
                class: "btn btn-primary mb-1",
                disabled: loading(),
                "Exportieren"
            }
            br {
            }
            for entry in result() {
                a {
                    href: {
                        #[cfg(target_arch = "wasm32")]
                        {
                            let blob_properties = web_sys::BlobPropertyBag::new();
                            blob_properties.set_type("octet/stream");
                            let bytes = js_sys::Array::new();
                            bytes.push(&js_sys::Uint8Array::from(&entry.1[..]));
                            let blob =
                                web_sys::Blob::new_with_blob_sequence_and_options(&bytes, &blob_properties)
                                    .unwrap();
                            web_sys::Url::create_object_url_with_blob(&blob).unwrap()
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        "/todo"
                    },
                    download: entry.0.clone(),
                    { format!("Download {}", entry.0.clone()) }
                }
                br {
                }
            }
            for progress in progresses() {
                div {
                class: "progress", role:"progressbar", "aria-label": "Basic example", "aria-valuenow": "25",
                "aria-valuemin": "0", "aria-valuemax": "100",
                        div { class: "progress-bar", style: "width: {progress.0()*100/progress.1()}%"
                        }
                }
                { progress.0().to_string() }
                "/"
                { progress.1().to_string() }
            }
        }
    }
}
