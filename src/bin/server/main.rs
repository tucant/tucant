#![feature(try_trait_v2)]

mod csrf_middleware;

use std::{fmt::Display, time::Duration};

use actix_cors::Cors;
use actix_identity::{IdentityMiddleware, Identity, config::IdentityMiddlewareBuilder};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{post, web, App, HttpServer, Responder, cookie::Key, get, HttpRequest, HttpResponse};
use csrf_middleware::CsrfMiddleware;
use serde::{Deserialize, Serialize};
use tucan_scraper::tucan::Tucan;
use actix_web::HttpMessage;

#[derive(Debug)]
struct MyError {
    err: anyhow::Error,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl actix_web::error::ResponseError for MyError {
}

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
async fn echo(request: HttpRequest, login: web::Json<Login>) -> Result<impl Responder, MyError> {
    let tucan = Tucan::new().await?;
    tucan.login(&login.username, &login.password).await?;
    Identity::login(&request.extensions(), login.username.to_string().into()).unwrap();
    Ok(web::Json(LoginResult {
        success: true
    }))
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

#[get("/")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_method().allow_any_header().allowed_origin("http://localhost:3000");

        App::new()
            .wrap(IdentityMiddleware::builder().visit_deadline(Some(Duration::from_secs(24*3600))).build())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone()
            ))
            .wrap(CsrfMiddleware {})
            .wrap(cors)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
