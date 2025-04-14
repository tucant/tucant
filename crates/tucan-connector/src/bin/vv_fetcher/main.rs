use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures_util::stream::FuturesUnordered;
use futures_util::{FutureExt, StreamExt};
use tucan_connector::TucanConnector;
use tucant_types::vv::ActionRequest;
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

    let welcome = tucan.welcome().await.unwrap();

    fetcher
        .recursive_vv(&tucan, &login_response, welcome.vorlesungsverzeichnis_url)
        .await;

    //fetcher.anmeldung_file.flush().await?;
    //fetcher.module_file.flush().await?;
    //fetcher.course_file.flush().await?;

    Ok(())
}

struct Fetcher {
    vv: AtomicU64,
    course: AtomicU64,
}

impl Fetcher {
    pub const fn new() -> Self {
        Self {
            vv: AtomicU64::new(0),
            course: AtomicU64::new(0),
        }
    }

    #[expect(clippy::manual_async_fn)]
    fn recursive_vv<'a, 'b>(
        self: Arc<Self>,
        tucan: &'a TucanConnector,
        login_response: &'b LoginResponse,
        action: ActionRequest,
    ) -> impl Future<Output = ()> + Send + use<'a, 'b> {
        async move {
            //self.anmeldung_file.write_all(anmeldung_request.inner().as_bytes()).await?;
            //self.anmeldung_file.write_all(b"\n").await?;

            //println!("action {}", action);
            let result = AssertUnwindSafe(async {
                tucan
                    .vv(None, RevalidationStrategy::cache(), action.clone())
                    .await
                    .unwrap()
            })
            .catch_unwind()
            .await;
            let anmeldung_response = match result {
                Err(err) => {
                    eprintln!("failed to fetch vv {action} with error {err:?}");
                    return;
                }
                Ok(value) => value,
            };
            self.vv.fetch_add(1, Ordering::Relaxed);
            //println!("anmeldung counter: {}", self.vv.load(Ordering::Relaxed));

            let results: FuturesUnordered<_> = anmeldung_response
                .entries
                .iter()
                .map(|entry| {
                    async {
                        self.clone()
                            .recursive_vv(tucan, login_response, entry.1.clone())
                            .await;
                    }
                    .boxed()
                })
                .chain(
                    anmeldung_response
                        .veranstaltungen_or_module
                        .iter()
                        .map(|entry| {
                            async {
                                let result = AssertUnwindSafe(async {
                                    let _course_details = tucan
                                        .course_details(
                                            login_response,
                                            RevalidationStrategy::cache(),
                                            entry.coursedetails_url.clone(),
                                        )
                                        .await
                                        .unwrap();
                                })
                                .catch_unwind()
                                .await;
                                if let Err(err) = result {
                                    eprintln!(
                                        "failed to fetch course {} with error {err:?}",
                                        entry.coursedetails_url
                                    );
                                }

                                self.course.fetch_add(1, Ordering::Relaxed);
                                //println!("course counter: {}", self.course.load(Ordering::Relaxed));
                            }
                            .boxed()
                        }),
                )
                .collect();
            results.collect::<Vec<()>>().await;
        }
    }
}
