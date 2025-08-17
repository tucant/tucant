use std::str::FromStr;

use dioxus::prelude::*;
use tucant_planning::{compress, recursive_anmeldung};
use tucant_types::{
    LoginResponse, SemesterId, Tucan, examresults::ExamResultsResponse,
    registration::AnmeldungRequest,
};

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
        }
        Ok(())
    };

    let navigator = use_navigator();

    let anonymize = use_context::<Anonymize>().0;

    use_authenticated_data_loader(
        handler,
        ReadSignal::new(Signal::new(())),
        14 * 24 * 60 * 60,
        60 * 60,
        |exam_results: (), reload| {
            rsx! {
                "Done"
            }
        },
    )
}
