use std::io::{Error, ErrorKind};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection,
};
use dotenvy::dotenv;
use regex::Regex;
use reqwest::{cookie::Jar, Client, Url};

use tokio::sync::Semaphore;

use crate::{create_pool, tucan_user::TucanUser};

pub struct Tucan {
    pub(crate) client: Client,
    pub(crate) semaphore: Semaphore,
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Tucan {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = create_pool();

        Ok(Self {
            pool,
            client: reqwest::Client::builder().build()?,
            semaphore: Semaphore::new(10),
        })
    }

    pub async fn continue_session(
        &self,
        session_nr: u64,
        session_id: String,
    ) -> anyhow::Result<TucanUser> {
        let url = "https://www.tucan.tu-darmstadt.de/scripts"
            .parse::<Url>()
            .unwrap();

        Ok(TucanUser {
            tucan: self,
            session_id: session_id,
            session_nr: session_nr,
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

            println!("{}", redirect_url);

            let url = Url::parse(redirect_url)?;

            let arguments = url.query_pairs().find(|e| e.0 == "ARGUMENTS").unwrap().1;

            println!("{}", arguments);

            let regex: Regex = Regex::new(
                r"(?x)
                ^-N(?P<nr>[[:digit:]]+),-N[[:digit:]]+,-N[[:digit:]]+$
                ",
            )
            .unwrap();

            let session_nr = regex
                .captures(&arguments)
                .and_then(|cap| cap.name("nr").map(|nr| nr.as_str()))
                .unwrap()
                .parse::<u64>()
                .unwrap();

            println!("session_nr {}", session_nr);

            let session_cookie = res_headers.cookies().next().unwrap();
            let session_id = session_cookie.value().to_string();

            res_headers.text().await?;

            return Ok(TucanUser {
                tucan: self,
                session_id,
                session_nr,
            });
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())

        //println!("{:#?}", self.cookie_jar);
    }
}
