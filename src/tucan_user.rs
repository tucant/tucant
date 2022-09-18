use std::io::{Error, ErrorKind};

use chrono::Utc;
use reqwest::header::HeaderValue;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::{element_by_selector, models::Module, s, tucan::Tucan};

#[derive(Serialize, Deserialize, Clone)]
pub struct TucanSession {
    pub tucan_nr: u64,
    pub tucan_id: String,
}

#[derive(Clone)]
pub struct TucanUser {
    pub tucan: Tucan,
    pub session: TucanSession,
}

#[derive(Debug, Serialize)]
pub enum RegistrationEnum {
    Submenu(Vec<(String, String)>),
    Modules(Vec<(String, String)>), // TODO types
}

impl TucanUser {
    pub(crate) async fn fetch_document(&self, url: &str) -> anyhow::Result<Html> {
        let _normalized_url = url.to_string();

        let cookie = format!("cnsc={}", self.session.tucan_id);

        let a = self.tucan.client.get(url);
        let mut b = a.build().unwrap();
        b.headers_mut()
            .insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

        //let permit = self.tucan.semaphore.acquire().await?;
        let resp = self.tucan.client.execute(b).await?.text().await?;
        //drop(permit);

        let html_doc = Html::parse_document(&resp);

        if html_doc
            .select(&s("h1"))
            .any(|s| s.inner_html() == "Timeout!")
        {
            return Err(Error::new(ErrorKind::Other, "well we got a timeout here. relogin").into());
        }
        Ok(html_doc)
    }

    pub async fn module(&self, url: &str) -> anyhow::Result<Module> {
        let document = self.fetch_document(url).await?;

        let name = element_by_selector(&document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.split("&nbsp;");
        let module_id = fs.next().unwrap().trim();

        let module_name = fs.next().map(str::trim);

        let credits = document
            .select(&s(r#"#contentlayoutleft b"#))
            .find(|e| e.inner_html() == "Credits: ")
            .unwrap()
            .next_sibling()
            .unwrap()
            .value()
            .as_text()
            .unwrap();

        // Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.
        let credits = credits
            .trim()
            .strip_suffix(",0")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        /* let responsible_person = document
        .select(&s("#dozenten"))
        .next()
        .unwrap()
        .inner_html();*/
        let content = document
            .select(&s("#contentlayoutleft tr.tbdata"))
            .next()
            .unwrap()
            .inner_html();

        Ok(Module {
            tucan_id: module_id.to_string(),
            tucan_last_checked: Utc::now().naive_utc(),
            title: module_name.unwrap().to_string(),
            credits: Some(credits),
            module_id: module_id.to_string(),
            content,
        })
    }

    async fn traverse_module_list(&self, url: &str) -> anyhow::Result<RegistrationEnum> {
        let document = self.fetch_document(url).await?;

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (_, Some(list)) => Ok(RegistrationEnum::Modules(
                list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#))
                    .map(|e| {
                        (
                            e.text()
                                .map(str::to_string)
                                .reduce(|a, b| a + &b)
                                .unwrap_or_default()
                                .trim()
                                .to_string(),
                            format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
                            ),
                        )
                    })
                    .collect(),
            )),
            (Some(list), None) => {
                Ok(RegistrationEnum::Submenu(
                    list.select(&s("a[href]"))
                        .map(|e| {
                            (
                                e.text()
                                    .map(str::to_string)
                                    .reduce(|a, b| a + &b)
                                    .unwrap_or_default()
                                    .trim()
                                    .to_string(),
                                format!(
                                    "https://www.tucan.tu-darmstadt.de{}",
                                    e.value().attr("href").unwrap()
                                ),
                            )
                        })
                        .collect(),
                ))

                /*
                let selector = s("a[href]");
                let iterat = list
                    .select(&selector)
                    .map(async move |b| self.handle_sublink(b).await);

                let mut futures: FuturesUnordered<_> = iterat.collect();
                while let Some(result) = futures.next().await {
                    result?;
                }
                */
            }
            _ => {
                panic!("{} {}", url, document.root_element().html())
            }
        }
    }

    pub async fn registration(&self, url: Option<String>) -> anyhow::Result<RegistrationEnum> {
        let url = url.unwrap_or(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{},-N000311,-A", self.session.tucan_nr));

        self.traverse_module_list(&url).await
    }
}
