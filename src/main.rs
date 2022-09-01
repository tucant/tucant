use scraper::{ElementRef, Html, Selector};

async fn fetch_document(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;

    Ok(Html::parse_document(&resp))
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

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

async fn handle_veranstaltung(document: &Html) {}

#[async_recursion::async_recursion(?Send)]
async fn traverse_module_list(document: &Html) -> Result<(), Box<dyn std::error::Error>> {
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

                let document =
                    fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", child_url))
                        .await?;

                traverse_module_list(&document).await?;
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

                let document =
                    fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", child_url))
                        .await?;

                handle_veranstaltung(&document).await;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = fetch_document("https://www.tucan.tu-darmstadt.de/").await?;

    let redirect_url = &element_by_selector(&document, r#"meta[http-equiv="refresh"]"#)
        .unwrap()
        .value()
        .attr("content")
        .unwrap()[7..];

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        redirect_url
    ))
    .await?;

    let redirect_url = element_by_selector(&document, r#"h2 a[href]"#)
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        redirect_url
    ))
    .await?;

    let vorlesungsverzeichnis_link = link_by_text(&document, "Vorlesungsverzeichnis (VV)");

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        vorlesungsverzeichnis_link
    ))
    .await?;

    let aktuelles_vorlesungsverzeichnis_link =
        link_by_text(&document, "Aktuell - Wintersemester 2022/23");

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        aktuelles_vorlesungsverzeichnis_link
    ))
    .await?;

    let informatik_link = link_by_text(&document, " FB20 - Informatik ");

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        informatik_link
    ))
    .await?;

    traverse_module_list(&document).await?;

    Ok(())
}
