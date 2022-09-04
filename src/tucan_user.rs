use futures::stream::FuturesUnordered;
use futures::StreamExt;
use scraper::{ElementRef, Html};

use crate::{element_by_selector, link_by_text, s, tucan::Tucan};

pub struct TucanUser {
    pub(crate) tucan: Tucan,
    pub(crate) username: String,
    pub(crate) session_id: String,
}

impl TucanUser {
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
        let document = self.tucan.fetch_document(url).await?;

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
                        .tucan
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
        let document = self.tucan.fetch_document(redirect_url).await?;

        let redirect_url = &element_by_selector(&document, r#".redirect h2 a[href]"#)
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        let document = self
            .tucan
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                redirect_url
            ))
            .await?;

        //println!("initial useful page {}", document.root_element().html());

        let vorlesungsverzeichnis_link = link_by_text(&document, "Veranstaltungen");

        let document = self
            .tucan
            .fetch_document(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                vorlesungsverzeichnis_link
            ))
            .await?;

        let aktuelles_vorlesungsverzeichnis_link = link_by_text(&document, "Anmeldung");

        let document = self
            .tucan
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
