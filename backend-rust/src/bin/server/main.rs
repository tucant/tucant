// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
mod s_course;
mod s_coursegroup;
mod s_courses;
mod s_exam;
mod s_get_modules;
mod s_module;
mod s_my_courses;
mod s_my_exams;
mod s_my_modules;
mod s_search_course;
mod s_search_module;
mod s_setup;
mod utils;

use axum::Json;

use axum::extract::FromRef;

use axum::extract::Query;
use axum::extract::State;

use axum::http::HeaderValue;
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

use file_lock::FileLock;
use file_lock::FileOptions;
use itertools::Itertools;
use reqwest::header::HeaderName;
use reqwest::header::ACCEPT;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;

use s_course::course;
use s_get_modules::get_modules;
use s_module::module;
use s_my_courses::my_courses;
use s_my_courses::MyCoursesTs;
use s_my_modules::my_modules;
use s_search_course::search_course;
use s_search_course::SearchCourseTs;
use s_search_module::search_module;
use s_search_module::SearchModuleOpensearchTs;

use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;
use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

use tracing::warn;
use tucant::schema::{sessions, users_unfinished};
use tucant::MyError;

use tucant::models::{TucanSession, UndoneUser};

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucant::typescript::TypescriptableApp;

use std::io::Write;
use tucant::tucan::Tucan;
use tucant::url::{parse_tucan_url, Coursedetails, Moduledetails, Registration};
use tucant_derive::{ts, Typescriptable};
use tucant_derive_lib::Typescriptable;

use crate::s_course::CourseTs;
use crate::s_coursegroup::course_group;
use crate::s_coursegroup::CourseGroupTs;
use crate::s_courses::courses;
use crate::s_courses::CoursesTs;
use crate::s_exam::exam;
use crate::s_exam::ExamTs;
use crate::s_get_modules::GetModulesTs;
use crate::s_module::ModuleTs;
use crate::s_my_exams::my_exams;
use crate::s_my_exams::MyExamsTs;
use crate::s_my_modules::MyModulesTs;
use crate::s_search_module::search_module_opensearch;
use crate::s_search_module::SearchModuleTs;
use crate::s_setup::setup;
use base64::prelude::*;

#[derive(Serialize, Typescriptable)]
pub struct WithTucanUrl<T: Typescriptable> {
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
        serde_json::to_string(&tucan_user.state.session)?,
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
    use diesel_async::RunQueryDsl;

    println!("{input:?}");

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
        let tucan_session = tucan_user.state.session.clone();
        let user = UndoneUser::new(tucan_user.state.session.matriculation_number);
        connection
            .build_transaction()
            .run(|mut connection| {
                Box::pin(async move {
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
            serde_json::to_string(&tucan_user.state.session)?,
        ));

        let url = match parse_tucan_url(&input.redirect).program {
            tucant::url::TucanProgram::Registration(registration) => Redirect::to(&format!(
                "http://localhost:5173/modules/{}",
                BASE64_URL_SAFE_NO_PAD.encode(registration.path,)
            )),
            tucant::url::TucanProgram::RootRegistration(_) => {
                Redirect::to("http://localhost:5173/modules/")
            }
            tucant::url::TucanProgram::Moduledetails(module_details) => Redirect::to(&format!(
                "http://localhost:5173/module/{}",
                BASE64_URL_SAFE_NO_PAD.encode(module_details.id,)
            )),
            tucant::url::TucanProgram::Coursedetails(course_details) => Redirect::to(&format!(
                "http://localhost:5173/course/{}",
                BASE64_URL_SAFE_NO_PAD.encode(course_details.id,)
            )),
            tucant::url::TucanProgram::Externalpages(_) => Redirect::to("http://localhost:5173/"),
            tucant::url::TucanProgram::Mlsstart(_) => Redirect::to("http://localhost:5173/"),
            other => {
                println!("unknown redirect for {:?}", other);
                Redirect::to("http://localhost:5173/")
            }
        };

        Ok((cookie_jar, url).into_response())
    } else {
        Ok((
            cookie_jar,
            Redirect::to("http://localhost:5173/not-logged-into-tucan"),
        )
            .into_response())
    }
}

#[ts]
#[allow(clippy::unused_async)]
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
#[allow(clippy::unused_async)]
async fn index(
    session: Option<TucanSession>,
    _cookie_jar: PrivateCookieJar,
    _input: Json<()>,
) -> Result<Json<String>, MyError> {
    Ok(Json(format!(
        "Welcome! {}",
        session.map_or(-1, |v| v.matriculation_number)
    )))
}

#[derive(Clone, FromRef)]
struct AppState {
    key: Key,
    tucan: Tucan,
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            dotenv().ok();
            env_logger::init();

