use axum::{
    debug_handler,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tucan_connector::{
    login::{login, LoginResponse},
    Tucan, TucanError,
};
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

// https://docs.rs/utoipa/latest/utoipa/attr.path.html#axum_extras-feature-support-for-axum
// https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs

// http://localhost:3000/swagger-ui/

// http://localhost:3000/api-docs/openapi.json

const TUCANT_TAG: &str = "tucant";

#[derive(OpenApi)]
#[openapi(
        tags(
            (name = TUCANT_TAG, description = "TUCaN't API")
        )
    )]
struct ApiDoc;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = TUCANT_TAG,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
#[debug_handler]
async fn login_endpoint(
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, TucanError> {
    let tucan = Tucan::new().await?;

    let response = login(
        &tucan.client,
        &login_request.username,
        &login_request.password,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(response)).into_response())
}

#[tokio::main]
async fn main() {
    // our router
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(login_endpoint))
        .split_for_parts();

    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
