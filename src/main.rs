use scraper::{Html, Selector};

async fn fetch_document(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
    .await?
    .text()
    .await?;

    Ok(Html::parse_document(&resp))
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

    let document = fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", redirect_url)).await?;

    let redirect_url = &document
        .select(&Selector::parse(r#"h2 a[href]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let document = fetch_document(&format!("https://www.tucan.tu-darmstadt.de/{}", redirect_url)).await?;

    println!("{}", document.root_element().html());

    Ok(())
}