            /*
                // https://crates.io/crates/tracing

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
            let migrations = FileBasedMigrations::find_migrations_directory()?;
            let mut connection = PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));
            connection.run_pending_migrations(migrations).unwrap();

            let random_secret_key = Key::generate();

            let file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open("sessions.key")
                .await;
            if let Ok(mut file) = file {
                file.write_all(random_secret_key.master()).await?;
                drop(file);
            }

            let secret_key_raw = fs::read("sessions.key").await?;
            let secret_key = Key::from(&secret_key_raw);

            let tucan = Tucan::new()?;

            let app_state = AppState {
                key: secret_key,
                tucan,
            };

            let app: Router<AppState> = Router::new()
                .with_state(app_state.clone())
                .route("/setup", post(setup))
                .route("/login-hack", get(login_hack));

            let app = TypescriptableApp {
                app,
                codes: BTreeSet::new(),
            };

            // TODO FIXME csrf protection

            // TODO FIXME these settings are dangerous
            let cors = CorsLayer::new()
                // allow `GET` and `POST` when accessing the resource
                .allow_methods([Method::GET, Method::POST])
                .allow_credentials(true)
                .allow_headers([
                    AUTHORIZATION,
                    ACCEPT,
                    CONTENT_TYPE,
                    "x-csrf-protection".parse::<HeaderName>().unwrap(),
                ])
                // allow requests from any origin
                .allow_origin([
                    "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
                    "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                ]);

            let app = app
                .route::<IndexTs>("/", post(index))
                .route::<LoginTs>("/login", post(login))
                .route::<LogoutTs>("/logout", post(logout))
                .route::<GetModulesTs>("/modules", post(get_modules))
                .route::<SearchModuleTs>("/search-modules", post(search_module))
                .route::<SearchModuleOpensearchTs>(
                    "/search-modules-opensearch",
                    post(search_module_opensearch),
                )
                .route::<SearchCourseTs>("/search-course", post(search_course))
                .route::<CourseTs>("/course", post(course))
                .route::<CourseGroupTs>("/course-group", post(course_group))
                .route::<ModuleTs>("/module", post(module))
                .route::<ExamTs>("/exam", post(exam))
                .route::<MyExamsTs>("/my-exams", post(my_exams))
                .route::<MyModulesTs>("/my-modules", post(my_modules))
                .route::<MyCoursesTs>("/my-courses", post(my_courses))
                .route::<CoursesTs>("/courses", post(courses));

            let should_we_block = true;
            let lock_for_writing = FileOptions::new().write(true).create(true).truncate(true);

            let mut filelock = match FileLock::lock(
                "../frontend-react/src/api.ts",
                should_we_block,
                lock_for_writing,
            ) {
                Ok(lock) => lock,
                Err(err) => panic!("Error getting write lock: {err}"),
            };

            filelock
                .file
                .write_all(
                    (r#"// This file is automatically generated at startup. Do not modify.
import { genericFetch } from "./api_base"
"#
                    .to_string()
                        + &app.codes.into_iter().join("\n"))
                        .as_bytes(),
                )
                .unwrap();

            filelock.unlock().unwrap();

            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(
                    app.app
                        .with_state::<()>(app_state)
                        .layer(cors)
                        //.layer(CompressionLayer::new()) // https://github.com/tower-rs/tower-http/issues/292
                        .into_make_service(),
                )
                .await
                .unwrap();

            Ok(())
        })
}
