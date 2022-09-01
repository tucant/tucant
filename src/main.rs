use scraper::{Html, Selector};

async fn fetch_document(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;

    Ok(Html::parse_document(&resp))
}

fn link_by_text<'a>(document: &'a Html, text: &str) -> &'a str {
    document
        .select(&Selector::parse(r#"a"#).unwrap())
        .find(|e| e.inner_html() == text)
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = fetch_document("https://www.tucan.tu-darmstadt.de/").await?;

    let redirect_url = &document
        .select(&Selector::parse(r#"meta[http-equiv="refresh"]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("content")
        .unwrap()[7..];

    let document = fetch_document(&format!(
        "https://www.tucan.tu-darmstadt.de/{}",
        redirect_url
    ))
    .await?;

    let redirect_url = &document
        .select(&Selector::parse(r#"h2 a[href]"#).unwrap())
        .next()
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

    println!("{}", document.root_element().html());

    Ok(())
}
