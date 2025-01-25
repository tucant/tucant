use coursedetails::index::course_details;
use key_value_database::Database;
use login::{login, logout};
use mlsstart::start_page::after_login;
use moduledetails::index::module_details;
use registration::index::anmeldung;
use reqwest::header;
use tucant_types::{Tucan, TucanError};

pub mod common;
pub mod coursedetails;
pub mod externalpages;
pub mod html_handler;
pub mod login;
pub mod mlsstart;
pub mod moduledetails;
pub mod registration;
pub mod root;
pub mod startpage_dispatch;

#[cfg(not(target_arch = "wasm32"))]
type MyClient = reqwest_middleware::ClientWithMiddleware;
#[cfg(target_arch = "wasm32")]
type MyClient = reqwest::Client;

pub struct TucanConnector {
    pub client: MyClient,
    pub database: Database,
}

impl TucanConnector {
    pub async fn new() -> Result<Self, TucanError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept-Language",
            header::HeaderValue::from_static("de-DE,de;q=0.5"),
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_total_retry_duration_and_max_retries(std::time::Duration::from_secs(
                    90,
                ));
            let client = reqwest_middleware::ClientBuilder::new(
                reqwest::Client::builder()
                    .default_headers(headers)
                    .user_agent(
                        "https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de",
                    )
                    .build()
                    .unwrap(),
            )
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
                .default_headers(headers)
                .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
                .build()
                .unwrap();
            Ok(Self {
                client,
                database: Database::new().await,
            })
        }
    }
}

impl Tucan for TucanConnector {
    async fn login(
        &self,
        request: tucant_types::LoginRequest,
    ) -> Result<tucant_types::LoginResponse, TucanError> {
        login(&self.client, &request).await
    }

    async fn after_login(
        &self,
        request: &tucant_types::LoginResponse,
    ) -> Result<tucant_types::LoggedInHead, TucanError> {
        after_login(&self.client, request).await
    }

    async fn logout(&self, request: &tucant_types::LoginResponse) -> Result<(), TucanError> {
        logout(&self.client, request).await
    }

    async fn anmeldung(
        &self,
        login_response: tucant_types::LoginResponse,
        request: tucant_types::registration::AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, TucanError> {
        anmeldung(self, &login_response, request).await
    }

    async fn module_details(
        &self,
        login_response: &tucant_types::LoginResponse,
        request: tucant_types::moduledetails::ModuleDetailsRequest,
    ) -> Result<tucant_types::moduledetails::ModuleDetailsResponse, TucanError> {
        module_details(self, login_response, request).await
    }

    async fn course_details(
        &self,
        login_response: &tucant_types::LoginResponse,
        request: tucant_types::coursedetails::CourseDetailsRequest,
    ) -> Result<tucant_types::coursedetails::CourseDetailsResponse, TucanError> {
        course_details(self, login_response, request).await
    }
}

#[cfg(test)]
mod tests {
    use tucant_types::{
        coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest, LoginRequest,
        LoginResponse, TucanError,
    };

    use crate::{
        externalpages::welcome::welcome, login::login, root::root,
        startpage_dispatch::one::startpage_dispatch_1, Tucan, TucanConnector,
    };

    #[tokio::test]
    pub async fn login_incorrect() {
        let tucan = TucanConnector::new().await.unwrap();
        assert!(matches!(
            login(
                &tucan.client,
                &LoginRequest {
                    username: "not_found".to_owned(),
                    password: "not_correct".to_owned(),
                },
            )
            .await,
            Err(TucanError::InvalidCredentials)
        ));
    }

    #[tokio::test]
    pub async fn test_root_page() {
        let tucan = TucanConnector::new().await.unwrap();
        root(&tucan.client).await.unwrap();
    }

    /// /
    /// redirects to
    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
    #[tokio::test]
    pub async fn test_startpage_dispatch_1() {
        let tucan = TucanConnector::new().await.unwrap();
        startpage_dispatch_1(&tucan.client).await.unwrap();
    }

    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
    /// redirects to
    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome
    #[tokio::test]
    pub async fn test_welcome() {
        let tucan = TucanConnector::new().await.unwrap();
        welcome(&tucan.client).await.unwrap();
    }

