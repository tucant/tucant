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
    models::{Module, ModuleMenu},
    s,
    tucan::Tucan,
    url::{parse_tucan_url, Moduledetails, Registration, TucanProgram, TucanUrl, RootRegistration},
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
    Submenu(Vec<ModuleMenu>),
    Modules(Vec<Module>),
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

        let module = Module {
            tucan_id: url.id,
            tucan_last_checked: Utc::now().naive_utc(),
            title: module_name.unwrap().to_string(),
            credits: Some(credits),
            module_id: module_id.to_string(),
            content,
            done: true,
        };

        diesel::insert_into(modules_unfinished::table)
            .values(&module)
            .on_conflict(modules_unfinished::tucan_id)
            .do_update()
            .set(&module)
            .execute(&mut connection)
            .await?;

        module
    }

    pub async fn root_registration(&self) -> anyhow::Result<ModuleMenu> {
        let document = self
            .fetch_document(&RootRegistration { }.into())
            .await?;

        let url_element = element_by_selector(&document, "h2 a:first-child").unwrap();

        let url = parse_tucan_url(&format!(
            "https://www.tucan.tu-darmstadt.de{}",
            url_element.value().attr("href").unwrap()
        ));

        let url = match url {
            TucanUrl {
                program: TucanProgram::Registration(r),
                ..
            } => r,
            _ => panic!(),
        };

        Ok(ModuleMenu {
            tucan_id: url.path,
            tucan_last_checked: Utc::now().naive_utc(),
            name: "TODO".to_string(),
            normalized_name: "TODO".to_string(),
            parent: None,
            child_type: 0,
        })
    }

    pub async fn registration(
        &self,
        url: Registration,
    ) -> anyhow::Result<(ModuleMenu, RegistrationEnum)> {
        let document = self.fetch_document(&url.clone().into()).await?;

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        match (submenu_list, modules_list) {
            (_, Some(list)) => Ok((
                ModuleMenu {
                    tucan_id: url.path,
                    tucan_last_checked: Utc::now().naive_utc(),
                    name: "TODO".to_string(),
                    normalized_name: "TODO".to_string(),
                    parent: None,
                    child_type: 1,
                },
                RegistrationEnum::Modules(
                    list.select(&s(r#"td.tbsubhead.dl-inner a[href]"#))
                        .map(|e| Module {
                            tucan_id: TryInto::<Moduledetails>::try_into(
                                parse_tucan_url(&format!(
                                    "https://www.tucan.tu-darmstadt.de{}",
                                    e.value().attr("href").unwrap()
                                ))
                                .program,
                            )
                            .unwrap()
                            .id,
                            tucan_last_checked: Utc::now().naive_utc(),
                            title: "TODO".to_string(),
                            module_id: "TODO".to_string(),
                            credits: None,
                            content: "TODO".to_string(),
                            done: false,
                        })
                        .collect(),
                ),
            )),
            (Some(list), None) => Ok((
                ModuleMenu {
                    tucan_id: url.path.clone(),
                    tucan_last_checked: Utc::now().naive_utc(),
                    name: "TODO".to_string(),
                    normalized_name: "TODO".to_string(),
                    parent: None,
                    child_type: 2,
                },
                RegistrationEnum::Submenu(
                    list.select(&s("a[href]"))
                        .map(|e| ModuleMenu {
                            tucan_id: TryInto::<Registration>::try_into(
                                parse_tucan_url(&format!(
                                    "https://www.tucan.tu-darmstadt.de{}",
                                    e.value().attr("href").unwrap()
                                ))
                                .program,
                            )
                            .unwrap()
                            .path,
                            tucan_last_checked: Utc::now().naive_utc(),
                            name: "TODO".to_string(),
                            normalized_name: "TODO".to_string(),
                            parent: Some(url.path.clone()),
                            child_type: 0,
                        })
                        .collect(),
                ),
            )),
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
