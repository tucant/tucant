#![feature(try_trait_v2)]

mod csrf_middleware;

use std::io::Error;

use std::fmt::Display;

use std::pin::Pin;

use actix_cors::Cors;

use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::web::{Bytes, Path};
use actix_web::Either;
use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};

use async_stream::{try_stream};
use chrono::Utc;
use csrf_middleware::CsrfMiddleware;
use diesel::dsl::not;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::pooled_connection::PoolError;
use diesel_async::RunQueryDsl;
use futures::{FutureExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tucan_scraper::schema::*;

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucan_scraper::models::{Module, ModuleMenu, ModuleMenuEntryModule};
use tucan_scraper::tucan::Tucan;
use tucan_scraper::tucan_user::{RegistrationEnum, TucanSession, TucanUser};
use tucan_scraper::url::{Moduledetails, Registration};

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
    Modules(Vec<(ModuleMenuEntryModule, Module)>),
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

async fn fetch_module(
    tucan: TucanUser,
    parent: Registration,
    module: Moduledetails,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, MyError>>>> {
    try_stream(move |mut stream| async move {
        let tucan_clone = tucan.clone();
        let parent_clone = parent.clone();
        stream
            .yield_item(Bytes::from(format!("module {}", module.id)))
            .await;

        // TODO FIXME check if module already fetched and in cache

        let module = tucan.clone().module(module).await.unwrap();

        // TODO FIXME warn if module already existed as that suggests recursive dependency
        // TODO normalize url in a way that this can use cached data?
        // modules can probably be cached because we don't follow outgoing links
        // probably no infinite recursion though as our menu urls should be unique and therefore hit the cache?
        tucan_clone
            .tucan
            .pool
            .get()
            .await?
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|connection| {
                async move {
                    diesel::insert_into(modules_unfinished::table)
                        .values(&module)
                        .on_conflict(modules_unfinished::tucan_id)
                        .do_update()
                        .set(modules_unfinished::content.eq(excluded(modules_unfinished::content)))
                        .execute(connection)
                        .await?;

                    diesel::insert_into(module_menu_module::table)
                        .values(&ModuleMenuEntryModule {
                            module_id: module.tucan_id,
                            module_menu_id: parent_clone.path.unwrap().to_vec(),
                        })
                        .on_conflict_do_nothing()
                        .execute(connection)
                        .await?;
                    Ok(())
                }
                .boxed()
            })
            .await?;
        Ok(())
    })
    .boxed_local()
}

async fn fetch_registration(
    tucan: TucanUser,
    parent: Registration,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, MyError>>>> {
    try_stream(move |mut stream| async move {
        let tucan_clone = tucan.clone();
        let _parent_clone = parent.clone();

        // TODO check if already in DB and cache good
        /*
                            let normalized_name = title
                                .to_lowercase()
                                .replace('-', "")
                                .replace(' ', "-")
                                .replace(',', "")
                                .replace('/', "-")
                                .replace('ä', "ae")
                                .replace('ö', "oe")
                                .replace('ü', "ue");
        */
        let connection = &mut tucan_clone.tucan.pool.get().await?;

        let existing_registration_already_fetched = module_menu_unfinished::table
            .filter(
                module_menu_unfinished::tucan_id
                    .nullable()
                    .eq(parent.clone().path),
            )
            .filter(not(module_menu_unfinished::child_type.eq(0)))
            .get_result::<ModuleMenu>(connection)
            .await
            .optional()?;

        match existing_registration_already_fetched {
            Some(ModuleMenu { child_type: 1, .. }) => {
                // existing submenus
                let submenus = module_menu_unfinished::table
                    .filter(module_menu_unfinished::parent.eq(parent.clone().path.unwrap()))
                    .load::<ModuleMenu>(connection)
                    .await?;
                for submenu in submenus {
                    let mut fetch_registration_stream = fetch_registration(
                        tucan.clone(),
                        Registration {
                            path: Some(submenu.tucan_id),
                        },
                    )
                    .await;

                    loop {
                        match fetch_registration_stream.next().await {
                            Some(Ok(value)) => {
                                stream.yield_item(value).await;
                            }
                            Some(err @ Err(_)) => {
                                err?;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
            }
            Some(ModuleMenu { child_type: 2, .. }) => {
                // existing submodules
                let submodules = module_menu_module::table
                    .inner_join(modules_unfinished::table)
                    .filter(
                        module_menu_module::module_menu_id
                            .nullable()
                            .eq(parent.clone().path.unwrap()),
                    )
                    .load::<(ModuleMenuEntryModule, Module)>(connection)
                    .await?;

                // TODO FIXME maybe store everything up until here in a registration enum and then unify the logic?
                for module in submodules {
                    let mut fetch_module_stream = fetch_module(
                        tucan.clone(),
                        parent.clone(),
                        Moduledetails {
                            id: module.1.tucan_id,
                        },
                    )
                    .await;

                    loop {
                        match fetch_module_stream.next().await {
                            Some(Ok(value)) => {
                                stream.yield_item(value).await;
                            }
                            Some(err @ Err(_)) => {
                                err?;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
            }
            Some(_) => panic!(),
            None => {
                // don't know children yet, fetch them
                let value = tucan.registration(parent.clone()).await?;

                // mark current element as finished
                tucan_clone
                    .tucan
                    .pool
                    .get()
                    .await?
                    .build_transaction()
                    .run::<_, diesel::result::Error, _>(|connection| {
                        let child_type = match value {
                            RegistrationEnum::Submenu(_) => 1,
                            RegistrationEnum::Modules(_) => 2,
                        };
                        let parent = parent.path.clone().unwrap();
                        async move {
                            diesel::insert_into(module_menu_unfinished::table)
                                .values(&ModuleMenu {
                                    name: "".to_string(),
                                    normalized_name: "".to_string(),
                                    parent: Some(parent.clone()), // TODO FIXMe simply not modify this (maybe use the other update syntax)
                                    tucan_id: parent,
                                    tucan_last_checked: Utc::now().naive_utc(),
                                    child_type,
                                })
                                .on_conflict(module_menu_unfinished::tucan_id)
                                .do_update()
                                .set(
                                    module_menu_unfinished::name
                                        .eq(excluded(module_menu_unfinished::name)),
                                )
                                .get_result::<ModuleMenu>(connection)
                                .await
                        }
                        .boxed()
                    })
                    .await?;

                match value {
                    RegistrationEnum::Submenu(ref submenu) => {
                        diesel::insert_into(module_menu_unfinished::table)
                            .values(
                                submenu
                                    .iter()
                                    .map(|s| ModuleMenu {
                                        name: "".to_string(),
                                        normalized_name: "".to_string(),
                                        parent: parent.clone().path,
                                        tucan_id: s.clone().path.unwrap(),
                                        tucan_last_checked: Utc::now().naive_utc(),
                                        child_type: match &value {
                                            RegistrationEnum::Submenu(_) => 1,
                                            RegistrationEnum::Modules(_) => 2,
                                        },
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .on_conflict(module_menu_unfinished::tucan_id)
                            .do_update()
                            .set(
                                module_menu_unfinished::name
                                    .eq(excluded(module_menu_unfinished::name)),
                            )
                            .get_result::<ModuleMenu>(connection)
                            .await?;

                        for menu in submenu {
                            let mut fetch_registration_stream =
                                fetch_registration(tucan.clone(), menu.clone()).await;

                            loop {
                                match fetch_registration_stream.next().await {
                                    Some(Ok(value)) => {
                                        stream.yield_item(value).await;
                                    }
                                    Some(err @ Err(_)) => {
                                        err?;
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    RegistrationEnum::Modules(modules) => {
                        diesel::insert_into(modules_unfinished::table)
                            .values(
                                modules
                                    .iter()
                                    .map(|m| Module {
                                        done: false,
                                        tucan_id: m.id,
                                        tucan_last_checked: Utc::now().naive_utc(),
                                        title: "".to_string(),
                                        module_id: "".to_string(),
                                        credits: None,
                                        content: "".to_string(),
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .execute(connection)
                            .await?;

                        // TODO FIXME transaction
                        diesel::insert_into(module_menu_module::table)
                            .values(
                                modules
                                    .iter()
                                    .map(|m| ModuleMenuEntryModule {
                                        module_id: m.id,
                                        module_menu_id: parent.clone().path.unwrap(),
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .on_conflict_do_nothing()
                            .execute(connection)
                            .await?;

                        for module in modules {
                            let mut fetch_module_stream =
                                fetch_module(tucan.clone(), parent.clone(), module).await;

                            loop {
                                match fetch_module_stream.next().await {
                                    Some(Ok(value)) => {
                                        stream.yield_item(value).await;
                                    }
                                    Some(err @ Err(_)) => {
                                        err?;
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
    .boxed_local()
}

#[post("/setup")]
async fn setup(tucan: web::Data<Tucan>, session: Session) -> Result<impl Responder, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => {
            let stream = try_stream(move |mut stream| async move {
                stream
                    .yield_item(Bytes::from("Alle Module werden heruntergeladen..."))
                    .await;

                let tucan = tucan.continue_session(session).await.unwrap();

                let mut input = fetch_registration(tucan, Registration { path: None }).await;

                loop {
                    match input.next().await {
                        Some(Ok(value)) => {
                            stream.yield_item(value).await;
                        }
                        Some(Err(err)) => {
                            Err::<(), MyError>(err).unwrap(); // TODO FIXME
                        }
                        None => {
                            break;
                        }
                    }
                }

                stream.yield_item(Bytes::from("Fertig!")).await;

                let return_value: Result<(), Error> = Ok(());

                return_value
            });

            // TODO FIXME search for <h1>Timeout!</h1>

            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .streaming(stream))
        }
        None => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("not logged in")),
    }
}

#[get("/")]
async fn index(session: Session) -> Result<impl Responder, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => Ok(web::Json(format!("Welcome! {}", session.nr))),
        None => Ok(web::Json("Welcome Anonymous!".to_owned())),
    }
}

// trailing slash is menu
#[get("/modules{tail:.*}")]
async fn get_modules<'a>(
    tucan: web::Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await?;

    let split_path = path.split_terminator('/').map(String::from);
    let menu_path_vec = split_path.skip(1).collect::<Vec<_>>();

    let menu_path: Vec<String>;
    let module: Option<&str>;
    if path.ends_with('/') {
        menu_path = menu_path_vec;
        module = None;
    } else {
        let tmp = menu_path_vec.split_last().unwrap();
        menu_path = tmp.1.to_vec();
        module = Some(tmp.0);
    }

    let mut node = None;
    for path_segment in menu_path {
        let the_parent = node.map(|v: ModuleMenu| v.tucan_id);

        node = Some(
            module_menu_unfinished::table
                .filter(
                    module_menu_unfinished::parent
                        .eq(the_parent)
                        .and(module_menu_unfinished::normalized_name.eq(path_segment)),
                )
                .load::<ModuleMenu>(&mut connection)
                .await?
                .into_iter()
                .next()
                .unwrap(),
        )
    }
    let parent = node.map(|v: ModuleMenu| v.tucan_id);

    if let Some(module) = module {
        let module_result = module_menu_module::table
            .inner_join(modules_unfinished::table)
            .filter(
                module_menu_module::module_menu_id
                    .eq(parent.unwrap())
                    .and(modules_unfinished::module_id.eq(module)),
            )
            .load::<(ModuleMenuEntryModule, Module)>(&mut connection)
            .await?
            .into_iter()
            .next()
            .unwrap();

        Ok(Either::Left(web::Json(module_result)))
    } else {
        let menu_result = tucan
            .pool
            .get()
            .await?
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|connection| {
                let parent = parent.clone();
                async move {
                    let return_value: Result<Vec<ModuleMenu>, diesel::result::Error> =
                        Ok(module_menu_unfinished::table
                            .filter(module_menu_unfinished::parent.eq(parent))
                            .load::<ModuleMenu>(connection)
                            .await?);
                    return_value
                }
                .boxed()
            })
            .await?;

        let module_result = tucan
            .pool
            .get()
            .await?
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|connection| {
                let parent = parent.clone();
                async move {
                    module_menu_module::table
                        .inner_join(modules_unfinished::table)
                        .filter(module_menu_module::module_menu_id.nullable().eq(parent))
                        .load::<(ModuleMenuEntryModule, Module)>(connection)
                        .await
                }
                .boxed()
            })
            .await?;

        if !menu_result.is_empty() {
            Ok(Either::Right(web::Json(ModulesOrModuleMenus::Menus(
                menu_result,
            ))))
        } else {
            Ok(Either::Right(web::Json(ModulesOrModuleMenus::Modules(
                module_result,
            ))))
        }
    }
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
            .allowed_origin("http://localhost:3000");

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
