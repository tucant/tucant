use axum::{debug_handler, extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use tucan_connector::{
    login::login, moduledetails::index::moduledetails, registration::index::anmeldung_cached, Tucan,
};
use tucant_types::{
    moduledetails::ModuleDetailsRequest,
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse, TucanError,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

// https://docs.rs/utoipa/latest/utoipa/attr.path.html#axum_extras-feature-support-for-axum
// https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs

// http://localhost:3000/swagger-ui/

// http://localhost:3000/api-docs/openapi.json

const TUCANT_TAG: &str = "tucant";

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
        tags(
            (name = TUCANT_TAG, description = "TUCaN't API")
        )
    )]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("api_key"))),
            )
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = TUCANT_TAG,
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
#[debug_handler]
async fn login_endpoint(
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, TucanError> {
    let tucan = Tucan::new().await?;

    let response = login(&tucan.client, &login_request).await?;

    let jar = jar.add(Cookie::new(
        "api_key",
        serde_json::to_string(&response).unwrap(),
    ));

    Ok((StatusCode::OK, jar, Json(response)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/registration/{registration}",
    tag = TUCANT_TAG,
    params(("registration" = String, Path)),
    responses(
        (status = 200, description = "Successful", body = AnmeldungResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
#[debug_handler]
async fn registration_endpoint(
    jar: CookieJar,
    Path(registration): Path<String>,
) -> Result<impl IntoResponse, TucanError> {
    let tucan = Tucan::new().await?;

    let login_response: LoginResponse =
        serde_json::from_str(jar.get("api_key").unwrap().value()).unwrap();

    let response = anmeldung_cached(
        &tucan,
        &login_response,
        AnmeldungRequest {
            arguments: registration,
        },
    )
    .await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/module-details/{module}",
    tag = TUCANT_TAG,
    params(("module" = String, Path)),
    responses(
        (status = 200, description = "Successful", body = AnmeldungResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
#[debug_handler]
async fn module_details_endpoint(
    jar: CookieJar,
    Path(module): Path<String>,
) -> Result<impl IntoResponse, TucanError> {
    let tucan = Tucan::new().await?;

    let login_response: LoginResponse =
        serde_json::from_str(jar.get("api_key").unwrap().value()).unwrap();

    let response = moduledetails(
        &tucan,
        &login_response,
        ModuleDetailsRequest { arguments: module },
    )
    .await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

#[tokio::main]
async fn main() {
    // our router
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(login_endpoint))
        .routes(routes!(registration_endpoint))
        .routes(routes!(module_details_endpoint))
        .split_for_parts();

    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
