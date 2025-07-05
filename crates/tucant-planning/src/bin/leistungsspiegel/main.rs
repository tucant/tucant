use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::u64;

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

fn validate(level: &StudentResultLevel) -> (u64, u64) {
    let mut cp = 0;
    let mut modules = 0;
    for level in &level.children {
        let inner = validate(&level);
        cp += inner.0;
        modules += inner.1;
    }
    for entry in &level.entries {
        if let Some(module_cp) = entry.cp {
            cp += module_cp;
        }
        modules += 1;
    }
    if cp > level.rules.max_cp.unwrap_or(u64::MAX) || cp < level.rules.min_cp {
        panic!("invalid cp")
    }
    if modules > level.rules.max_modules.unwrap_or(u64::MAX) || modules < level.rules.min_modules {
        panic!("invalid module count")
    }
    (cp, modules)
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

    let course_of_studies = tucan.student_result(&login_response, RevalidationStrategy::cache(), 0).await.unwrap();
    let bachelor = course_of_studies.course_of_study.iter().find(|v| v.name == "B.Sc. Informatik (2015)").unwrap().value;
    let student_result = tucan.student_result(&login_response, RevalidationStrategy::cache(), bachelor).await.unwrap();
    println!("{:#?}", student_result);

    validate(&student_result.level0);

    Ok(())
}
