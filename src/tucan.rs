use std::{io::ErrorKind, str::FromStr, sync::Arc};

use regex::Regex;
use reqwest::{cookie::Jar, Client, Url};
use scraper::Html;
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
    pub(crate) pool: Pool<Sqlite>,
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
            "SELECT active_session FROM users WHERE username = ? AND active_session IS NOT NULL",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        match active_session {
            Some(active_session) => {
                let cookie = format!("cnsc={}", active_session.active_session.as_ref().unwrap());
                let url = "https://www.tucan.tu-darmstadt.de/scripts"
                    .parse::<Url>()
                    .unwrap();

                self.cookie_jar.add_cookie_str(&cookie, &url);
                //println!("{:#?}", self.cookie_jar);

                Ok(TucanUser {
                    tucan: self,
                    username: username.to_string(),
                    session_id: active_session.active_session.unwrap(),
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
            ("usrname", &username),
            ("pass", &password),
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
            "INSERT INTO sessions (session_id, user) VALUES (?, ?)",
            session_id,
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

        let redirect_url = &format!(
            "https://www.tucan.tu-darmstadt.de{}",
            &res_headers.headers().get("refresh").unwrap().to_str()?[7..]
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
            .unwrap();

        println!("session_nr {}", session_nr);

        res_headers.text().await?;

        //println!("{:#?}", self.cookie_jar);

        Ok(TucanUser {
            tucan: self,
            username: username.to_string(),
            session_id,
        })
    }

    pub(crate) async fn fetch_document(&self, url: &str) -> anyhow::Result<Html> {
        // TODO FIXME don't do this like that but just cache based on module id that should also be in the title on the previous page
        // maybe try the same with the navigation menus

        let mut normalized_url = url.to_string();
        if normalized_url.contains("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=") {
            normalized_url = normalized_url[0..normalized_url.rfind(",-A").unwrap()].to_string();
            //println!("normalized: {}", normalized_url);
            //println!("url       : {}", url);
        }

        // can't cache these as the links inside there are invalid for new sessions
        /*
        if normalized_url.contains("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=") {
            let start = normalized_url.find("ARGUMENTS=").unwrap() + "ARGUMENTS=".len();
            let end = normalized_url.find(",").unwrap() + 1;
            normalized_url = normalized_url[0..start].to_string() + &normalized_url[end..];
            //println!("normalized: {}", normalized_url);
            //println!("url       : {}", url);
        }*/

        let document = sqlx::query!(
            "SELECT content FROM http_cache WHERE url = ?",
            normalized_url
        )
        .fetch_optional(&self.pool)
        .await?;

        // SELECT url, instr(url, ",-A") FROM http_cache WHERE url LIKE "%MODULEDETAILS%" ORDER BY url;
        // SELECT substr(url, 0, instr(url, ",-A")) AS b, COUNT(*) AS c FROM http_cache WHERE url LIKE "%MODULEDETAILS%" GROUP BY b ORDER BY c DESC;
        // the data at the end is random every login

        // SELECT substr(url, 0, instr(url, "PRGNAME")) FROM http_cache;

        // SELECT substr(url, instr(url, "PRGNAME"), instr(url, "&ARGUMENTS=")-instr(url, "PRGNAME")) AS a, COUNT(*) FROM http_cache GROUP BY a;

        // SELECT url FROM http_cache WHERE url LIKE "%REGISTRATION%" ORDER BY url;

        if let Some(doc) = document {
            return Ok(Html::parse_document(&doc.content));
        } else {
            println!("didnt hit cache")
        }

        let a = self.client.get(url);
        let b = a.build().unwrap();

        //println!("{:?}", b);

        let permit = self.semaphore.acquire().await?;
        let resp = self.client.execute(b).await?.text().await?;
        drop(permit);

        // warning: not transactional with check above
        let cnt = sqlx::query!(
            "INSERT OR REPLACE INTO http_cache (url, content) VALUES (?, ?)",
            url,
            resp
        )
        .execute(&self.pool)
        .await?;
        assert_eq!(cnt.rows_affected(), 1);

        Ok(Html::parse_document(&resp))
    }
}
