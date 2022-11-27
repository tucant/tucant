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

use axum::async_trait;
use axum::body::Body;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::extract::Query;
use axum::extract::State;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::IntoResponseParts;
use axum::response::Redirect;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::Key;
use axum_extra::extract::PrivateCookieJar;
use diesel::{Connection, PgConnection};
use diesel_migrations::FileBasedMigrations;
use diesel_migrations::MigrationHarness;
use dotenvy::dotenv;
use file_lock::{FileLock, FileOptions};
use itertools::Itertools;

use log::error;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use s_course::course;
use s_get_modules::get_modules;
use s_module::module;
use s_my_courses::my_courses;
use s_my_modules::my_modules;
use s_search_course::search_course;
use s_search_module::search_module;
use s_setup::setup;
use serde::{Deserialize, Serialize};
use tucant::MyError;
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

#[derive(Deserialize, Debug, Typescriptable)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Debug, Typescriptable)]
struct LoginResult {
    success: bool,
}

struct TsHide<H: IntoResponseParts, V: IntoResponse + Typescriptable> {
    hidden: H,
    visible: V,
}

impl<H: IntoResponseParts, V: IntoResponse + Typescriptable> IntoResponse for TsHide<H, V> {
    fn into_response(self) -> Response {
        (self.hidden, self.visible).into_response()
    }
}

impl<H: IntoResponseParts, V: IntoResponse + Typescriptable> Typescriptable for TsHide<H, V> {
    fn name() -> String {
        V::name()
    }
    fn code() -> BTreeSet<String> {
        V::code()
    }
}

// https://docs.rs/axum-extra/latest/axum_extra/extract/struct.PrivateCookieJar.html
#[ts]
async fn login(
    cookie_jar: PrivateCookieJar,
    tucan: State<Tucan>,
    input: Json<Login>,
) -> Result<TsHide<PrivateCookieJar, Json<LoginResult>>, MyError> {
    let tucan_user = tucan.login(&input.username, &input.password).await?;
    let cookie_jar = cookie_jar.add(Cookie::new(
        "session",
        serde_json::to_string(&tucan_user.session)?,
    ));
    Ok(TsHide {
        hidden: cookie_jar,
        visible: Json(LoginResult { success: true }),
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginHack {
    pub session_nr: Option<i64>,
    pub session_id: Option<String>,
    pub redirect: String,
}

async fn login_hack(
    mut cookie_jar: PrivateCookieJar,
    tucan: State<Tucan>,
    input: Query<LoginHack>,
) -> Result<Response, MyError> {
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
        cookie_jar = cookie_jar.add(Cookie::new(
            "session",
            serde_json::to_string(&tucan_user.session)?,
        ));
    }

    let url = match parse_tucan_url(&input.redirect).program {
        tucant::url::TucanProgram::Registration(registration) => Redirect::to(&format!(
            "http://localhost:5173/modules/{}",
            base64::encode_config(registration.path, base64::URL_SAFE_NO_PAD,)
        )),
        tucant::url::TucanProgram::RootRegistration(_) => {
            Redirect::to("http://localhost:5173/modules/")
        }
        tucant::url::TucanProgram::Moduledetails(module_details) => Redirect::to(&format!(
            "http://localhost:5173/module/{}",
            base64::encode_config(module_details.id, base64::URL_SAFE_NO_PAD,)
        )),
        tucant::url::TucanProgram::Coursedetails(course_details) => Redirect::to(&format!(
            "http://localhost:5173/course/{}",
            base64::encode_config(course_details.id, base64::URL_SAFE_NO_PAD,)
        )),
        tucant::url::TucanProgram::Externalpages(_) => Redirect::to("http://localhost:5173/"),
        other => {
            println!("{:?}", other);
            return Ok(StatusCode::NOT_FOUND.into_response());
        }
    };

    Ok((cookie_jar, url).into_response())
}

#[ts]
async fn logout(
    cookie_jar: PrivateCookieJar,
    _input: Json<()>,
) -> Result<TsHide<PrivateCookieJar, Json<()>>, MyError> {
    let cookie_jar = cookie_jar.remove(Cookie::named("session"));
    Ok(TsHide {
        hidden: cookie_jar,
        visible: Json(()),
    })
}

#[ts]
async fn index(cookie_jar: PrivateCookieJar, _input: Json<()>) -> Result<Json<String>, MyError> {
    let session: TucanSession = serde_json::from_str(cookie_jar.get("session").unwrap().value())?;
    Ok(Json(format!("Welcome! {}", session.matriculation_number)))
}

#[derive(Clone, FromRef)]
struct AppState {
    key: Key,
    tucan: Tucan
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

    let app_state = AppState {
        key: secret_key,
        tucan
    };

    let app: Router<AppState> = Router::new().with_state(app_state);

    let app = TypescriptableApp {
        app,
        codes: BTreeSet::new(),
    };

    app.route::<IndexTs>("/", post(index))
        // .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/modules", post(get_modules))
        .route("/search-modules", post(search_module))
        .route("/search-modules-opensearch", post(search_module_opensearch))
        .route("/search-course", post(search_course))
        .route("/course", post(course))
        .route("/module", post(module))
        .route("/my-modules", post(my_modules))
        .route("/my-courses", post(my_courses));
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
        })
        .bind(("localhost", 8080))?
        .run()
        .await?;
    */
    Ok(())
}
