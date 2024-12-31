use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tucan_connector::moduledetails::index::moduledetails;
use tucan_connector::registration::index::anmeldung_cached;
use tucan_connector::Tucan;
use tucant_types::registration::AnmeldungRequest;
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
    let tucan = Tucan::new().await?;

    let login_response = LoginResponse {
        id: std::env::var("SESSION_ID").unwrap().parse().unwrap(),
        cookie_cnsc: std::env::var("SESSION_KEY").unwrap(),
    };

    let mut fetcher = Fetcher::new().await?;

    fetcher
        .recursive_anmeldung(&tucan, &login_response, AnmeldungRequest::new())
        .await?;

    fetcher.anmeldung_file.flush().await?;
    fetcher.module_file.flush().await?;

    Ok(())
}

struct Fetcher {
    anmeldung_counter: u64,
    anmeldung_file: File,
    module_file: File,
}

impl Fetcher {
    pub async fn new() -> Result<Self, TucanError> {
        Ok(Self {
            anmeldung_counter: 0,
            anmeldung_file: File::options()
                .append(true)
                .create(true)
                .open("anmeldung.log")
                .await?,
            module_file: File::options()
                .append(true)
                .create(true)
                .open("module.log")
                .await?,
        })
    }

    // we should retry:
    // Error: Http(reqwest::Error { kind: Decode, source: hyper::Error(Body, Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }) })
    async fn recursive_anmeldung(
        &mut self,
        tucan: &Tucan,
        login_response: &LoginResponse,
        anmeldung_request: AnmeldungRequest,
    ) -> Result<(), TucanError> {
        // here we can use cached but for the actual test we can't use cached

        self.anmeldung_file
            .write_all(anmeldung_request.arguments.as_bytes())
            .await?;
        self.anmeldung_file.write_all(b"\n").await?;

        println!("anmeldung {}", anmeldung_request.arguments);
        let anmeldung_response =
            anmeldung_cached(&tucan, &login_response, anmeldung_request).await?;
        println!("counter: {}", self.anmeldung_counter);
        self.anmeldung_counter += 1;

        for entry in &anmeldung_response.submenus {
            for entry in &anmeldung_response.entries {
                if let Some(module) = &entry.module {
                    println!("module {}", module.url.arguments.clone());
                    self.module_file
                        .write_all(module.url.arguments.as_bytes())
                        .await?;
                    self.module_file.write_all(b"\n").await?;

                    let module_details =
                        moduledetails(&tucan, &login_response, module.url.clone()).await?;
                }
            }

            let anmeldung_response =
                Box::pin(self.recursive_anmeldung(&tucan, &login_response, entry.1.clone()))
                    .await?;
        }
        Ok(())
    }
}
