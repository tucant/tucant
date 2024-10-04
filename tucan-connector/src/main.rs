use reqwest::Response;

fn main() -> Result<(), TucanError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = Tucan::new().await?;
    Ok(())
}

pub struct Tucan {}

#[derive(thiserror::Error, Debug)]
pub enum TucanError {
    #[error("HTTP status code not 200")]
    HttpNotOk(Response),
    #[error("HTTP error {0:?}")]
    Http(#[from] reqwest::Error),
}

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        let resp = reqwest::get("https://www.tucan.tu-darmstadt.de/").await?;
        if resp.status() != 200 {
            return Err(TucanError::HttpNotOk(resp));
        }
        println!("{resp:#?}");
        let content = resp.text().await?;
        println!("{content:#?}");
        Ok(Tucan {})
    }
}
