use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::u64;

use futures_util::stream::{self, FuturesUnordered};
use futures_util::{FutureExt, Stream, StreamExt};
use tucan_connector::TucanConnector;
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::registration::{AnmeldungModule, AnmeldungRequest, RegistrationState};
use tucant_types::student_result::StudentResultLevel;
use tucant_types::{LoginRequest, RevalidationStrategy, Tucan};
use tucant_types::{LoginResponse, TucanError};

fn main() -> Result<(), TucanError> {
    dotenvy::dotenv().unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

fn validate(errors: &mut Vec<String>, level: &StudentResultLevel) -> (u64, u64) {
    let mut cp = 0;
    let mut modules = 0;
    for level in &level.children {
        let inner = validate(errors, &level);
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
        errors.push(format!("invalid cp for {}", level.name))
    }
    if modules > level.rules.max_modules.unwrap_or(u64::MAX) || modules < level.rules.min_modules {
        errors.push(format!("invalid module count for {}", level.name))
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

    let course_of_studies = tucan
        .student_result(&login_response, RevalidationStrategy::cache(), 0)
        .await
        .unwrap();
    let bachelor = course_of_studies
        .course_of_study
        .iter()
        .find(|v| v.name == "B.Sc. Informatik (2015)")
        .unwrap()
        .value;
    let student_result = tucan
        .student_result(&login_response, RevalidationStrategy::cache(), bachelor)
        .await
        .unwrap();
    println!("{:#?}", student_result);

    let mut errors = Vec::new();
    validate(&mut errors, &student_result.level0);
    println!("{:#?}", errors);

    let master = course_of_studies
        .course_of_study
        .iter()
        .find(|v| v.name == "M.Sc. Informatik (2023)")
        .unwrap()
        .value;
    let student_result = tucan
        .student_result(&login_response, RevalidationStrategy::cache(), master)
        .await
        .unwrap();
    println!("{:#?}", student_result);

    let fetcher = Arc::new(Fetcher::new());

    let stream = fetcher.recursive_anmeldung(
        tucan,
        login_response,
        AnmeldungRequest::default(),
        String::new(),
    );

    // TODO add modules from fetcher that are not already in leistungsspiegel

    let mut errors = Vec::new();
    validate(&mut errors, &student_result.level0);
    println!("{:#?}", errors);

    Ok(())
}

struct Fetcher {}

impl Fetcher {
    pub const fn new() -> Self {
        Self {}
    }

    fn recursive_anmeldung(
        self: Arc<Self>,
        tucan: TucanConnector,
        login_response: LoginResponse,
        anmeldung_request: AnmeldungRequest,
        path: String,
    ) -> impl Stream<Item = AnmeldungModule> + Send {
        let stream = {
            let tucan = tucan.clone();
            let login_response = login_response.clone();
            async move {
                tucan
                    .anmeldung(
                        login_response.clone(),
                        RevalidationStrategy::cache(),
                        anmeldung_request.clone(),
                    )
                    .await
                    .unwrap()
            }
        }
        .into_stream();
        stream.flat_map(move |anmeldung_response| {
            stream::iter(anmeldung_response.entries.into_iter().filter_map(|entry| {
                if let Some(module) = &entry.module {
                    if matches!(
                        &module.registration_state,
                        RegistrationState::Registered { unregister_link: _ }
                    ) {
                        Some(module.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }))
            .chain(
                stream::iter(
                    anmeldung_response
                        .submenus
                        .into_iter()
                        .filter(|entry| entry.0 != "Zusätzliche Leistungen"),
                )
                .flat_map({
                    let path = path.clone();
                    let self_clone = self.clone();
                    let tucan = tucan.clone();
                    let login_response = login_response.clone();
                    move |entry| {
                        self_clone
                            .clone()
                            .recursive_anmeldung(
                                tucan.clone(),
                                login_response.clone(),
                                entry.1.clone(),
                                path.clone() + " > " + &entry.0,
                            )
                            .boxed()
                    }
                }),
            )
        })
    }
}
