use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use tucan_connector::{
    TucanConnector,
    login::{login, logout},
    registration::index::anmeldung_cached,
};
use tucant_types::{LoggedInHead, Tucan, coursedetails::CourseDetailsResponse, mlsstart::MlsStart, moduledetails::ModuleDetailsResponse};
use tucant_types::{
    LoginRequest, LoginResponse, TucanError,
    coursedetails::CourseDetailsRequest,
    moduledetails::ModuleDetailsRequest,
    registration::{AnmeldungRequest, AnmeldungResponse},
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::{router::OpenApiRouter, routes};

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
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme("cnsc", SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("cnsc"))));
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
pub async fn login_endpoint(jar: CookieJar, Json(login_request): Json<LoginRequest>) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let response = login(&tucan.client, &login_request).await?;

    let jar = jar.add(Cookie::build(("id", response.id.to_string())).path("/")).add(Cookie::build(("cnsc", response.cookie_cnsc.to_string())).path("/"));

    Ok((StatusCode::OK, jar, Json(response)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    tag = TUCANT_TAG,
    responses(
        (status = 200, description = "Logout successful", body = ()),
        (status = 500, description = "Some TUCaN error")
    )
)]
pub async fn logout_endpoint(jar: CookieJar) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let login_response: LoginResponse = LoginResponse {
        id: jar.get("id").unwrap().value().parse().unwrap(),
        cookie_cnsc: jar.get("cnsc").unwrap().value().to_owned(),
    };

    logout(&tucan.client, &login_response).await.unwrap();

    let jar = jar.remove(Cookie::from("id")).remove(Cookie::from("cnsc"));

    Ok((StatusCode::OK, jar, Json(())).into_response())
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
pub async fn registration_endpoint(jar: CookieJar, Path(registration): Path<String>) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let login_response: LoginResponse = LoginResponse {
        id: jar.get("id").unwrap().value().parse().unwrap(),
        cookie_cnsc: jar.get("cnsc").unwrap().value().to_owned(),
    };

    let response = anmeldung_cached(&tucan, &login_response, AnmeldungRequest::parse(&registration)).await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/module-details/{module}",
    tag = TUCANT_TAG,
    params(("module" = String, Path)),
    responses(
        (status = 200, description = "Successful", body = ModuleDetailsResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
pub async fn module_details_endpoint(jar: CookieJar, Path(module): Path<String>) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let login_response: LoginResponse = LoginResponse {
        id: jar.get("id").unwrap().value().parse().unwrap(),
        cookie_cnsc: jar.get("cnsc").unwrap().value().to_owned(),
    };

    let response = tucan.module_details(&login_response, ModuleDetailsRequest::parse(&module)).await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/course-details/{course}",
    tag = TUCANT_TAG,
    params(("course" = String, Path)),
    responses(
        (status = 200, description = "Successful", body = CourseDetailsResponse),
        (status = 500, description = "Some TUCaN error")
    )
)]
pub async fn course_details_endpoint(jar: CookieJar, Path(course): Path<String>) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let login_response: LoginResponse = LoginResponse {
        id: jar.get("id").unwrap().value().parse().unwrap(),
        cookie_cnsc: jar.get("cnsc").unwrap().value().to_owned(),
    };

    let response = tucan.course_details(&login_response, CourseDetailsRequest::parse(&course)).await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/v1/after-login",
    tag = TUCANT_TAG,
    responses(
        (status = 200, description = "Successful", body = MlsStart),
        (status = 500, description = "Some TUCaN error")
    )
)]
pub async fn after_login_endpoint(jar: CookieJar) -> Result<impl IntoResponse, TucanError> {
    let tucan = TucanConnector::new().await?;

    let login_response: LoginResponse = LoginResponse {
        id: jar.get("id").unwrap().value().parse().unwrap(),
        cookie_cnsc: jar.get("cnsc").unwrap().value().to_owned(),
    };

    let response = tucan.after_login(&login_response).await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).routes(routes!(login_endpoint)).routes(routes!(logout_endpoint)).routes(routes!(registration_endpoint)).routes(routes!(module_details_endpoint)).routes(routes!(course_details_endpoint)).routes(routes!(after_login_endpoint))
}
