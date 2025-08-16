use std::fs;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use async_compression::futures::bufread::{BrotliDecoder, LzmaEncoder};
use async_compression::tokio::write::{
    BrotliEncoder, BzEncoder, DeflateEncoder, Lz4Encoder, ZstdEncoder,
};
use async_compression::tokio::write::{ZlibDecoder, ZlibEncoder};
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

async fn compress(in_data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = async_compression::tokio::write::BrotliEncoder::with_quality(
        Vec::new(),
        async_compression::Level::Best,
    );
    encoder.write_all(in_data).await?;
    encoder.shutdown().await?;
    Ok(encoder.into_inner())
}

async fn decompress(in_data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = async_compression::tokio::write::BrotliDecoder::new(Vec::new());
    decoder.write_all(in_data).await?;
    decoder.shutdown().await?;
    Ok(decoder.into_inner())
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
        let result =
            recursive_anmeldung(&tucan, &login_response, course_of_study.value.clone()).await;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungResponse {
    pub path: (String, AnmeldungRequest),
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub submenus: Vec<ExportableAnmeldungResponse>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<ExportableAnmeldungEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportableAnmeldungEntry {
    pub module: Option<ExportableAnmeldungModule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    tucan: &'a TucanConnector,
    login_response: &'b LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> impl Future<Output = ExportableAnmeldungResponse> + Send + use<'a, 'b> {
    async move {
        let anmeldung_response = tucan
            .anmeldung(
                login_response.clone(),
                RevalidationStrategy::cache(),
                anmeldung_request.clone(),
            )
            .await
            .unwrap();

        let results: FuturesUnordered<_> = anmeldung_response
            .submenus
            .iter()
            .map(|entry| {
                async { recursive_anmeldung(tucan, login_response, entry.1.clone()).await }.boxed()
            })
            .collect();
        let results = results.collect::<Vec<ExportableAnmeldungResponse>>().await;

        ExportableAnmeldungResponse {
            path: anmeldung_response.path.last().unwrap().clone(),
            submenus: results,
            entries: anmeldung_response
                .entries
                .into_iter()
                .map(ExportableAnmeldungEntry::from)
                .collect(),
        }
    }
}
