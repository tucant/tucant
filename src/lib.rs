#![feature(async_closure)]

use std::{
    env,
    io::{Error, ErrorKind},
    str::FromStr,
    sync::Arc,
};

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::{cookie::Jar, Client};
use scraper::{ElementRef, Html, Selector};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::sync::Semaphore;

pub struct TucanUser {
    pub username: String,
    pub session_id: String,
}

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn link_by_text<'a>(document: &'a Html, text: &str) -> &'a str {
    document
        .select(&s(r#"a"#))
        .find(|e| e.inner_html() == text)
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

pub struct Tucan {
    pub client: Client,
    pub cookie_jar: Arc<Jar>,
    pub semaphore: Semaphore,
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

    pub async fn continue_session(&self, username: &str) -> anyhow::Result<TucanUser> {
        Err(Box::new(Error::new(ErrorKind::Other, "oh no!")))?
    }

    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<TucanUser> {
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
        let session_id = session_cookie.value();

        let mut tx = self.pool.begin().await?;

        let cnt = sqlx::query!(
            "INSERT OR REPLACE INTO users (username, active_session) VALUES (?, ?)",
            username,
            session_id
        )
        .execute(&mut tx)
        .await?;
        assert_eq!(cnt.rows_affected(), 1);

        let cnt = sqlx::query!(
            "INSERT INTO sessions (session_id, user) VALUES (?, ?)",
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

        res_headers.text().await?;

        Ok(TucanUser {
            username: username.to_string(),
            session_id: "1".to_string(),
        })
    }

    async fn fetch_document(&self, url: &str) -> anyhow::Result<Html> {
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

    async fn handle_veranstaltung(&self, document: &Html) {
        let name = element_by_selector(document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.split("&nbsp;");
        println!("ID: {}", fs.next().unwrap().trim());
        println!("Name: {}", fs.next().unwrap().trim());
        let credits = document
            .select(&s(r#"#contentlayoutleft b"#))
            .find(|e| e.inner_html() == "Credits: ")
            .unwrap()
            .next_sibling()
            .unwrap()
            .value()
            .as_text()
            .unwrap();

        println!("Credits: {}", credits.trim());
        println!("-----------------------");
    }

    async fn handle_sublink<'a>(&self, child: ElementRef<'a>) -> anyhow::Result<()> {
        println!("> {}", child.inner_html());

        let child_url = child.value().attr("href").unwrap();

        self.traverse_module_list(&format!("https://www.tucan.tu-darmstadt.de{}", child_url))
            .await
    }

    #[async_recursion::async_recursion(?Send)]
    async fn traverse_module_list(&self, url: &str) -> anyhow::Result<()> {
        let document = self.fetch_document(url).await?;

        //println!("traverse_module_list {}", document.root_element().html());

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (_, Some(list)) => {
                for child in list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#)) {
                    //println!("{}", child.inner_html());

                    let child_url = child.value().attr("href").unwrap();

                    let document = self
                        .fetch_document(&format!("https://www.tucan.tu-darmstadt.de{}", child_url))
                        .await?;

                    self.handle_veranstaltung(&document).await;
                }
            }
            (Some(list), None) => {
                let selector = s("a[href]");
                let iterat = list
                    .select(&selector)
                    .map(async move |b| self.handle_sublink(b).await);

                let mut futures: FuturesUnordered<_> = iterat.collect();
                while let Some(result) = futures.next().await {
                    result?;
                }

                /*
                while let Some(result) = iterat.next() {
                    result.await?;
                }
                */
            }
            _ => {
                panic!("{} {}", url, document.root_element().html())
            }
        }
        Ok(())
    }

    pub async fn start(&self, redirect_url: &str) -> anyhow::Result<()> {
        let document = self.fetch_document(redirect_url).await?;

        let redirect_url = &element_by_selector(&document, r#".redirect h2 a[href]"#)
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                redirect_url
            ))
            .await?;

        //println!("initial useful page {}", document.root_element().html());

        let vorlesungsverzeichnis_link = link_by_text(&document, "Veranstaltungen");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                vorlesungsverzeichnis_link
            ))
            .await?;

        let aktuelles_vorlesungsverzeichnis_link = link_by_text(&document, "Anmeldung");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                aktuelles_vorlesungsverzeichnis_link
            ))
            .await?;
        {
            let informatik_link = link_by_text(&document, " Wahlbereich");

            self.traverse_module_list(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                informatik_link
            ))
            .await?;
        }

        {
            let informatik_link = link_by_text(&document, " Pflichtbereich");

            self.traverse_module_list(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                informatik_link
            ))
            .await?;
        }
        Ok(())
    }
}
