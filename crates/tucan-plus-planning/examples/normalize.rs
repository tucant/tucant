use futures_util::StreamExt as _;
use futures_util::stream::{FuturesOrdered, FuturesUnordered};
use tucan_connector::TucanConnector;
use tucan_plus_planning::{compress, recursive_anmeldung};
use tucan_types::TucanError;
use tucan_types::registration::{AnmeldungRequest, AnmeldungResponse};
use tucan_types::{DynTucan, LoginRequest, RevalidationStrategy, Tucan};

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let args: Vec<String> = std::env::args().collect();
    let results: FuturesOrdered<_> = args
        .iter()
        .skip(1)
        .map(async |arg| {
            serde_json::from_str(&tokio::fs::read_to_string(arg).await.unwrap()).unwrap()
        })
        .collect();
    let results = results.collect::<Vec<Vec<AnmeldungResponse>>>().await;
    println!("{:?}", results);

    Ok(())
}
