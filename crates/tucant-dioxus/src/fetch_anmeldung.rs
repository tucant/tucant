use std::str::FromStr;

use dioxus::prelude::*;
use js_sys::{Array, Uint8Array};
use tucant_planning::{compress, recursive_anmeldung};
use tucant_types::{
    LoginResponse, SemesterId, Tucan, examresults::ExamResultsResponse,
    registration::AnmeldungRequest,
};
use wasm_bindgen::JsValue;
use web_sys::{Blob, Url};

use crate::{Anonymize, RcTucanType, Route, common::use_authenticated_data_loader};

#[component]
pub fn FetchAnmeldung() -> Element {
    let handler = async |tucan: RcTucanType,
                         current_session: LoginResponse,
                         revalidation_strategy,
                         additional: ()| {
        let anmeldung_response = tucan
            .anmeldung(
                current_session.clone(),
                revalidation_strategy,
                AnmeldungRequest::default(),
            )
            .await
            .unwrap();
        let mut output = Vec::new();
        for course_of_study in anmeldung_response.studiumsauswahl {
            let result =
                recursive_anmeldung(&tucan.0, &current_session, course_of_study.value.clone())
                    .await;
            let content = serde_json::to_string(&result).unwrap();
            /*tokio::fs::write(
                format!(
                    "registration{}_{}.json.br",
                    course_of_study.value, course_of_study.name
                ),
                &compress(content.as_bytes()).await.unwrap(),
            )
            .await
            .unwrap();*/
            output.push((
                format!(
                    "registration{}_{}.json.br",
                    course_of_study.value, course_of_study.name
                ),
                compress(content.as_bytes()).await.unwrap(),
            ));
        }
        Ok(output)
    };

    let navigator = use_navigator();

    let anonymize = use_context::<Anonymize>().0;

    use_authenticated_data_loader(
        handler,
        ReadSignal::new(Signal::new(())),
        14 * 24 * 60 * 60,
        60 * 60,
        |output: Vec<(String, Vec<u8>)>, reload| {
            rsx! {
                for entry in output {
                    a {
                        href: {
                            let blob_properties = web_sys::BlobPropertyBag::new();
                            blob_properties.set_type("octet/stream");
                            let bytes = Array::new();
                            bytes.push(&Uint8Array::from(&entry.1[..]));
                            let blob = Blob::new_with_blob_sequence_and_options(&bytes, &blob_properties).unwrap();
                            let url = Url::create_object_url_with_blob(&blob).unwrap();
                            url
                        },
                        download: entry.0.clone(),
                        { format!("Download {}",  entry.0.clone()) }
                    }
                    br {}
                }
            }
        },
    )
}
