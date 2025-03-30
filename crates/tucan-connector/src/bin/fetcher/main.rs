use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use tucan_connector::TucanConnector;
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::registration::{AnmeldungRequest, RegistrationState};
use tucant_types::{LoginRequest, RevalidationStrategy, Tucan};
use tucant_types::{LoginResponse, TucanError};

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

    let fetcher = Arc::new(Fetcher::new());

    fetcher.recursive_anmeldung(&tucan, &login_response, AnmeldungRequest::default(), String::new()).await;

    //fetcher.anmeldung_file.flush().await?;
    //fetcher.module_file.flush().await?;
    //fetcher.course_file.flush().await?;

    Ok(())
}

struct Fetcher {
    anmeldung: AtomicU64,
    module: AtomicU64,
    course: AtomicU64,
}

impl Fetcher {
    pub const fn new() -> Self {
        Self { anmeldung: AtomicU64::new(0), module: AtomicU64::new(0), course: AtomicU64::new(0) }
    }

    #[expect(clippy::manual_async_fn)]
    fn recursive_anmeldung<'a, 'b>(self: Arc<Self>, tucan: &'a TucanConnector, login_response: &'b LoginResponse, anmeldung_request: AnmeldungRequest, path: String) -> impl Future<Output = ()> + Send + use<'a, 'b> {
        async move {
            //self.anmeldung_file.write_all(anmeldung_request.inner().as_bytes()).await?;
            //self.anmeldung_file.write_all(b"\n").await?;

            //println!("anmeldung {}", anmeldung_request.inner());
            let result = AssertUnwindSafe(async { tucan.anmeldung(login_response.clone(), RevalidationStrategy::cache(), anmeldung_request.clone()).await.unwrap() }).catch_unwind().await;
            let anmeldung_response = match result {
                Err(err) => {
                    eprintln!("failed to fetch anmeldung {anmeldung_request} with error {err:?}");
                    return;
                }
                Ok(value) => value,
            };
            //println!("anmeldung counter: {}", self.anmeldung_counter.load(Ordering::Relaxed));
            self.anmeldung.fetch_add(1, Ordering::Relaxed);

            let results: FuturesUnordered<_> = anmeldung_response
                .submenus
                .iter()
                .map(|entry| {
                    async {
                        self.clone().recursive_anmeldung(tucan, login_response, entry.1.clone(), path.clone() + " > " + &entry.0).await;
                    }
                    .boxed()
                })
                .chain(anmeldung_response.entries.iter().map(|entry| {
                    async {
                        if let Some(module) = &entry.module {
                            //println!("module {}", module.url.inner());
                            //self.module_file.write_all(module.url.inner().as_bytes()).await.unwrap();
                            //self.module_file.write_all(b"\n").await.unwrap();

                            if matches!(&module.registration_state, RegistrationState::Registered { unregister_link }) {
                                println!("registered for {} at {}", module.name, path)
                            }

                            let result = AssertUnwindSafe(async { tucan.module_details(login_response, RevalidationStrategy::cache(), module.url.clone()).await.unwrap() }).catch_unwind().await;
                            match result {
                                Err(err) => {
                                    eprintln!("failed to fetch module {} with error {err:?}", module.url);
                                }
                                Ok(module) => {
                                    if module.registered {
                                        //println!("registered for {} at {}", module.module_id, path)
                                    }
                                }
                            }

                            //println!("module counter: {}", self.module_counter.load(Ordering::Relaxed));
                            self.module.fetch_add(1, Ordering::Relaxed);
                        }

                        for course in &entry.courses {
                            //println!("course {}", course.1.url.inner());
                            //self.course_file.write_all(course.1.url.inner().as_bytes()).await.unwrap();
                            //self.course_file.write_all(b"\n").await.unwrap();

                            let result = AssertUnwindSafe(async {
                                let _course_details = tucan.course_details(login_response, RevalidationStrategy::cache(), CourseDetailsRequest::parse(course.1.url.inner())).await.unwrap();
                            })
                            .catch_unwind()
                            .await;
                            if let Err(err) = result {
                                eprintln!("failed to fetch course {} with error {err:?}", course.1.url);
                            }

                            //println!("course counter: {}", self.course_counter.load(Ordering::Relaxed));
                            self.course.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    .boxed()
                }))
                .collect();
            results.collect::<Vec<()>>().await;
        }
    }
}
