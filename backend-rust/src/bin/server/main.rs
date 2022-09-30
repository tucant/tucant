// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod csrf_middleware;
mod s_course;
mod s_get_modules;
mod s_module;
mod s_search_course;
mod s_search_module;
mod s_setup;

use actix_cors::Cors;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};
use csrf_middleware::CsrfMiddleware;
use diesel_async::pooled_connection::PoolError;
use s_course::course;
use s_get_modules::get_modules;
use s_module::module;
use s_search_course::search_course;
use s_search_module::search_module;
use s_setup::setup;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucant::typescript::TypescriptableApp;

use tucant::tucan::Tucan;
use tucant::tucan_user::{TucanSession, TucanUser};
use tucant::url::{Coursedetails, Moduledetails, Registration};
use tucant_derive::ts;

#[derive(Debug)]
pub struct MyError {
    err: anyhow::Error,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl actix_web::error::ResponseError for MyError {}

impl<E: Into<anyhow::Error>> From<E> for MyError {
    fn from(err: E) -> MyError {
        MyError { err: err.into() }
    }
}

#[ts]
#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[ts]
#[derive(Serialize)]
struct LoginResult {
    success: bool,
}

#[ts]
#[post("/login")]
async fn login(
    session: Session,
    tucan: web::Data<Tucan>,
    input: web::Json<Login>,
) -> Result<Json<LoginResult>, MyError> {
    let tucan_user = tucan.login(&input.username, &input.password).await?;
    session.insert("session", tucan_user.session).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[ts]
#[post("/logout")]
async fn logout(session: Session, input: Json<()>) -> Result<Json<()>, MyError> {
    session.purge();
    Ok(web::Json(()))
}

#[ts]
#[get("/")]
async fn index(session: Session, input: Json<()>) -> Result<Json<String>, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => Ok(web::Json(format!("Welcome! {}", session.nr))),
        None => Ok(web::Json("Welcome Anonymous!".to_owned())),
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let random_secret_key = Key::generate();

    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("sessions.key")
        .await;
    if let Ok(mut file) = file {
        file.write_all(random_secret_key.master()).await?;
        drop(file)
    }

    let secret_key_raw = fs::read("sessions.key").await?;
    let secret_key = Key::derive_from(&secret_key_raw);

    let tucan = web::Data::new(Tucan::new().await?);

    HttpServer::new(move || {
        let logger = Logger::default();

        let cors = Cors::default()
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin("http://localhost:5173");

        let app = App::new()
            .app_data(tucan.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_same_site(SameSite::None)
                    .cookie_secure(true)
                    .cookie_http_only(false)
                    .build(), // TODO FIXME
            )
            .wrap(CsrfMiddleware {})
            .wrap(cors)
            .wrap(logger);

        let app = TypescriptableApp { app };
        let app = app
            .service(index)
            .service(login)
            .service(logout)
            .service(get_modules)
            .service(search_module)
            .service(search_course)
            .service(course)
            .service(module);

        app.app.service(setup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
