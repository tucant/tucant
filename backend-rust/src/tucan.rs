// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use axum::http::HeaderValue;
use deadpool::managed::Pool;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    OpenSearch,
};
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use tokio::sync::Semaphore;

use crate::{
    models::{TucanSession, UndoneUser},
    schema::{sessions, users_unfinished},
    url::{parse_tucan_url, Action, Externalpages, TucanProgram, TucanUrl},
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

pub trait GetTucanSession {
    fn session(&self) -> Option<&TucanSession>;
}

#[derive(Clone)]
pub struct Unauthenticated;

impl GetTucanSession for Unauthenticated {
    fn session(&self) -> Option<&TucanSession> {
        None
    }
}

#[derive(Clone)]
pub struct Authenticated {
    pub session: TucanSession,
}

impl GetTucanSession for Authenticated {
    fn session(&self) -> Option<&TucanSession> {
        Some(&self.session)
    }
}

#[derive(Clone)]
pub struct Tucan<State: GetTucanSession + Sync + Send = Unauthenticated> {
    pub(crate) client: Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub opensearch: OpenSearch,
    pub state: State,
}

impl std::fmt::Debug for Tucan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tucan").finish()
    }
}

impl Tucan<Unauthenticated> {
    pub fn new() -> anyhow::Result<Self> {
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
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Tucant/0.1.0 https://github.com/tucant/tucant")
                .build()?,
            semaphore: Arc::new(Semaphore::new(3)),
            opensearch,
            state: Unauthenticated,
        })
    }
}

#[must_use]
pub fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

impl<State: GetTucanSession + Sync + Send> Tucan<State> {
    pub async fn vv(&self) -> anyhow::Result<()> {
        let document = self
            .fetch_document(&TucanProgram::Externalpages(Externalpages {
                id: 344,
                name: "welcome".to_string(),
            }))
            .await?;
        let document = Self::parse_document(&document)?;

        let vv_link = document
            .select(&s("a"))
            .find(|e| e.inner_html() == "Vorlesungsverzeichnis (VV)")
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let vv_program =
            parse_tucan_url(&format!("https://www.tucan.tu-darmstadt.de{}", vv_link)).program;

        self.action(vv_program).await
    }

    #[async_recursion::async_recursion]
    pub async fn action(&self, url: TucanProgram) -> anyhow::Result<()> {
        if let TucanProgram::Action(Action { magic }) = &url {
            println!("{}", magic)
        }

        let registrations = {
            let document = self.fetch_document(&url).await?;

            let document = Self::parse_document(&document)?;

            document
                .select(&s("#auditRegistration_list li a.auditRegNodeLink"))
                .map(|registration| {
                    //println!("{}", registration.inner_html());

                    parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        registration.value().attr("href").unwrap()
                    ))
                    .program
                })
                .collect_vec()
        };

        let results = registrations
            .into_iter()
            .map(|url| async { self.action(url).await })
            .collect::<FuturesUnordered<_>>();

        let results: Vec<anyhow::Result<()>> = results.collect().await;

        let results: anyhow::Result<Vec<()>> = results.into_iter().collect();

        let results = results?;

        Ok(())
    }

    pub(crate) async fn fetch_document(&self, url: &TucanProgram) -> anyhow::Result<String> {
        let mut request = self
            .client
            .get(
                url.to_tucan_url(
                    self.state
                        .session()
                        .map(|session| session.session_nr.try_into().unwrap()),
                ),
            )
            .build()
            .unwrap();

        if let Some(session) = self.state.session() {
            request.headers_mut().insert(
                "Cookie",
                HeaderValue::from_str(&format!("cnsc={}", session.session_id)).unwrap(),
            );
        }

        let permit = self.semaphore.clone().acquire_owned().await?;
        let resp = self.client.execute(request).await?.text().await?;
        drop(permit);

        Ok(resp)
    }

    pub(crate) fn parse_document(resp: &str) -> anyhow::Result<Html> {
        let html_doc = Html::parse_document(resp);

        if html_doc
            .select(&s("h1"))
            .any(|s| s.inner_html() == "Timeout!")
        {
            return Err(Error::new(ErrorKind::Other, "well we got a timeout here. relogin").into());
            // TODO FIXME propagate error better
        }
        Ok(html_doc)
    }

    #[must_use]
    pub fn continue_session(&self, session: TucanSession) -> Tucan<Authenticated> {
        Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        }
    }

    pub async fn tucan_session_from_session_data(
        &self,
        session_nr: i64,
        session_id: String,
    ) -> anyhow::Result<Tucan<Authenticated>> {
        let session = TucanSession {
            matriculation_number: -1, // TODO FIXME implement this more cleanly
            session_nr,
            session_id: session_id.clone(),
        };

        let tucan_user = Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        };

        let user = tucan_user.personal_data().await?;

        let session = TucanSession {
            matriculation_number: user.matriculation_number,
            session_nr,
            session_id,
        };

        Ok(Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        })
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Tucan<Authenticated>> {
        use diesel_async::RunQueryDsl;

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

                let mut connection = self.pool.get().await?;

                {
                    let user_session = user.state.session.clone();
                    connection
                        .build_transaction()
                        .run(|mut connection| {
                            Box::pin(async move {
                                diesel::insert_into(users_unfinished::table)
                                    .values(UndoneUser::new(
                                        user.state.session.matriculation_number,
                                    ))
                                    .on_conflict(users_unfinished::matriculation_number)
                                    .do_nothing()
                                    .execute(&mut connection)
                                    .await?;

                                diesel::insert_into(sessions::table)
                                    .values(user_session)
                                    .execute(&mut connection)
                                    .await?;

                                Ok::<(), diesel::result::Error>(())
                            })
                        })
                        .await?;
                }

                return Ok(user);
            }
            panic!("Failed to extract session_nr");
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }
}
