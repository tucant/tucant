#![feature(try_trait_v2)]

mod csrf_middleware;

use std::io::Error;

use std::fmt::Display;
use std::pin::Pin;

use actix_cors::Cors;

use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::web::{Bytes, Path};
use actix_web::Either;
use actix_web::{cookie::Key, get, post, web, App, HttpResponse, HttpServer, Responder};

use async_stream::try_stream;
use chrono::Utc;
use csrf_middleware::CsrfMiddleware;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use futures::{FutureExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tucan_scraper::models::{Module, ModuleMenu, ModuleMenuEntryModule};
use tucan_scraper::schema::{self};
use tucan_scraper::tucan::Tucan;
use tucan_scraper::tucan_user::{RegistrationEnum, TucanUser};

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
async fn login(
    session: Session,
    tucan: web::Data<Tucan>,
    login: web::Json<Login>,
) -> Result<impl Responder, MyError> {
    let tucan_user = tucan.login(&login.username, &login.password).await?;
    session.insert("tucan_nr", tucan_user.session_nr).unwrap();
    session.insert("tucan_id", tucan_user.session_id).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[post("/logout")]
async fn logout(session: Session) -> Result<impl Responder, MyError> {
    session.purge();
    Ok(HttpResponse::Ok())
}

async fn fetch_everything(
    tucan: TucanUser,
    parent: Option<String>,
    value: RegistrationEnum,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>>>> {
    try_stream(move |mut stream| async move {
        match value {
            RegistrationEnum::Submenu(value) => {
                for (title, url) in value {
                    let tucan_clone = tucan.clone();
                    let parent_clone = parent.clone();
                    let title_clone = title.clone();
                    let url_ref = url.clone();
                    let normalized_name = title
                        .to_lowercase()
                        .replace('-', "")
                        .replace(' ', "-")
                        .replace(',', "")
                        .replace('/', "-")
                        .replace('ä', "ae")
                        .replace('ö', "oe")
                        .replace('ü', "ue");

                    let cnt = tucan_clone
                        .tucan
                        .pool
                        .get()
                        .await
                        .unwrap()
                        .build_transaction()
                        .run::<_, diesel::result::Error, _>(move |connection| {
                            async move {
                                Ok(
                                    diesel::insert_into(tucan_scraper::schema::module_menu::table)
                                        .values(&ModuleMenu {
                                            name: title_clone,
                                            normalized_name,
                                            parent: parent_clone,
                                            tucan_id: url_ref.clone(),
                                            tucan_last_checked: Utc::now().naive_utc(),
                                        })
                                        .get_result::<ModuleMenu>(connection)
                                        .await
                                        .unwrap(),
                                )
                            }
                            .boxed()
                        })
                        .await
                        .unwrap();

                    stream.yield_item(Bytes::from(title)).await;

                    let value = tucan.registration(Some(url.clone())).await.unwrap();
                    let mut inner_stream =
                        fetch_everything(tucan.clone(), Some(cnt.tucan_id), value).await;

                    while let Some(Ok(value)) = inner_stream.next().await {
                        stream.yield_item(value).await;
                    }
                }
            }
            RegistrationEnum::Modules(value) => {
                for (title, url) in value {
                    let tucan_clone = tucan.clone();
                    let parent_clone = parent.clone();
                    stream.yield_item(Bytes::from(title.clone())).await;
                    let module = tucan.clone().module(&url).await.unwrap();

                    // TODO FIXME warn if module already existed as that suggests recursive dependency
                    // TODO normalize url in a way that this can use cached data?
                    // modules can probably be cached because we don't follow outgoing links
                    // probably no infinite recursion though as our menu urls should be unique and therefore hit the cache?
                    tucan_clone
                        .tucan
                        .pool
                        .get()
                        .await
                        .unwrap()
                        .build_transaction()
                        .run::<_, diesel::result::Error, _>(move |connection| {
                            async move {
                                diesel::insert_into(tucan_scraper::schema::modules::table)
                                    .values(&module)
                                    .execute(connection)
                                    .await
                                    .unwrap();

                                diesel::insert_into(
                                    tucan_scraper::schema::module_menu_module::table,
                                )
                                .values(&ModuleMenuEntryModule {
                                    module_id: module.tucan_id,
                                    module_menu_id: parent_clone.unwrap(),
                                })
                                .execute(connection)
                                .await
                                .unwrap();
                                Ok(())
                            }
                            .boxed()
                        })
                        .await
                        .unwrap();
                }
            }
        }
        Ok(())
    })
    .boxed_local()
}

#[post("/setup")]
async fn setup(tucan: web::Data<Tucan>, session: Session) -> Result<impl Responder, MyError> {
    let tucan_nr = session.get::<u64>("tucan_nr").unwrap().unwrap();
    let tucan_id = session.get::<String>("tucan_id").unwrap().unwrap();

    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from("Alle Module werden heruntergeladen..."))
            .await;

        let tucan = tucan.continue_session(tucan_nr, tucan_id).await.unwrap();

        let res = tucan.registration(None).await.unwrap();

        let mut input = fetch_everything(tucan, None, res).await;

        while let Some(Ok(value)) = input.next().await {
            stream.yield_item(value).await;
        }

        let return_value: Result<(), Error> = Ok(());

        return_value
    });

    // TODO FIXME search for <h1>Timeout!</h1>

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(stream))
}

