use std::{
    env,
    io::{self, BufRead},
    sync::Arc,
};

use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::{
    cookie::{self, CookieStore, Jar},
    Client,
};
use scraper::{ElementRef, Html, Selector};
use tokio::sync::{Semaphore, TryAcquireError};

struct TUCAN {
    client: Client,
    cookie_store: Arc<Jar>,
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

impl TUCAN {
    async fn fetch_document(&self, url: &str) -> Result<Html, Box<dyn std::error::Error>> {
        let a = self.client.get(url);
        let b = a.build().unwrap();

        println!("{:?}", b);

        let permit = self.semaphore.acquire().await?;
        let resp = self.client.execute(b).await?.text().await?;
        drop(permit);

        Ok(Html::parse_document(&resp))
    }

    async fn handle_veranstaltung(&self, document: &Html) {
        let name = element_by_selector(&document, "h1").unwrap();

        println!("Name: {}", name.inner_html().trim());
    }

    async fn handle_sublink<'a>(
        &self,
        child: ElementRef<'a>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", child.inner_html());

        let child_url = child.value().attr("href").unwrap();

        //println!("{}", child_url);

        let document = self
            .fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", child_url))
            .await?;

        self.traverse_module_list(&document).await
    }

    #[async_recursion::async_recursion(?Send)]
    async fn traverse_module_list(
        &self,
        document: &Html,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // submenu list
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (Some(list), None) => {
                let mut futures: FuturesUnordered<_> = list
                    .select(&s("a[href]"))
                    .map(|b| self.handle_sublink(b))
                    .collect();
                while let Some(result) = futures.next().await {
                    result?;
                }
            }
            (None, Some(list)) => {
                println!("a {}", document.root_element().html());

                for child in list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#)) {
                    println!("{}", child.inner_html());

                    let child_url = child.value().attr("href").unwrap();

                    println!("{}", child_url);

                    let document = self
                        .fetch_document(&format!("https://www.tucan.tu-darmstadt.de{}", child_url))
                        .await?;

                    self.handle_veranstaltung(&document).await;
                }
            }
            _ => panic!(),
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

        /*
        let document = self
            .fetch_document("https://www.tucan.tu-darmstadt.de/")
            .await?;

        let redirect_url = &element_by_selector(&document, r#"meta[http-equiv="refresh"]"#)
            .unwrap()
            .value()
            .attr("content")
            .unwrap()[7..];

            */

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

        println!("initial useful page {}", document.root_element().html());

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

        let informatik_link = link_by_text(&document, " Wahlbereich");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                informatik_link
            ))
            .await?;

        self.traverse_module_list(&document).await?;

        let informatik_link = link_by_text(&document, " Pflichtbereich");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                informatik_link
            ))
            .await?;

        self.traverse_module_list(&document).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie_store = Arc::new(Jar::default());
    let tucan = TUCAN {
        cookie_store: cookie_store.clone(),
        client: reqwest::Client::builder()
            .cookie_provider(cookie_store)
            .build()?,
        semaphore: Semaphore::new(1),
    };

    tucan.start().await
}
