use tucan_connector::moduledetails::index::moduledetails;
use tucan_connector::registration::index::anmeldung_cached;
use tucan_connector::{Tucan, TucanError};
use tucant_types::registration::AnmeldungRequest;
use tucant_types::LoginResponse;

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = Tucan::new().await?;

    let result = LoginResponse {
        id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
        cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
    };

    let mut progress = 1;

    let anmeldung_response = anmeldung_cached(&tucan, &result, AnmeldungRequest::new()).await?;

    for entry in &anmeldung_response.submenus {
        let anmeldung_response = anmeldung_cached(&tucan, &result, entry.1.clone()).await?;
        progress += 1;

        for entry in anmeldung_response.submenus {
            let anmeldung_response = anmeldung_cached(&tucan, &result, entry.1.clone()).await?;
            progress += 1;

            for entry in anmeldung_response.entries {
                if let Some(module) = entry.module {
                    println!("fetching");
                    let module_details = moduledetails(&tucan, &result, module.url).await?;
                }
            }
        }
    }

    Ok(())
}
