use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use tucan_connector::TucanConnector;
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::registration::{AnmeldungRequest, RegistrationState};
use tucant_types::student_result::StudentResultLevel;
use tucant_types::{LoginRequest, RevalidationStrategy, Tucan};
use tucant_types::{LoginResponse, TucanError};

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async_main())
}

fn validate(level: &StudentResultLevel) {}

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

    let course_of_studies = tucan.student_result(&login_response, RevalidationStrategy::cache(), 0).await.unwrap();
    let bachelor = course_of_studies.course_of_study.iter().find(|v| v.name == "B.Sc. Informatik (2015)").unwrap().value;
    let student_result = tucan.student_result(&login_response, RevalidationStrategy::cache(), bachelor).await.unwrap();
    println!("{:#?}", student_result);

    validate(&student_result.level0);

    Ok(())
}
