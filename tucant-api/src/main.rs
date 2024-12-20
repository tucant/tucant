use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use utoipa::IntoParams;

// https://docs.rs/utoipa/latest/utoipa/attr.path.html#axum_extras-feature-support-for-axum

/// Get todo by id and name.
#[utoipa::path(
    get,
    path = "/todo/{id}",
    params(
        ("id", description = "Todo id"),
        ("name", description = "Todo name")
    ),
    responses(
        (status = 200, description = "Get todo success", body = String)
    )
)]
async fn get_todo(Path((id, name)): Path<(i32, String)>) -> String {
    String::new()
}

#[derive(Deserialize, IntoParams)]
struct TodoSearchQuery {
    /// Search by value. Search is incase sensitive.
    value: String,
    /// Search by `done` status.
    done: bool,
}

/// Search Todos by query params.
#[utoipa::path(
    get,
    path = "/todo/search",
    params(
        TodoSearchQuery
    ),
    responses(
        (status = 200, description = "List matching todos by query", body = [String])
    )
)]
async fn search_todos(query: Query<TodoSearchQuery>) -> Json<Vec<String>> {
    Json(vec![])
}

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
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
