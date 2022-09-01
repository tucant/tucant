use std::{
    env,
    io::{self, BufRead},
};

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

struct TUCAN {
    client: Client,
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

        println!("{:?}", b.headers());

        let resp = self.client.execute(b).await?.text().await?;

        Ok(Html::parse_document(&resp))
    }

    async fn handle_veranstaltung(&self, document: &Html) {
        let name = element_by_selector(&document, "h1").unwrap();

        println!("Name: {}", name.inner_html().trim());
    }

    #[async_recursion::async_recursion(?Send)]
    async fn traverse_module_list(
        &self,
        document: &Html,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //println!("{}", document.root_element().html());

        let list = element_by_selector(&document, "#auditRegistration_list");

        match list {
            Some(list) => {
                for child in list.select(&s("li")) {
                    println!("{}", child.value().attr("title").unwrap());

                    let child_url = child
                        .select(&s(r#"a[href]"#))
                        .next()
                        .unwrap()
                        .value()
                        .attr("href")
                        .unwrap();

                    //println!("{}", child_url);

                    let document = self
                        .fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", child_url))
                        .await?;

                    self.traverse_module_list(&document).await?;
                }
            }
            None => {
                for child in document
                    .select(&s(r#"table[class="nb eventTable"]"#))
                    .next()
                    .unwrap()
                    .select(&s(r#"a[name="eventLink"]"#))
                {
                    println!("{}", child.inner_html());

                    let child_url = child.value().attr("href").unwrap();

                    //println!("{}", child_url);

                    let document = self
                        .fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", child_url))
                        .await?;

                    self.handle_veranstaltung(&document).await;
                }
            }
        }
        Ok(())
    }

    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let username = env::args().nth(1).unwrap();

        let password = rpassword::prompt_password("TUCAN password: ").unwrap();

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

        let res = res_headers.text().await?;

        let document = Html::parse_document(&res);

        println!("{}", document.root_element().html());

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de/{}",
                redirect_url
            ))
            .await?;

        println!("{}", document.root_element().html());

        let redirect_url = element_by_selector(&document, r#"h2 a[href]"#)
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de/{}",
                redirect_url
            ))
            .await?;

        let vorlesungsverzeichnis_link = link_by_text(&document, "Vorlesungsverzeichnis (VV)");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de/{}",
                vorlesungsverzeichnis_link
            ))
            .await?;

        let aktuelles_vorlesungsverzeichnis_link =
            link_by_text(&document, "Aktuell - Wintersemester 2022/23");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de/{}",
                aktuelles_vorlesungsverzeichnis_link
            ))
            .await?;

        let informatik_link = link_by_text(&document, " FB20 - Informatik ");

        let document = self
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de/{}",
                informatik_link
            ))
            .await?;

        self.traverse_module_list(&document).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tucan = TUCAN {
        client: reqwest::Client::builder().cookie_store(true).build()?,
    };

    tucan.start().await
}
