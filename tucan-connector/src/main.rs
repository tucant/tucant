use std::time::Duration;

use data_encoding::HEXLOWER;
use html_extractor::html;
use regex::Regex;
use reqwest::{header::HeaderValue, Client};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use scraper::Html;
use tucan_connector::common::head::{html_head, html_head_2};
use tucan_connector::externalpages::studveranst::veranstaltungen;
use tucan_connector::html_handler::Root;
use tucan_connector::login::{login, LoginResponse};
use tucan_connector::mlsstart::start_page::after_login;
use tucan_connector::registration::index::{anmeldung, AnmeldungRequest};
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

    let anmeldung_response = anmeldung(&tucan.client, &result, AnmeldungRequest::new()).await?;

    println!("{progress} {anmeldung_response:#?}");
    for entry in &anmeldung_response.submenus {
        let anmeldung_response = anmeldung(&tucan.client, &result, entry.1.to_owned()).await?;
        progress += 1;
        println!("{progress} {anmeldung_response:#?}");

        for entry in anmeldung_response.submenus {
            let anmeldung_response = anmeldung(&tucan.client, &result, entry.1.to_owned()).await?;
            progress += 1;
            println!("{progress} {anmeldung_response:#?}");

            for entry in anmeldung_response.submenus {
                let anmeldung_response =
                    anmeldung(&tucan.client, &result, entry.1.to_owned()).await?;
                progress += 1;
                println!("{progress} {anmeldung_response:#?}");

                for entry in anmeldung_response.submenus {
                    let anmeldung_response =
                        anmeldung(&tucan.client, &result, entry.1.to_owned()).await?;
                    progress += 1;
                    println!("{progress} {anmeldung_response:#?}");
                }
            }
        }
    }

    Ok(())
}