#[get("/")]
async fn index(session: Session) -> Result<impl Responder, MyError> {
    if let Some(user) = session.get::<u64>("tucan_nr").unwrap() {
        Ok(web::Json(format!("Welcome! {}", user)))
    } else {
        Ok(web::Json("Welcome Anonymous!".to_owned()))
    }
}

// trailing slash is menu
#[get("/modules{tail:.*}")]
async fn get_modules<'a>(
    tucan: web::Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await.unwrap();
    println!("{:?}", path);

    let split_path = path.split_terminator('/').map(String::from);
    let menu_path_vec = split_path.skip(1).collect::<Vec<_>>();
    println!("{:?}", menu_path_vec);

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
    println!("{:?}", menu_path);

    let mut node = None;
    for path_segment in menu_path {
        let the_parent = node.map(|v: ModuleMenu| v.tucan_id);

        use self::schema::module_menu::dsl::*;

        node = Some(
            module_menu
                .filter(parent.eq(the_parent).and(normalized_name.eq(path_segment)))
                .load::<ModuleMenu>(&mut connection)
                .await
                .unwrap()
                .into_iter()
                .next()
                .unwrap(),
        )
    }
    let parent = node.map(|v: ModuleMenu| v.tucan_id);

    if let Some(module) = module {
        use self::schema::module_menu_module::dsl::*;
        use self::schema::modules::dsl::*;

        let module_result = module_menu_module
            .inner_join(modules)
            .filter(module_menu_id.eq(parent.unwrap()).and(tucan_id.eq(module)))
            .load::<(ModuleMenuEntryModule, Module)>(&mut connection)
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        Ok(Either::Left(web::Json(module_result)))
    } else {
        let menu_result = tucan
            .pool
            .get()
            .await
            .unwrap()
            .build_transaction()
            .run::<_, diesel::result::Error, _>(move |connection| {
                async move {
                    use self::schema::module_menu::dsl::*;

                    let return_value: Result<Vec<ModuleMenu>, diesel::result::Error> =
                        Ok(module_menu
                            .filter(parent.eq(parent))
                            .load::<ModuleMenu>(connection)
                            .await
                            .unwrap());
                    return_value
                }
                .boxed()
            })
            .await
            .unwrap();

        let module_result = tucan
            .pool
            .get()
            .await
            .unwrap()
            .build_transaction()
            .run::<_, diesel::result::Error, _>(move |connection| {
                async move {
                    use self::schema::module_menu_module::dsl::*;
                    use self::schema::modules::dsl::*;

                    Ok(module_menu_module
                        .inner_join(modules)
                        .filter(module_menu_id.nullable().eq(parent))
                        .load::<(ModuleMenuEntryModule, Module)>(connection)
                        .await
                        .unwrap())
                }
                .boxed()
            })
            .await
            .unwrap();

        if !menu_result.is_empty() {
            Ok(Either::Right(web::Json(RegistrationEnum::Submenu(
                menu_result
                    .iter()
                    .map(|r| (r.name.clone(), r.normalized_name.clone()))
                    .collect::<Vec<_>>(),
            ))))
        } else {
            Ok(Either::Right(web::Json(RegistrationEnum::Modules(
                module_result
                    .iter()
                    .map(|r| (r.1.title.clone(), r.1.module_id.clone()))
                    .collect::<Vec<_>>(),
            ))))
        }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
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
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .allowed_origin("http://localhost:3000");

        App::new()
            .app_data(tucan.clone())
            /*.wrap(
                IdentityMiddleware::builder()
                    .logout_behaviour(LogoutBehaviour::PurgeSession)
                    .build(),
            )*/
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_same_site(SameSite::None)
                    .cookie_secure(true)
                    .cookie_http_only(false)
                    .build(), // TODO FIXME
            )
            .wrap(CsrfMiddleware {})
            .wrap(cors)
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
