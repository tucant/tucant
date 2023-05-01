pub mod typescript;

#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl<E: Into<anyhow::Error>> From<E> for MyError {
    fn from(err: E) -> Self {
        Self { err: err.into() }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        println!("{:?}", self.err);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self.err)).into_response()
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for TucanSession
where
    Key: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = PrivateCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .map_err(axum::response::IntoResponse::into_response)?;

        let session: Self = serde_json::from_str(
            cookie_jar
                .get("session")
                .ok_or_else(|| {
                    (axum::http::StatusCode::UNAUTHORIZED, "session not found").into_response()
                })?
                .value(),
        )
        .map_err(|err| Into::<tucant::MyError>::into(err).into_response())?;
        Ok(session)
    }
}
