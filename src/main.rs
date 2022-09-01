use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.tucan.tu-darmstadt.de/")
        .await?
        .text()
        .await?;
    println!("{}", resp);

    let document = Html::parse_document(&resp);

    let redirect_url = &document
        .select(&Selector::parse(r#"meta[http-equiv="refresh"]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("content")
        .unwrap()[7..];

    println!("{}", redirect_url);

    Ok(())
}
