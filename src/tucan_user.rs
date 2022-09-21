use std::{
    convert::TryInto,
    io::{Error, ErrorKind},
};

use chrono::Utc;
use reqwest::header::HeaderValue;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::{
    element_by_selector,
    models::Module,
    s,
    tucan::Tucan,
    url::{parse_tucan_url, Moduledetails, Registration, TucanProgram},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TucanSession {
    pub nr: i64,
    pub id: String,
}

#[derive(Clone)]
pub struct TucanUser {
    pub tucan: Tucan,
    pub session: TucanSession,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RegistrationEnum {
    Submenu(Vec<Registration>),
    Modules(Vec<Moduledetails>),
}

impl TucanUser {
    pub(crate) async fn fetch_document(&self, url: &TucanProgram) -> anyhow::Result<Html> {
        let cookie = format!("cnsc={}", self.session.id);

        let a = self
            .tucan
            .client
            .get(url.to_tucan_url(Some(self.session.nr)));
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

    pub async fn module(&self, url: Moduledetails) -> anyhow::Result<Module> {
        let document = self.fetch_document(&url.clone().into()).await?;

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
            tucan_id: url.id,
            tucan_last_checked: Utc::now().naive_utc(),
            title: module_name.unwrap().to_string(),
            credits: Some(credits),
            module_id: module_id.to_string(),
            content,
        })
    }

    pub async fn registration(&self, url: Registration) -> anyhow::Result<RegistrationEnum> {
        let document = self.fetch_document(&url.clone().into()).await?;

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (_, Some(list)) => Ok(RegistrationEnum::Modules(
                list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#))
                    .map(|e| {
                        parse_tucan_url(&format!(
                            "https://www.tucan.tu-darmstadt.de{}",
                            e.value().attr("href").unwrap()
                        ))
                        .program
                        .try_into()
                        .unwrap()
                    })
                    .collect(),
            )),
            (Some(list), None) => {
                Ok(RegistrationEnum::Submenu(
                    list.select(&s("a[href]"))
                        .map(|e| {
                            parse_tucan_url(&format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
                            ))
                            .program
                            .try_into()
                            .unwrap()
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
                panic!(
                    "{:?} {} {}",
                    url.clone(),
                    Into::<TucanProgram>::into(url).to_tucan_url(Some(self.session.nr)),
                    document.root_element().html()
                )
            }
        }
    }
}
