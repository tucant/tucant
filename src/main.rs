#![feature(async_closure)]
use std::{env, str::FromStr};

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::Semaphore;

// TODO store raw html for later analysis

struct Tucan {
    client: Client,
    semaphore: Semaphore,
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

impl Tucan {
    async fn fetch_document(&self, url: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let a = self.client.get(url);
        let b = a.build().unwrap();

        //println!("{:?}", b);

        let permit = self.semaphore.acquire().await?;
        let resp = self.client.execute(b).await?.text().await?;
        drop(permit);

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

    async fn handle_sublink<'a>(
        &self,
        child: ElementRef<'a>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("> {}", child.inner_html());

        let child_url = child.value().attr("href").unwrap();

        self.traverse_module_list(&format!("https://www.tucan.tu-darmstadt.de{}", child_url))
            .await
    }

    #[async_recursion::async_recursion(?Send)]
    async fn traverse_module_list(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let username = env::args().nth(1).unwrap();

        let password = env::args().nth(2).unwrap();
        //let password = rpassword::prompt_password("TUCAN password: ").unwrap();

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

        let redirect_url =
            &res_headers.headers().get("refresh").unwrap().to_str()?[7..].to_string();

        res_headers.text().await?;

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                redirect_url
            ))
            .await?;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .test_before_acquire(false)
        .connect_with(SqliteConnectOptions::new().create_if_missing(true))
        .await?;

    sqlx::migrate!().run(&pool).await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{}", row.0);

    let tucan = Tucan {
        client: reqwest::Client::builder().cookie_store(true).build()?,
        semaphore: Semaphore::new(10), // risky
    };

    tucan.start().await
}
