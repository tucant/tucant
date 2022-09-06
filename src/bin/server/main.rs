#![feature(try_trait_v2)]

mod csrf_middleware;

use std::{fmt::Display, time::Duration};

use actix_cors::Cors;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::web::{Bytes, Path};
use actix_web::HttpMessage;
use actix_web::{
    cookie::Key, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use async_recursion::async_recursion;
use csrf_middleware::CsrfMiddleware;
use futures::channel::mpsc::{unbounded, UnboundedSender};

use futures::SinkExt;
use serde::{Deserialize, Serialize};

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
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
async fn login(request: HttpRequest, login: web::Json<Login>) -> Result<impl Responder, MyError> {
    let tucan = Tucan::new().await?;
    tucan.login(&login.username, &login.password).await?;
    Identity::login(&request.extensions(), login.username.to_string()).unwrap();
    Ok(web::Json(LoginResult { success: true }))
}

#[post("/logout")]
async fn logout(user: Identity) -> Result<impl Responder, MyError> {
    user.logout();
    Ok(HttpResponse::Ok())
}

#[async_recursion]
async fn fetch_everything(
    username: &str,
    tucan: &TucanUser,
    mut sender: UnboundedSender<Result<actix_web::web::Bytes, std::io::Error>>,
    parent: Option<i64>,
    value: RegistrationEnum,
) -> Result<(), MyError> {
    match value {
        RegistrationEnum::Submenu(value) => {
            for (title, url) in value {
                let normalized_name = title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(",", "")
                    .replace("-", "");

                // TODO FIXME we need to add username to primary key for this and modules
                let cnt = sqlx::query!(
                    "INSERT INTO module_menu
                    (username, name, normalized_name, parent)
                    VALUES
                    (?1, ?2, ?3, ?4)
                    ON CONFLICT (username, parent, name) DO UPDATE SET
                    name = ?2,
                    normalized_name = ?3,
                    parent = ?4
                    ",
                    username,
                    title,
                    normalized_name,
                    parent
                )
                .execute(&tucan.tucan.pool)
                .await
                .unwrap();
                assert_eq!(cnt.rows_affected(), 1);

                sender.send(Ok(Bytes::from(title))).await.unwrap();
                let value = tucan.registration(Some(url)).await?;
                fetch_everything(
                    username,
                    tucan,
                    sender.clone(),
                    Some(cnt.last_insert_rowid()),
                    value,
                )
                .await?;
            }
        }
        RegistrationEnum::Modules(value) => {
            for (title, url) in value {
                sender.send(Ok(Bytes::from(title.clone()))).await.unwrap();
                let module = tucan.module(&url).await?;

                let cnt = sqlx::query!(
                    "INSERT INTO modules
                    (username, title, module_id, shortcode, credits, responsible_person, content)
                    VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                    ON CONFLICT (module_id) DO UPDATE SET
                    title = ?2,
                    shortcode = ?4,
                    credits = ?5,
                    responsible_person = ?6,
                    content = ?7
                    ",
                    username,
                    module.name,
                    module.id,
                    title,
                    module.credits,
                    module.responsible_person,
                    module.content
                )
                .execute(&tucan.tucan.pool)
                .await
                .unwrap();
                assert_eq!(cnt.rows_affected(), 1);
            }
        }
    }
    Ok::<(), MyError>(())
}

#[post("/setup")]
async fn setup(user: Option<Identity>) -> Result<impl Responder, MyError> {
    if let Some(user) = user {
        let (mut sender, receiver) = unbounded::<Result<actix_web::web::Bytes, std::io::Error>>();

        let user_id = user.id()?;
        tokio::spawn(async move {
            sender
                .send(Ok(Bytes::from("Alle Module werden heruntergeladen...")))
                .await
                .unwrap();

            let tucan = Tucan::new().await?;
            let tucan = tucan.continue_session(&user_id).await?;

            let res = tucan.registration(None).await?;
            fetch_everything(&user_id, &tucan, sender.clone(), None, res).await?;

            Ok::<(), MyError>(())
        });

        // TODO FIXME search for <h1>Timeout!</h1>

        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .streaming(receiver))
    } else {
        Err(anyhow::Error::msg("Not logged in!"))?
    }
}

#[get("/")]
async fn index(user: Option<Identity>) -> Result<impl Responder, MyError> {
    if let Some(user) = user {
        Ok(web::Json(format!("Welcome! {}", user.id().unwrap())))
    } else {
        Ok(web::Json("Welcome Anonymous!".to_owned()))
    }
}

#[derive(Debug)]
struct MenuItem {
    id: i64,
    normalized_name: String,
}

// trailing slash is menu
#[get("/modules/{tail:.*}")]
async fn modules(user: Option<Identity>, path: Path<String>) -> Result<impl Responder, MyError> {
    if let Some(user) = user {
        // TODO FIXME put this in app data so we don't open countless db pools
        let tucan = Tucan::new().await?;

        println!("{:?}", path.split_terminator("/").collect::<Vec<_>>());

        let user_id = user.id()?;
        let mut node = None;
        for path_segment in path.split_terminator("/") {
            let parent = node.map(|v: MenuItem| v.id);
            node = Some(sqlx::query_as!(MenuItem, "SELECT id, normalized_name FROM module_menu WHERE username = ?1 AND parent IS ?2 AND normalized_name = ?3", user_id, parent, path_segment)
            .fetch_one(&tucan.pool).await.unwrap()); // TODO FIXME these unwraps
        }

        let parent = node.map(|v: MenuItem| v.id);
        let result = sqlx::query!(
            "SELECT id, name, normalized_name FROM module_menu WHERE username = ?1 AND parent IS ?2",
            user_id,
            parent
        )
        .fetch_all(&tucan.pool)
        .await
        .unwrap(); // TODO FIXME these unwraps

        Ok(web::Json(
            result
                .iter()
                .map(|r| vec![r.name.clone(), r.normalized_name.clone()])
                .collect::<Vec<_>>(),
        ))
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
        file.write_all(random_secret_key.master()).await?;
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
            .service(modules)
            .service(setup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
