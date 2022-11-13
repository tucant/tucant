// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod csrf_middleware;
mod s_course;
mod s_get_modules;
mod s_module;
mod s_my_courses;
mod s_my_modules;
mod s_search_course;
mod s_search_module;
mod s_setup;

use actix_cors::Cors;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::web::{Json, Query};
use actix_web::{cookie::Key, post, web, App, HttpServer};
use actix_web::{get, HttpResponse};

use csrf_middleware::CsrfMiddleware;

use file_lock::{FileLock, FileOptions};
use itertools::Itertools;

use s_course::course;
use s_get_modules::get_modules;
use s_module::module;
use s_my_courses::my_courses;
use s_my_modules::my_modules;
use s_search_course::search_course;
use s_search_module::search_module;
use s_setup::setup;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Display;
use tracing::warn;
use tucant::schema::{sessions, users_unfinished};

use tucant::models::{TucanSession, UndoneUser};

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucant::typescript::TypescriptableApp;

use std::io::Write;
use tucant::tucan::Tucan;
use tucant::tucan_user::TucanUser;
use tucant::url::{Coursedetails, Moduledetails, Registration};
use tucant_derive::{ts, Typescriptable};

use crate::s_search_module::search_module_opensearch;

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

#[derive(Deserialize, Debug, Typescriptable)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Debug, Typescriptable)]
struct LoginResult {
    success: bool,
}

#[tracing::instrument(skip(session))]
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

#[tracing::instrument(skip(session))]
#[get("/login-hack")]
async fn login_hack(
    session: Session,
    tucan: web::Data<Tucan>,
    input: Query<TucanSession>,
) -> Result<HttpResponse, MyError> {
    // TODO FIXME check that this session belongs to the user etc. (simply don't request the user id but fetch it from server)
    // TODO FIXME
    let user = UndoneUser::new("mh58hyqa".to_string());

    use diesel_async::RunQueryDsl;

    let mut connection = tucan.pool.get().await?;

    {
        let input = input.0.clone();
        connection
            .build_transaction()
            .run(|mut connection| {
                Box::pin(async move {
                    // TODO FIXME implement this by fetching and checking the session
                    diesel::insert_into(users_unfinished::table)
                        .values(user)
                        .on_conflict(users_unfinished::tu_id)
                        .do_nothing()
                        .execute(&mut connection)
                        .await?;

                    diesel::insert_into(sessions::table)
                        .values(input)
                        .on_conflict((sessions::tu_id, sessions::session_nr, sessions::session_id))
                        .do_nothing()
                        .execute(&mut connection)
                        .await?;

                    Ok::<(), diesel::result::Error>(())
                })
            })
            .await?;
    }

    session.insert("session", input.0).unwrap();
    Ok(HttpResponse::Found()
        .append_header(("Location", "http://localhost:5173/"))
        .finish())
}

#[tracing::instrument(skip(session))]
#[ts]
#[post("/logout")]
async fn logout(session: Session, _input: Json<()>) -> Result<Json<()>, MyError> {
    session.purge();
    Ok(web::Json(()))
}

#[ts]
#[post("/")]
async fn index(session: TucanSession, _input: Json<()>) -> Result<Json<String>, MyError> {
    Ok(web::Json(format!("Welcome! {}", session.tu_id)))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    /*
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(opentelemetry_otlp::new_exporter().tonic()) // with_endpoint("http://localhost:")
            .install_batch(opentelemetry::runtime::Tokio)?;

        // Create a tracing layer with the configured tracer
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        // Use the tracing subscriber `Registry`, or any other subscriber
        // that impls `LookupSpan`
        let subscriber = Registry::default().with(telemetry);

        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    */

    warn!("Starting server...");

    // https://crates.io/crates/tracing

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
        /*
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin_fn(|origin, _| {
                println!("{:?}", origin);
                origin == "http://127.0.0.1:5173" ||  origin == "http://localhost:5173"
            });*/
        let cors = Cors::permissive();

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

        let app = TypescriptableApp {
            app,
            codes: BTreeSet::new(),
        };
        let app = app
            // TODO FIXME looks like this generates massive backtraces, maybe switch to manual get and post and not this macro and service magic
            .service(index)
            .service(login)
            .service(logout)
            .service(get_modules)
            .service(search_module)
            .service(search_module_opensearch)
            .service(search_course)
            .service(course)
            .service(module)
            .service(my_modules)
            .service(my_courses);

        let should_we_block = true;
        let lock_for_writing = FileOptions::new().write(true).create(true).truncate(true);

        let mut filelock = match FileLock::lock(
            "../frontend-react/src/api.ts",
            should_we_block,
            lock_for_writing,
        ) {
            Ok(lock) => lock,
            Err(err) => panic!("Error getting write lock: {}", err),
        };

        filelock
            .file
            .write_all(
                (r#"
// This file is automatically generated at startup. Do not modify.
import { genericFetch } from "./api_base"
"#
                .to_string()
                    + &app.codes.into_iter().join("\n"))
                    .as_bytes(),
            )
            .unwrap();

        // Manually unlocking is optional as we unlock on Drop
        filelock.unlock().unwrap();

        app.app.service(setup).service(login_hack)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
