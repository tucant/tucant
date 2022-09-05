#![feature(try_trait_v2)]

mod csrf_middleware;

use std::{fmt::Display, time::Duration};

use actix_cors::Cors;
use actix_identity::{config::IdentityMiddlewareBuilder, Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::HttpMessage;
use actix_web::{
    cookie::Key, error::ErrorUnauthorized, get, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use csrf_middleware::CsrfMiddleware;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucan_scraper::tucan::Tucan;

#[derive(Debug)]
struct MyError {
    err: anyhow::Error,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl actix_web::error::ResponseError for MyError {}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResult {
    success: bool,
}

#[post("/login")]
async fn login(request: HttpRequest, login: web::Json<Login>) -> Result<impl Responder, MyError> {
    let tucan = Tucan::new().await?;
    tucan.login(&login.username, &login.password).await?;
    Identity::login(&request.extensions(), login.username.to_string().into()).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[post("/logout")]
async fn logout(user: Identity) -> Result<impl Responder, MyError> {
    user.logout();
    Ok(HttpResponse::Ok())
}

#[get("/")]
async fn index(user: Option<Identity>) -> Result<impl Responder, MyError> {
    if let Some(user) = user {
        Ok(web::Json(format!("Welcome! {}", user.id().unwrap())))
    } else {
        Ok(web::Json("Welcome Anonymous!".to_owned()))
    }
}

#[get("/registration")]
async fn registration(user: Option<Identity>) -> Result<impl Responder, MyError> {
    if let Some(user) = user {
        let tucan = Tucan::new().await?;
        let user_id = user.id()?;
        let tucan = tucan.continue_session(&user_id).await?;

        Ok(web::Json(tucan.registration(None).await?))
    } else {
        Err(anyhow::Error::msg("Not logged in!"))?
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let random_secret_key = Key::generate();

    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("sessions.key")
        .await;
    if let Ok(mut file) = file {
        file.write(random_secret_key.master()).await?;
        drop(file)
    }

    let secret_key_raw = fs::read("sessions.key").await?;
    let secret_key = Key::derive_from(&secret_key_raw);

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin("http://localhost:3000");

        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(Duration::from_secs(24 * 3600)))
                    .build(),
            )
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(CsrfMiddleware {})
            .wrap(cors)
            .service(index)
            .service(login)
            .service(logout)
            .service(registration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
