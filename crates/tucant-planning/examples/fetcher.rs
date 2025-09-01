use tucan_connector::TucanConnector;
use tucant_planning::{compress, recursive_anmeldung};
use tucant_types::TucanError;
use tucant_types::registration::AnmeldungRequest;
use tucant_types::{DynTucan, LoginRequest, RevalidationStrategy, Tucan};

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async_main())
}

async fn async_main() -> Result<(), TucanError> {
    let tucan = TucanConnector::new().await?;

    /*let login_response = LoginResponse {
        id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
        cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
    };*/

    let login_response = tucan
        .login(LoginRequest {
            username: std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing"),
            password: std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing"),
        })
        .await
        .unwrap();

    let anmeldung_response = tucan.anmeldung(login_response.clone(), RevalidationStrategy::cache(), AnmeldungRequest::default()).await.unwrap();
    for course_of_study in anmeldung_response.studiumsauswahl {
        let result = recursive_anmeldung(DynTucan::from_ref(&tucan), &login_response, course_of_study.value.clone()).await;
        let content = serde_json::to_string(&result).unwrap();
        tokio::fs::write(
            format!("registration{}_{}.json.br", course_of_study.value, course_of_study.name),
            &compress(content.as_bytes()).await.unwrap(),
        )
        .await
        .unwrap();
    }

    Ok(())
}
