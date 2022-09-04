use futures::stream::FuturesUnordered;
use futures::StreamExt;
use scraper::{ElementRef, Html};

use crate::{element_by_selector, link_by_text, s, tucan::Tucan};

pub struct TucanUser {
    pub(crate) tucan: Tucan,
    pub(crate) username: String,
    pub(crate) session_id: String,
    pub(crate) session_nr: i64,
}

#[derive(Debug)]
pub enum RegistrationEnum {
    Submenu(Vec<String>),
    Modules(Vec<String>), // TODO types
}

impl TucanUser {
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
            "SELECT content FROM http_cache WHERE url = ? AND session = ?",
            normalized_url,
            self.session_id
        )
        .fetch_optional(&self.tucan.pool)
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

        let a = self.tucan.client.get(url);
        let b = a.build().unwrap();

        //println!("{:?}", b);

        let permit = self.tucan.semaphore.acquire().await?;
        let resp = self.tucan.client.execute(b).await?.text().await?;
        drop(permit);

        // warning: not transactional with check above
        let cnt = sqlx::query!(
            "INSERT OR REPLACE INTO http_cache (url, session, content) VALUES (?, ?, ?)",
            url,
            self.session_id,
            resp
        )
        .execute(&self.tucan.pool)
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
                        format!(
                            "https://www.tucan.tu-darmstadt.de{}",
                            e.value().attr("href").unwrap()
                        )
                    })
                    .collect(),
            )),
            (Some(list), None) => {
                Ok(RegistrationEnum::Submenu(
                    list.select(&s("a[href]"))
                        .map(|e| {
                            format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
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

    pub async fn registration(&self) -> anyhow::Result<RegistrationEnum> {
        let url = format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N{},-N000311,-A", self.session_nr);

        self.traverse_module_list(&url).await
    }
}
