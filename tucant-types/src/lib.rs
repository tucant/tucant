pub mod moduledetails;
pub mod registration;

use axum_core::response::{IntoResponse, Response};
use registration::{AnmeldungRequest, AnmeldungResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub id: u64,
    pub cookie_cnsc: String,
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
    ) -> impl std::future::Future<Output = Result<LoginResponse, TucanError>> + Send;

    fn anmeldung(
        request: AnmeldungRequest,
    ) -> impl std::future::Future<Output = Result<AnmeldungResponse, TucanError>> + Send;
}
