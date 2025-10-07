use dioxus::prelude::*;
use time::{Month, macros::offset};
use tucan_plus_planning::{compress, recursive_anmeldung};
use tucan_types::{LoginResponse, RevalidationStrategy, Tucan, registration::AnmeldungRequest};

use crate::RcTucanType;

#[component]
pub fn FetchAnmeldung() -> Element {
    let mut result: Signal<Vec<(String, Vec<u8>)>> = use_signal(Vec::new);
    let tucan: RcTucanType = use_context();
    let current_session_handle = use_context::<Signal<Option<LoginResponse>>>();
    let mut loading = use_signal(|| false);

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

            let mut output = Vec::new();
            for course_of_study in anmeldung_response.studiumsauswahl {
                log::info!("start");
                let session = current_session_handle().unwrap();
                let result = recursive_anmeldung(
                    &tucan.0,
                    &session,
                    course_of_study.value.clone(),
                );
                // now extract the modules in there?

                log::info!("downloaded done");
                let content = serde_json::to_string(&"result").unwrap();
                output.push((
                    format!(
                        "registration{}_{}.{semester}.v1.tucan",
                        course_of_study.value, course_of_study.name
                    ),
                    compress(content.as_bytes()).await.unwrap(),
                ));
            }
            result.set(output);
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
                class: "btn btn-primary",
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
        }
    }
}
