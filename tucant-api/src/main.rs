use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tucan_connector::{login::LoginResponse, TucanError};
use utoipa::{IntoParams, OpenApi};
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

struct LoginRequest {
    username: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "",
    tag = TUCANT_TAG,
    responses(
        (status = 201, description = "Todo item created successfully", body = LoginResponse),
        (status = 409, description = "Todo already exists", body = TucanError)
    )
)]
async fn login(Json(todo): Json<LoginRequest>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(todo)).into_response()
}

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todo", get(get_todo))
        .route("/search_todos", get(search_todos));

    // which calls one of these handlers
    async fn root() {}
    async fn get_foo() {}
    async fn post_foo() {}
    async fn foo_bar() {}

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
