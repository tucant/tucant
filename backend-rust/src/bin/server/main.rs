// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod s_course;
mod s_get_modules;
mod s_module;
mod s_my_courses;
mod s_my_modules;
mod s_search_course;
mod s_search_module;
mod s_setup;

use axum::Json;

use axum::Router;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Key;
use diesel::{Connection, PgConnection};
use diesel_migrations::FileBasedMigrations;
use diesel_migrations::MigrationHarness;
use dotenvy::dotenv;
use file_lock::{FileLock, FileOptions};
use itertools::Itertools;

use log::error;
use reqwest::StatusCode;
use reqwest::header::USER_AGENT;
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
use tucant::url::{parse_tucan_url, Coursedetails, Moduledetails, Registration};
use tucant_derive::{ts, Typescriptable};
use tucant_derive_lib::Typescriptable;

use crate::s_search_module::search_module_opensearch;

#[derive(Serialize, Typescriptable)]
pub struct WithTucanUrl<T: Serialize + Typescriptable> {
    pub tucan_url: String,
    //#[serde(flatten)] // not supported by Typescriptable
    pub inner: T,
}

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

// https://docs.rs/axum-extra/latest/axum_extra/extract/struct.PrivateCookieJar.html
#[ts]
async fn login(
    cookie_jar: PrivateCookieJar,
    tucan: web::Data<Tucan>,
    input: Json<Login>,
) -> Result<Json<LoginResult>, MyError> {
    let tucan_user = tucan.login(&input.username, &input.password).await?;
    session.insert("session", tucan_user.session).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginHack {
    pub session_nr: Option<i64>,
    pub session_id: Option<String>,
    pub redirect: String,
}

#[tracing::instrument(skip(session))]
async fn login_hack(
    session: Session,
    req: HttpRequest,
    tucan: web::Data<Tucan>,
    input: Query<LoginHack>,
) -> Result<HttpResponse, MyError> {
    println!("{:?}", input);

    use diesel_async::RunQueryDsl;

    let mut connection = tucan.pool.get().await?;

    if let LoginHack {
        session_nr: Some(session_nr),
        session_id: Some(session_id),
        ..
    } = input.0.clone()
    {
        let tucan_user = tucan
            .tucan_session_from_session_data(session_nr, session_id)
            .await?;
        let tucan_session = tucan_user.session.clone();
        let user = UndoneUser::new(tucan_user.session.matriculation_number);
        connection
            .build_transaction()
            .run(|mut connection| {
                Box::pin(async move {
                    // TODO FIXME implement this by fetching and checking the session
                    diesel::insert_into(users_unfinished::table)
                        .values(user)
                        .on_conflict(users_unfinished::matriculation_number)
                        .do_nothing()
                        .execute(&mut connection)
                        .await?;

                    diesel::insert_into(sessions::table)
                        .values(tucan_session)
                        .on_conflict((
                            sessions::matriculation_number,
                            sessions::session_nr,
                            sessions::session_id,
                        ))
                        .do_nothing()
                        .execute(&mut connection)
                        .await?;

                    Ok::<(), diesel::result::Error>(())
                })
            })
            .await?;
        session
            .insert("session", tucan_user.session.clone())
            .unwrap();
    }

    let url = match parse_tucan_url(&input.redirect).program {
        tucant::url::TucanProgram::Registration(registration) => req.url_for(
            "registration",
            [base64::encode_config(
                registration.path,
                base64::URL_SAFE_NO_PAD,
            )],
        )?,
        tucant::url::TucanProgram::RootRegistration(_) => {
            req.url_for::<[String; 0], _>("root_registration", [])?
        }
        tucant::url::TucanProgram::Moduledetails(module_details) => req.url_for(
            "module",
            [base64::encode_config(
                module_details.id,
                base64::URL_SAFE_NO_PAD,
            )],
        )?,
        tucant::url::TucanProgram::Coursedetails(course_details) => req.url_for(
            "course",
            [base64::encode_config(
                course_details.id,
                base64::URL_SAFE_NO_PAD,
            )],
        )?,
        tucant::url::TucanProgram::Externalpages(_) => {
            req.url_for::<[String; 0], _>("index", [])?
        }
        other => {
            println!("{:?}", other);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url.as_str()))
        .finish())
}

#[tracing::instrument(skip(session))]
#[ts]
async fn logout(session: Session, _input: Json<()>) -> Result<Json<()>, MyError> {
    session.purge();
    Ok(web::Json(()))
}

#[ts]
async fn index(session: TucanSession, _input: Json<()>) -> Result<Json<String>, MyError> {
    Ok(web::Json(format!(
        "Welcome! {}",
        session.matriculation_number
    )))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // https://github.com/weiznich/diesel_async/issues/17
    let migrations = FileBasedMigrations::find_migrations_directory()?;
    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    connection.run_pending_migrations(migrations).unwrap();

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
    let secret_key = Key::from(&secret_key_raw);

    let tucan = Tucan::new().await?;

    let app = Router::new().with_state(secret_key);
/*
    HttpServer::new(move || {
        let logger = Logger::default();
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

        app.app
            .service(setup)
            .service(login_hack)
            .external_resource("course", "http://localhost:5173/course/{course_name}")
            .external_resource("module", "http://localhost:5173/module/{course_name}")
            .external_resource(
                "registration",
                "http://localhost:5173/modules/{registration}",
            )
            .external_resource("root_registration", "http://localhost:5173/modules/")
            .external_resource("my_modules", "http://localhost:5173/my-modules/")
            .external_resource("my_courses", "http://localhost:5173/my-courses/")
            .external_resource("index", "http://localhost:5173/")
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;
*/
    Ok(())
}
