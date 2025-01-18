pub mod coursedetails;
pub mod moduledetails;
pub mod registration;

use axum_core::response::{IntoResponse, Response};
use coursedetails::{CourseDetailsRequest, CourseDetailsResponse};
use moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse};
use registration::{AnmeldungRequest, AnmeldungResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct LoginResponse {
    pub id: u64,
    pub cookie_cnsc: String,
}

#[derive(PartialEq, Clone)]
pub struct LoggedInHead {
    pub messages_url: String,
    pub vorlesungsverzeichnis_url: String,
    pub vv: VorlesungsverzeichnisUrls,
    pub antraege_url: String,
    pub meine_bewerbung_url: String,
}

#[derive(PartialEq, Clone)]
pub struct VorlesungsverzeichnisUrls {
    pub lehrveranstaltungssuche_url: String,
    pub vvs: Vec<(String, String)>,
}

#[derive(thiserror::Error, Debug)]
pub enum TucanError {
    #[error("HTTP error {0:?}")]
    Http(#[from] reqwest::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("HTTP middleware error {0:?}")]
    HttpMiddleware(#[from] reqwest_middleware::Error),
    #[error("IO error {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Tucan session timeout")]
    Timeout,
    #[error("Invalid credentials for TUCaN")]
    InvalidCredentials,
}

impl IntoResponse for TucanError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub trait Tucan {
    fn login(
        request: LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, TucanError>>;

    fn after_login(
        request: &LoginResponse,
    ) -> impl std::future::Future<Output = Result<LoggedInHead, TucanError>>;

    fn logout(request: &LoginResponse)
        -> impl std::future::Future<Output = Result<(), TucanError>>;

    fn anmeldung(
        login_response: LoginResponse,
        request: AnmeldungRequest,
    ) -> impl std::future::Future<Output = Result<AnmeldungResponse, TucanError>>;

    fn module_details(
        login_response: &LoginResponse,
        request: ModuleDetailsRequest,
    ) -> impl std::future::Future<Output = Result<ModuleDetailsResponse, TucanError>>;

    fn course_details(
        login_response: &LoginResponse,
        request: CourseDetailsRequest,
    ) -> impl std::future::Future<Output = Result<CourseDetailsResponse, TucanError>>;
}
