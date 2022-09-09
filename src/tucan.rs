use std::{
    io::{Error, ErrorKind},
    str::FromStr,
    sync::Arc,
};

use regex::Regex;
use reqwest::{cookie::Jar, Client, Url};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::sync::Semaphore;

use crate::tucan_user::TucanUser;

pub struct Tucan {
    pub(crate) client: Client,
    pub(crate) cookie_jar: Arc<Jar>,
    pub(crate) semaphore: Semaphore,
    pub pool: Pool<Sqlite>,
}

impl Tucan {
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .test_before_acquire(false)
            .connect_with(SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true))
            .await?;

        sqlx::migrate!().run(&pool).await?;

        let cookie_jar = Arc::new(Jar::default());
        Ok(Self {
            cookie_jar: cookie_jar.clone(),
            pool,
            client: reqwest::Client::builder()
                .cookie_provider(cookie_jar)
                .build()?,
            semaphore: Semaphore::new(10), // risky
        })
    }

    pub async fn continue_session(self, username: &str) -> anyhow::Result<TucanUser> {
        let active_session = sqlx::query!(
            "SELECT session_id, session_nr FROM users JOIN sessions ON users.active_session = sessions.session_id WHERE username = ? AND active_session IS NOT NULL",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        match active_session {
            Some(active_session) => {
                let cookie = format!("cnsc={}", active_session.session_id);
                let url = "https://www.tucan.tu-darmstadt.de/scripts"
                    .parse::<Url>()
                    .unwrap();

                self.cookie_jar.add_cookie_str(&cookie, &url);
                //println!("{:#?}", self.cookie_jar);

                Ok(TucanUser {
                    tucan: self,
                    username: username.to_string(),
                    session_id: active_session.session_id,
                    session_nr: active_session.session_nr,
                })
            }
            None => Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "No active session for this user!",
            )))?,
        }
    }

    pub async fn login(self, username: &str, password: &str) -> anyhow::Result<TucanUser> {
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
                .parse::<i64>()
                .unwrap();

            println!("session_nr {}", session_nr);

            let session_cookie = res_headers.cookies().next().unwrap();
            let session_id = session_cookie.value().to_string();

            let mut tx = self.pool.begin().await?;

            sqlx::query!(
                "INSERT OR IGNORE INTO users (username) VALUES (?)",
                username
            )
            .execute(&mut tx)
            .await?;

            let cnt = sqlx::query!(
                "INSERT INTO sessions (session_id, session_nr, user) VALUES (?, ?, ?)",
                session_id,
                session_nr,
                username
            )
            .execute(&mut tx)
            .await?;
            assert_eq!(cnt.rows_affected(), 1);

            let cnt = sqlx::query!(
                "UPDATE users SET active_session = ? WHERE username = ?",
                session_id,
                username
            )
            .execute(&mut tx)
            .await?;
            assert_eq!(cnt.rows_affected(), 1);

            tx.commit().await?;

            res_headers.text().await?;

            return Ok(TucanUser {
                tucan: self,
                username: username.to_string(),
                session_id,
                session_nr,
            });
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())

        //println!("{:#?}", self.cookie_jar);
    }
}
