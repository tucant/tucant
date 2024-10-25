use tucan_connector::login::LoginResponse;
use tucan_connector::registration::index::{anmeldung, anmeldung_cached, AnmeldungRequest};
use tucan_connector::{Tucan, TucanError};

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

    let anmeldung_response =
        anmeldung_cached(&tucan.client, &result, AnmeldungRequest::new()).await?;

    println!("{progress} {anmeldung_response:#?}");
    for entry in &anmeldung_response.submenus {
        let anmeldung_response =
            anmeldung_cached(&tucan.client, &result, entry.1.to_owned()).await?;
        progress += 1;
        println!("{progress} {anmeldung_response:#?}");

        for entry in anmeldung_response.submenus {
            let anmeldung_response =
                anmeldung_cached(&tucan.client, &result, entry.1.to_owned()).await?;
            progress += 1;
            println!("{progress} {anmeldung_response:#?}");

            for entry in anmeldung_response.submenus {
                let anmeldung_response =
                    anmeldung_cached(&tucan.client, &result, entry.1.to_owned()).await?;
                progress += 1;
                println!("{progress} {anmeldung_response:#?}");

                for entry in anmeldung_response.submenus {
                    let anmeldung_response =
                        anmeldung_cached(&tucan.client, &result, entry.1.to_owned()).await?;
                    progress += 1;
                    println!("{progress} {anmeldung_response:#?}");
                }
            }
        }
    }

    Ok(())
}
