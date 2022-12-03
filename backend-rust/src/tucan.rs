// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use deadpool::managed::Pool;

use mongodb::options::{ClientOptions, SessionOptions, TransactionOptions, DatabaseOptions, CollectionOptions, InsertOneOptions, UpdateModifications, UpdateOptions};
use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    OpenSearch,
};
use reqwest::{Client, Url};
use tokio::sync::Semaphore;

use crate::{
    models::{TucanSession, UndoneUser, User},
    tucan_user::TucanUser,
    url::{parse_tucan_url, TucanUrl},
};

use dotenvy::dotenv;

async fn create_pool() -> Result<mongodb::Client, mongodb::error::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut client_options = ClientOptions::parse(database_url).await?;

    client_options.app_name = Some("tucant".to_string());
    // TODO FIXME some consistency values probably need to be set here

    mongodb::Client::with_options(client_options)
}

#[derive(Clone)]
pub struct Tucan {
    pub(crate) client: Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub pool: mongodb::Client,
    pub opensearch: OpenSearch,
}

impl std::fmt::Debug for Tucan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tucan").finish()
    }
}

impl Tucan {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = create_pool();

        let url = Url::parse("https://localhost:9200")?;
        let conn_pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(Credentials::Basic("admin".to_string(), "admin".to_string()))
            .cert_validation(CertificateValidation::None)
            .build()?;
        let opensearch = OpenSearch::new(transport);

        Ok(Self {
            pool,
            client: reqwest::Client::builder().build()?,
            semaphore: Arc::new(Semaphore::new(3)),
            opensearch,
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

    pub async fn tucan_session_from_session_data(
        &self,
        session_nr: i64,
        session_id: String,
    ) -> anyhow::Result<TucanUser> {
        let session = TucanSession {
            matriculation_number: -1, // TODO FIXME implement this more cleanly
            session_nr,
            session_id: session_id.clone(),
        };

        let tucan_user = TucanUser {
            tucan: self.clone(),
            session,
        };

        let user = tucan_user.personal_data().await?;

        let session = TucanSession {
            matriculation_number: user.matriculation_number,
            session_nr,
            session_id,
        };

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

                let session_nr = nr.try_into().unwrap();
                let session_id = id.to_string();

                let user = self
                    .tucan_session_from_session_data(session_nr, session_id.clone())
                    .await?;

                let db = self.pool.database_with_options("tucant", DatabaseOptions::builder().build());

                let users_unfinished = db.collection::<UndoneUser>("users_unfinished");
                let sessions = db.collection::<TucanSession>("sessions");

                let value = UndoneUser::new(user.session.matriculation_number);
                let value = bson::to_document(&value)?;
                users_unfinished.update_one(value.clone(), value, UpdateOptions::builder().upsert(Some(true)).build()).await?;

                sessions.insert_one(user.session.clone(), None).await?;

                return Ok(user);
            } else {
                panic!("Failed to extract session_nr");
            }
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }
}
