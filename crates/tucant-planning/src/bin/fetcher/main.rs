use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt as _};
use tucan_connector::TucanConnector;
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::moduledetails::ModuleDetailsRequest;
use tucant_types::registration::{
    AnmeldungCourse, AnmeldungEntry, AnmeldungModule, AnmeldungRequest, AnmeldungResponse,
    RegistrationState,
};
use tucant_types::{LoginRequest, RevalidationStrategy, Tucan};
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
        let mut file = File::create_new(format!("registration{}_{}.json", course_of_study.value, course_of_study.name)).await.unwrap();
        file.write_all(b"[\n").await.unwrap();
       
       recursive_anmeldung(file.try_clone().await.unwrap(), &tucan, &login_response, course_of_study.value)
            .await;
        file.seek(std::io::SeekFrom::Current(-2)).await.unwrap();
        file.write_all(b"\n]\n").await.unwrap();
    }

    Ok(())
}

struct Fetcher {
    anmeldung_file: File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungResponse {
    pub path: Vec<(String, AnmeldungRequest)>,
    pub submenus: Vec<(String, AnmeldungRequest)>,
    pub entries: Vec<ExportableAnmeldungEntry>,
}

impl From<AnmeldungResponse> for ExportableAnmeldungResponse {
    fn from(value: AnmeldungResponse) -> Self {
        Self {
            path: value.path,
            submenus: value.submenus,
            entries: value.entries.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungEntry {
    pub module: Option<ExportableAnmeldungModule>,
    pub courses: Vec<ExportableAnmeldungCourse>,
}

impl From<AnmeldungEntry> for ExportableAnmeldungEntry {
    fn from(value: AnmeldungEntry) -> Self {
        Self {
            module: value.module.map(Into::into),
            courses: value.courses.into_iter().map(|e| e.1.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungModule {
    pub url: ModuleDetailsRequest,
    pub id: String,
    pub name: String,
}

impl From<AnmeldungModule> for ExportableAnmeldungModule {
    fn from(value: AnmeldungModule) -> Self {
        Self {
            url: value.url,
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungCourse {
    pub url: CourseDetailsRequest,
    pub id: String,
    pub name: String,
}

impl From<AnmeldungCourse> for ExportableAnmeldungCourse {
    fn from(value: AnmeldungCourse) -> Self {
        Self {
            url: value.url,
            id: value.id,
            name: value.name,
        }
    }
}

#[expect(clippy::manual_async_fn)]
fn recursive_anmeldung<'a, 'b>(
    file: File,
    tucan: &'a TucanConnector,
    login_response: &'b LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> impl Future<Output = ()> + Send + use<'a, 'b> {
    async move {
        let anmeldung_response = tucan
            .anmeldung(
                login_response.clone(),
                RevalidationStrategy::cache(),
                anmeldung_request.clone(),
            )
            .await
            .unwrap();
        // let anmeldung_response = ExportableAnmeldungResponse::from(anmeldung_response);

        let mut output = serde_json::to_string(&anmeldung_response).unwrap();
        output.push_str(",\n");
        let len = file
            .try_clone()
            .await
            .unwrap()
            .write(output.as_bytes())
            .await
            .unwrap();
        assert_eq!(len, output.len());

        let results: FuturesUnordered<_> = anmeldung_response
            .submenus
            .iter()
            .map(|entry| {
                async {
                    recursive_anmeldung(file.try_clone().await.unwrap(), tucan, login_response, entry.1.clone())
                        .await;
                }
                .boxed()
            })
            .collect();
        results.collect::<Vec<()>>().await;
    }
}
