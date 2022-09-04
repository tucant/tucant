// https://docs.rs/axum/latest/axum/struct.Router.html#method.nest
// https://docs.rs/axum/latest/axum/extract/index.html
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::Path,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, collections::HashMap};


async fn users_get(Path(params): Path<HashMap<String, String>>) {
    // Both `version` and `id` were captured even though `users_api` only
    // explicitly captures `id`.
    let version = params.get("version");
    let id = params.get("id");
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let users_api = Router::new().route("/users/:id", get(users_get));

    let app = Router::new().nest("/:version/api", users_api);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}