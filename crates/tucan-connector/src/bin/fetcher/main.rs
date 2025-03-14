use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tucan_connector::TucanConnector;
use tucant_types::coursedetails::CourseDetailsRequest;
use tucant_types::registration::AnmeldungRequest;
use tucant_types::{LoginRequest, Tucan};
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

    let login_response = tucan.login(LoginRequest { username: std::env::var("TUCAN_USERNAME").expect("env variable TUCAN_USERNAME missing"), password: std::env::var("TUCAN_PASSWORD").expect("env variable TUCAN_PASSWORD missing") }).await.unwrap();

    let mut fetcher = Fetcher::new().await?;

    fetcher.recursive_anmeldung(&tucan, &login_response, AnmeldungRequest::new()).await?;

    fetcher.anmeldung_file.flush().await?;
    fetcher.module_file.flush().await?;

    Ok(())
}

struct Fetcher {
    anmeldung_counter: u64,
    anmeldung_file: File,
    module_file: File,
    module_counter: u64,
    course_file: File,
    course_counter: u64,
}

// N675523572713350
// 7159A90AD44826D065E8F1E43AA16A23

impl Fetcher {
    pub async fn new() -> Result<Self, TucanError> {
        Ok(Self {
            anmeldung_counter: 0,
            anmeldung_file: File::options().append(true).create(true).open("anmeldung.log").await?,
            module_file: File::options().append(true).create(true).open("module.log").await?,
            module_counter: 0,
            course_file: File::options().append(true).create(true).open("course.log").await?,
            course_counter: 0,
        })
    }

    // we should retry:
    // Error: Http(reqwest::Error { kind: Decode, source: hyper::Error(Body, Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }) })
    async fn recursive_anmeldung(&mut self, tucan: &TucanConnector, login_response: &LoginResponse, anmeldung_request: AnmeldungRequest) -> Result<(), TucanError> {
        // here we can use cached but for the actual test we can't use cached

        self.anmeldung_file.write_all(anmeldung_request.arguments.as_bytes()).await?;
        self.anmeldung_file.write_all(b"\n").await?;

        println!("anmeldung {}", anmeldung_request.arguments);
        let anmeldung_response = tucan.anmeldung(login_response.clone(), anmeldung_request).await?;
        println!("anmeldung counter: {}", self.anmeldung_counter);
        self.anmeldung_counter += 1;

        for entry in &anmeldung_response.submenus {
            Box::pin(self.recursive_anmeldung(tucan, login_response, entry.1.clone())).await?;
        }

        for entry in &anmeldung_response.entries {
            if let Some(module) = &entry.module {
                println!("module {}", module.url.arguments.clone());
                self.module_file.write_all(module.url.arguments.as_bytes()).await?;
                self.module_file.write_all(b"\n").await?;

                let module_details = tucan.module_details(login_response, module.url.clone()).await?;
                println!("module counter: {}", self.module_counter);
                self.module_counter += 1;
            }

            for course in &entry.courses {
                println!("course {}", course.1.url.clone());
                self.course_file.write_all(course.1.url.as_bytes()).await?;
                self.course_file.write_all(b"\n").await?;

                let course_details = tucan.course_details(login_response, CourseDetailsRequest { arguments: course.1.url.clone() }).await?;
                println!("course counter: {}", self.course_counter);
                self.course_counter += 1;
            }
        }

        Ok(())
    }
}