    #[tokio::test]
    pub async fn module_1() {
        let tucan = TucanConnector::new().await.unwrap();
        let result = tucan
            .module_details(
                &LoginResponse {
                    id: 1,
                    cookie_cnsc: String::new(),
                },
                ModuleDetailsRequest {
                    arguments: ",-N000311,-N389455489906019".to_owned(),
                },
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    pub async fn course_1() {
        let tucan = TucanConnector::new().await.unwrap();
        let result = tucan
            .course_details(
                &LoginResponse {
                    id: 1,
                    cookie_cnsc: String::new(),
                },
                CourseDetailsRequest {
                    arguments: ",-N000311,-N0,-N389955196599934,-N389955196524935,-N0,-N0,-N3"
                        .to_owned(),
                },
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    pub async fn course_2() {
        let tucan = TucanConnector::new().await.unwrap();
        let result = tucan
            .course_details(
                &LoginResponse {
                    id: 1,
                    cookie_cnsc: String::new(),
                },
                CourseDetailsRequest {
                    arguments: ",-N000311,-N0,-N389955196291846,-N389955196210847,-N0,-N0,-N3"
                        .to_owned(),
                },
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    pub async fn course_3() {
        let tucan = TucanConnector::new().await.unwrap();
        let result = tucan
            .course_details(
                &LoginResponse {
                    id: 1,
                    cookie_cnsc: String::new(),
                },
                CourseDetailsRequest {
                    arguments: ",-N000311,-N0,-N389947398808423,-N389947398839424,-N0,-N0,-N3"
                        .to_owned(),
                },
            )
            .await
            .unwrap();
    }
}

#[cfg(all(test, not(feature = "authenticated_tests")))]
mod authenticated_tests {

    #[test]
    #[ignore = "feature authenticated_tests disabled"]
    pub const fn authenticated_tests() {}
}

#[cfg(all(test, feature = "authenticated_tests"))]
mod authenticated_tests {
    use tucant_types::{registration::AnmeldungRequest, LoginRequest, TucanError};

    use crate::{
        login::login, mlsstart::start_page::after_login, registration::index::anmeldung,
        startpage_dispatch::after_login::redirect_after_login, Tucan,
    };

    #[tokio::test]
    pub async fn test_login() {
        dotenvy::dotenv().unwrap();
        let tucan = Tucan::new().await.unwrap();
        assert!(matches!(
            login(
                &tucan.client,
                &LoginRequest {
                    username: std::env::var("USERNAME").expect("env variable USERNAME missing"),
                    password: std::env::var("PASSWORD").expect("env variable PASSWORD missing"),
                },
            )
            .await,
            Ok(_)
        ));
    }

    #[tokio::test]
    pub async fn test_redirect_after_login() {
        dotenvy::dotenv().unwrap();
        let tucan = Tucan::new().await.unwrap();
        let login_response = login(
            &tucan.client,
            &LoginRequest {
                username: std::env::var("USERNAME").expect("env variable USERNAME missing"),
                password: std::env::var("PASSWORD").expect("env variable PASSWORD missing"),
            },
        )
        .await
        .unwrap();
        redirect_after_login(&tucan.client, login_response)
            .await
            .unwrap()
    }

    #[tokio::test]
    pub async fn test_mlsstart() {
        dotenvy::dotenv().unwrap();
        let tucan = Tucan::new().await.unwrap();
        let login_response = login(
            &tucan.client,
            &LoginRequest {
                username: std::env::var("USERNAME").expect("env variable USERNAME missing"),
                password: std::env::var("PASSWORD").expect("env variable PASSWORD missing"),
            },
        )
        .await
        .unwrap();
        after_login(&tucan.client, &login_response).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_registration() {
        dotenvy::dotenv().unwrap();
        let tucan = Tucan::new().await.unwrap();
        let login_response = login(
            &tucan.client,
            &LoginRequest {
                username: std::env::var("USERNAME").expect("env variable USERNAME missing"),
                password: std::env::var("PASSWORD").expect("env variable PASSWORD missing"),
            },
        )
        .await
        .unwrap();
        let response = anmeldung(
            &tucan,
            &login_response,
            AnmeldungRequest {
                arguments: ",-N000311,-A".to_owned(),
            },
        )
        .await
        .unwrap();
    }
}
