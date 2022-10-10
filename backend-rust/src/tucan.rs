// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use deadpool::managed::Pool;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use reqwest::{Client, Url};
use tokio::sync::Semaphore;

use crate::{
    models::{TucanSession, UndoneUser},
    schema::{sessions, users_unfinished},
    tucan_user::TucanUser,
    url::{parse_tucan_url, TucanUrl},
};

use dotenvy::dotenv;

fn get_config() -> AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url)
}

fn create_pool() -> deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let config = get_config();
    Pool::builder(config).build().unwrap()
}

#[derive(Clone)]
pub struct Tucan {
    pub(crate) client: Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl Tucan {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = create_pool();

        Ok(Self {
            pool,
            client: reqwest::Client::builder().build()?,
            semaphore: Arc::new(Semaphore::new(1)),
        })
    }

    pub async fn continue_session(&self, session: TucanSession) -> anyhow::Result<TucanUser> {
        let _url = "https://www.tucan.tu-darmstadt.de/scripts"
            .parse::<Url>()
            .unwrap();

        Ok(TucanUser {
            tucan: self.clone(),
            session,
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<TucanUser> {
        let params: [(&str, &str); 10] = [
            ("usrname", username),
            ("pass", password),
            ("APPNAME", "CampusNet"),
            ("PRGNAME", "LOGINCHECK"),
            (
                "ARGUMENTS",
                "clino,usrname,pass,menuno,menu_type,browser,platform",
            ),
            ("clino", "000000000000001"),
            ("menuno", "000344"),
            ("menu_type", "classic"),
            ("browser", ""),
            ("platform", ""),
        ];
        let res_headers = self
            .client
            .post("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll")
            .form(&params)
            .send()
            .await?;

        let refresh_header = res_headers.headers().get("refresh");

        if refresh_header.is_some() {
            let redirect_url = &format!(
                "https://www.tucan.tu-darmstadt.de{}",
                &refresh_header.unwrap().to_str()?[7..]
            );

            let url = parse_tucan_url(redirect_url);

            if let TucanUrl {
                session_nr: Some(nr),
                ..
            } = url
            {
                let session_cookie = res_headers.cookies().next().unwrap();
                let id = session_cookie.value().to_string();

                res_headers.text().await?;

                let user = UndoneUser::new(username.to_string());

                let session = TucanSession {
                    tu_id: username.to_string(),
                    session_nr: nr.try_into().unwrap(),
                    session_id: id.to_string(),
                };

                use diesel_async::RunQueryDsl;

                let mut connection = self.pool.get().await?;

                {
                    let session = session.clone();
                    connection
                        .build_transaction()
                        .run(|mut connection| {
                            Box::pin(async move {
                                diesel::insert_into(users_unfinished::table)
                                    .values(user)
                                    .on_conflict(users_unfinished::tu_id)
                                    .do_nothing()
                                    .execute(&mut connection)
                                    .await?;

                                diesel::insert_into(sessions::table)
                                    .values(session)
                                    .execute(&mut connection)
                                    .await?;

                                Ok::<(), diesel::result::Error>(())
                            })
                        })
                        .await?;
                }

                return Ok(TucanUser {
                    tucan: self.clone(),
                    session,
                });
            } else {
                panic!("Failed to extract session_nr");
            }
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }
}
