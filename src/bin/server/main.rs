mod csrf_middleware;
mod s_course;
mod s_get_modules;
mod s_search_course;
mod s_search_module;
mod s_setup;

use actix_cors::Cors;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};
use csrf_middleware::CsrfMiddleware;
use diesel_async::pooled_connection::PoolError;
use s_course::course;
use s_get_modules::get_modules;
use s_search_course::search_course;
use s_search_module::search_module;
use s_setup::setup;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucan_scraper::models::{Module, ModuleMenu};
use tucan_scraper::tucan::Tucan;
use tucan_scraper::tucan_user::{TucanSession, TucanUser};
use tucan_scraper::url::{Coursedetails, Moduledetails, Registration};

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

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> MyError {
        MyError { err }
    }
}

impl From<deadpool::managed::PoolError<PoolError>> for MyError {
    fn from(err: deadpool::managed::PoolError<PoolError>) -> MyError {
        MyError { err: err.into() }
    }
}

impl From<diesel::result::Error> for MyError {
    fn from(err: diesel::result::Error) -> MyError {
        MyError { err: err.into() }
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

#[derive(Serialize)]
pub enum ModulesOrModuleMenus {
    Menus(Vec<ModuleMenu>),
    Modules(Vec<Module>),
}

#[post("/login")]
async fn login(
    session: Session,
    tucan: web::Data<Tucan>,
    login: web::Json<Login>,
) -> Result<impl Responder, MyError> {
    let tucan_user = tucan.login(&login.username, &login.password).await?;
    session.insert("session", tucan_user.session).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[post("/logout")]
async fn logout(session: Session) -> Result<impl Responder, MyError> {
    session.purge();
    Ok(HttpResponse::Ok())
}

#[get("/")]
async fn index(session: Session) -> Result<impl Responder, MyError> {
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

        App::new()
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
            .wrap(logger)
            .service(index)
            .service(login)
            .service(logout)
            .service(get_modules)
            .service(setup)
            .service(search_module)
            .service(search_course)
            .service(course)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
