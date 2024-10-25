use key_value_database::Database;

pub mod common;
pub mod externalpages;
pub mod html_handler;
pub mod login;
pub mod mlsstart;
pub mod registration;
pub mod root;
pub mod startpage_dispatch;

pub struct Tucan {
    #[cfg(target_arch = "wasm32")]
    pub client: reqwest::Client,
    #[cfg(not(target_arch = "wasm32"))]
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub database: Database,
}

#[derive(thiserror::Error, Debug)]
pub enum TucanError {
    #[error("HTTP error {0:?}")]
    Http(#[from] reqwest::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("HTTP middleware error {0:?}")]
    HttpMiddleware(#[from] reqwest_middleware::Error),
    #[error("IO error {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Tucan session timeout")]
    Timeout,
}

#[cfg(not(target_arch = "wasm32"))]
type MyClient = reqwest_middleware::ClientWithMiddleware;
#[cfg(target_arch = "wasm32")]
type MyClient = reqwest::Client;

// TODO write small program that converts html to this format? so this is even easier

impl Tucan {
    pub async fn new() -> Result<Self, TucanError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_total_retry_duration_and_max_retries(std::time::Duration::from_secs(
                    90,
                ));
            let client = reqwest_middleware::ClientBuilder::new(
                reqwest::Client::builder()
                    .user_agent(
                        "https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de",
                    )
                    .build()
                    .unwrap(),
            )
            // Retry failed requests.
            .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                retry_policy,
            ))
            .build();
            Ok(Self {
                client,
                database: Database::new().await,
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let client = reqwest::Client::builder()
                .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
                .build()
                .unwrap();
            Ok(Self {
                client,
                database: Database::new().await,
            })
        }

        /*        let username = std::env::var("USERNAME").unwrap();
                let password = std::env::var("PASSWORD").unwrap();

                let result = login(&client, username.as_str(), password.as_str()).await?;
                println!("{:?}", result);
        */

        // TODO FIXME retry on
        // Error: Http(reqwest::Error { kind: Decode, source: hyper::Error(Body, Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }) })

        // 281

        // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N145497569815170,-N000311,-N391343674191079,-N0,-N383963762024372,-N346654580556776
    }
}
