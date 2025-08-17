use std::fs;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use async_compression::futures::bufread::BrotliDecoder;
use async_compression::tokio::write::BrotliEncoder;
use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt as _};
use tucan_connector::TucanConnector;
use tucant_planning::{compress, recursive_anmeldung};
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::moduledetails::ModuleDetailsRequest;
use tucant_types::registration::{
    AnmeldungCourse, AnmeldungEntry, AnmeldungModule, AnmeldungRequest, AnmeldungResponse,
    RegistrationState,
};
use tucant_types::{DynTucan, LoginRequest, RevalidationStrategy, Tucan};
use tucant_types::{LoginResponse, TucanError};

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = TucanConnector::new().await?;

    /*let login_response = LoginResponse {
        id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
        cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
    };*/

    let login_response = tucan
        .login(LoginRequest {
            username: std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing"),
            password: std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing"),
        })
        .await
        .unwrap();

    let anmeldung_response = tucan
        .anmeldung(
            login_response.clone(),
            RevalidationStrategy::cache(),
            AnmeldungRequest::default(),
        )
        .await
        .unwrap();
    for course_of_study in anmeldung_response.studiumsauswahl {
        let result = recursive_anmeldung(
            DynTucan::from_ref(&tucan),
            &login_response,
            course_of_study.value.clone(),
        )
        .await;
        let content = serde_json::to_string(&result).unwrap();
        tokio::fs::write(
            format!(
                "registration{}_{}.json.br",
                course_of_study.value, course_of_study.name
            ),
            &compress(content.as_bytes()).await.unwrap(),
        )
        .await
        .unwrap();
    }

    Ok(())
}
