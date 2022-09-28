use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use deadpool::managed::Pool;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use reqwest::{Client, Url};
use tokio::sync::Semaphore;

use crate::{
    create_pool,
    tucan_user::{TucanSession, TucanUser},
    url::{parse_tucan_url, TucanUrl},
};

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
            semaphore: Arc::new(Semaphore::new(5)),
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

                return Ok(TucanUser {
                    tucan: self.clone(),
                    session: TucanSession { id, nr },
                });
            } else {
                panic!("Failed to extract session_nr");
            }
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }
}
