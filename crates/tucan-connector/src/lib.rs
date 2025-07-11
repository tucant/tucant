use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use coursedetails::course_details;
use courseresults::courseresults;
use examresults::examresults;
use externalpages::welcome::welcome;
use key_value_database::Database;
use login::{login, logout};
use mlsstart::after_login;
use moduledetails::module_details;
use mycourses::mycourses;
use mydocuments::my_documents;
use myexams::my_exams;
use mymodules::mymodules;
use regex::Regex;
use registration::anmeldung;
use reqwest::header;
use student_result::student_result;
use time::{OffsetDateTime, format_description::well_known::Rfc2822};
use tokio::sync::Semaphore;
use tucant_types::{
    LoginResponse, RevalidationStrategy, SemesterId, Tucan, TucanError,
    courseresults::ModuleResultsResponse,
    examresults::ExamResultsResponse,
    gradeoverview::{GradeOverviewRequest, GradeOverviewResponse},
    mlsstart::MlsStart,
    mycourses::MyCoursesResponse,
    mydocuments::MyDocumentsResponse,
    myexams::MyExamsResponse,
    mymodules::MyModulesResponse,
    student_result::StudentResultResponse,
    vv::{ActionRequest, Vorlesungsverzeichnis},
};
use vv::vv;

pub mod coursedetails;
pub mod courseprep;
pub mod courseresults;
pub mod examresults;
pub mod externalpages;
pub mod gradeoverview;
pub mod head;
pub mod login;
pub mod mlsstart;
pub mod moduledetails;
pub mod month;
pub mod mycourses;
pub mod mydocuments;
pub mod myexams;
pub mod mymodules;
pub mod registration;
pub mod root;
pub mod startpage_dispatch;
pub mod student_result;
pub mod vv;

#[cfg(target_arch = "wasm32")]
pub async fn sleep(duration: Duration) {
    let mut cb = |resolve: js_sys::Function, reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                duration.as_millis().try_into().unwrap(),
            );
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

use crate::gradeoverview::gradeoverview;

static COURSEDETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        "^/scripts/mgrqispi.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,",
    )
    .unwrap()
});

type MyClient = reqwest::Client;

#[cfg_attr(not(target_arch = "wasm32"), derive(Clone))]
pub struct TucanConnector {
    pub client: MyClient,
    pub database: Database,
    semaphore: Arc<Semaphore>,
}

/// `TUCaN` being unreliable is a feature
pub async fn retryable_get(
    connector: &TucanConnector,
    url: &str,
) -> Result<(String, OffsetDateTime), TucanError> {
    let mut i = 0;
    loop {
        let permit = connector.semaphore.acquire().await.unwrap();
        let result = async {
            let response = connector.client.get(url).send().await?.error_for_status()?;
            let date = &response.headers()["Date"];
            let date = OffsetDateTime::parse(date.to_str().unwrap(), &Rfc2822).unwrap();
            Ok((response.text().await?, date))
        }
        .await;
        drop(permit);
        if i == 4 {
            return result;
        }
        match result {
            Ok((value, date)) => return Ok((value, date)),
            Err(err) => eprintln!("ignoring error: {err:?}"),
        }
        sleep(Duration::from_secs(2u64.pow(i))).await;
        i += 1;
    }
}
pub async fn authenticated_retryable_get(
    connector: &TucanConnector,
    url: &str,
    cookie_cnsc: &str,
) -> Result<(String, OffsetDateTime), TucanError> {
    let mut i = 0;
    loop {
        let permit = connector.semaphore.acquire().await.unwrap();
        let result = async {
            let response = connector
                .client
                .get(url)
                .header("Cookie", format!("cnsc={cookie_cnsc}"))
                .send()
                .await?
                .error_for_status()?;
            let date = &response.headers()["Date"];
            let date = OffsetDateTime::parse(date.to_str().unwrap(), &Rfc2822).unwrap();
            Ok((response.text().await?, date))
        }
        .await;
        drop(permit);
        if i == 4 {
            return result;
        }
        match result {
            Ok((value, date)) => return Ok((value, date)),
            Err(err) => eprintln!("ignoring error: {err:?}"),
        }
        sleep(Duration::from_secs(2u64.pow(i))).await;
        i += 1;
    }
}

impl TucanConnector {
    pub async fn new() -> Result<Self, TucanError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept-Language",
            header::HeaderValue::from_static("de-DE,de;q=0.5"),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de")
            .build()
            .unwrap();
        Ok(Self {
            client,
            database: Database::new().await,
            semaphore: Arc::new(Semaphore::new(10)),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn new_test(
        client: reqwest::Client,
        semaphore: Arc<Semaphore>,
    ) -> Result<Self, TucanError> {
        Ok(Self {
            client,
            database: Database::new_test().await,
            semaphore,
        })
    }
}

impl Tucan for TucanConnector {
    async fn login(
        &self,
        request: tucant_types::LoginRequest,
    ) -> Result<tucant_types::LoginResponse, TucanError> {
        login(&self.client, &request).await
    }

    async fn welcome(&self) -> Result<tucant_types::LoggedOutHead, TucanError> {
        welcome(self).await
    }

    async fn after_login(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
    ) -> Result<MlsStart, TucanError> {
        after_login(self, request, revalidation_strategy).await
    }

    async fn logout(&self, request: &tucant_types::LoginResponse) -> Result<(), TucanError> {
        logout(self, request).await
    }

    async fn my_modules(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        semester: SemesterId,
    ) -> Result<MyModulesResponse, TucanError> {
        mymodules(self, request, revalidation_strategy, semester).await
    }

    async fn my_courses(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        semester: SemesterId,
    ) -> Result<MyCoursesResponse, TucanError> {
        mycourses(self, request, revalidation_strategy, semester).await
    }

    async fn my_exams(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        semester: SemesterId,
    ) -> Result<MyExamsResponse, TucanError> {
        my_exams(self, request, revalidation_strategy, semester).await
    }

    async fn exam_results(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        semester: SemesterId,
    ) -> Result<ExamResultsResponse, TucanError> {
        examresults(self, request, revalidation_strategy, semester).await
    }

    async fn course_results(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        semester: SemesterId,
    ) -> Result<ModuleResultsResponse, TucanError> {
        courseresults(self, request, revalidation_strategy, semester).await
    }

    async fn my_documents(
        &self,
        request: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
    ) -> Result<MyDocumentsResponse, TucanError> {
        my_documents(self, request, revalidation_strategy).await
    }

    async fn anmeldung(
        &self,
        login_response: tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        request: tucant_types::registration::AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, TucanError> {
        anmeldung(self, &login_response, revalidation_strategy, request).await
    }

    async fn module_details(
        &self,
        login_response: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        request: tucant_types::moduledetails::ModuleDetailsRequest,
    ) -> Result<tucant_types::moduledetails::ModuleDetailsResponse, TucanError> {
        module_details(self, login_response, revalidation_strategy, request).await
    }

    async fn course_details(
        &self,
        login_response: &tucant_types::LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        request: tucant_types::coursedetails::CourseDetailsRequest,
    ) -> Result<tucant_types::coursedetails::CourseDetailsResponse, TucanError> {
        course_details(self, login_response, revalidation_strategy, request).await
    }

    async fn vv(
        &self,
        login_response: Option<&tucant_types::LoginResponse>,
        revalidation_strategy: RevalidationStrategy,
        action: ActionRequest,
    ) -> Result<Vorlesungsverzeichnis, TucanError> {
        vv(self, login_response, revalidation_strategy, action).await
    }

    async fn student_result(
        &self,
        login_response: &LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        course_of_study: u64,
    ) -> Result<StudentResultResponse, TucanError> {
        student_result(self, login_response, revalidation_strategy, course_of_study).await
    }

    fn gradeoverview(
        &self,
        login_response: &LoginResponse,
        revalidation_strategy: RevalidationStrategy,
        request: GradeOverviewRequest,
    ) -> impl std::future::Future<Output = Result<GradeOverviewResponse, TucanError>> {
        gradeoverview(self, login_response, revalidation_strategy, request)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, OnceLock};

    use reqwest::{Client, header};
    use tokio::{
        runtime::Runtime,
        sync::{OnceCell, Semaphore},
    };
    use tucant_types::{
        LoginRequest, LoginResponse, RevalidationStrategy, TucanError,
        coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest,
    };

    use crate::{
        Tucan, TucanConnector, externalpages::welcome::welcome, login::login, root::root,
        startpage_dispatch::one::startpage_dispatch_1,
    };

    static ONCE_CONNECTOR: OnceCell<(Client, Arc<Semaphore>)> = OnceCell::const_new();

    pub fn runtime() -> &'static Runtime {
        static RUNTIME: OnceLock<Runtime> = OnceLock::new();
        RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
        })
    }

    pub async fn get_tucan_connector() -> TucanConnector {
        let (client, semaphore) = ONCE_CONNECTOR
            .get_or_init(|| async {
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    "Accept-Language",
                    header::HeaderValue::from_static("de-DE,de;q=0.5"),
                );
                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .user_agent(
                        "https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de",
                    )
                    .build()
                    .unwrap();

                let semaphore = Arc::new(Semaphore::new(10));
                (client, semaphore)
            })
            .await;
        TucanConnector::new_test(client.clone(), semaphore.clone())
            .await
            .unwrap()
    }

    #[test]
    pub fn login_incorrect() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            assert!(matches!(
                login(
                    &tucan.client,
                    &LoginRequest {
                        username: "not_found".to_owned(),
                        password: "not_correct".to_owned()
                    },
                )
                .await,
                Err(TucanError::InvalidCredentials)
            ));
        });
    }

    #[test]
    pub fn test_root_page() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            root(&tucan).await.unwrap();
        });
    }

    /// /
    /// redirects to
    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
    #[test]
    pub fn test_startpage_dispatch_1() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            startpage_dispatch_1(&tucan).await.unwrap();
        });
    }

    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
    /// redirects to
    /// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome
    #[test]
    pub fn test_welcome() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            welcome(&tucan).await.unwrap();
        });
    }

    #[test]
    pub fn module_keine_leistungskombination() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .module_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    ModuleDetailsRequest::parse("-N383723477792938"),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn module_leistungskombination() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .module_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    ModuleDetailsRequest::parse("-N374884241922478"),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_1() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N389955196599934,-N389955196524935,-N0,-N0,-N3",
                    ),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_2() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N389955196291846,-N389955196210847,-N0,-N0,-N3",
                    ),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_3() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N389947398808423,-N389947398839424,-N0,-N0,-N3",
                    ),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_4() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N389043269698095,-N389043269646096,-N0,-N0,-N3",
                    ),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_5() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N392125895008100,-N392125895040101,-N0,-N0,-N3",
                    ),
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn course_6() {
        runtime().block_on(async {
            let tucan = get_tucan_connector().await;
            let _result = tucan
                .course_details(
                    &LoginResponse {
                        id: 1,
                        cookie_cnsc: String::new(),
                    },
                    RevalidationStrategy::default(),
                    CourseDetailsRequest::parse(
                        "-N0,-N391415618587221,-N391415618615224,-N0,-N0,-N0",
                    ),
                )
                .await
                .unwrap();
        });
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
    use tokio::sync::OnceCell;
    use tucant_types::{
        LoginRequest, LoginResponse, RevalidationStrategy, SemesterId,
        registration::AnmeldungRequest,
    };

    use crate::{
        Tucan,
        courseresults::courseresults,
        examresults::examresults,
        gradeoverview::gradeoverview,
        login::login,
        mlsstart::after_login,
        mycourses::mycourses,
        mydocuments::my_documents,
        myexams::my_exams,
        registration::anmeldung,
        startpage_dispatch::after_login::redirect_after_login,
        student_result::student_result,
        tests::{get_tucan_connector, runtime},
    };

    static ONCE: OnceCell<LoginResponse> = OnceCell::const_new();

    async fn get_login_session() -> &'static LoginResponse {
        ONCE.get_or_init(|| async {
            login(
                &get_tucan_connector().await.client,
                &LoginRequest {
                    username: std::env::var("TUCAN_USERNAME")
                        .expect("env variable TUCAN_USERNAME missing"),
                    password: std::env::var("TUCAN_PASSWORD")
                        .expect("env variable TUCAN_PASSWORD missing"),
                },
            )
            .await
            .unwrap()
        })
        .await
    }

    #[test]
    pub fn test_login() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            get_login_session().await;
        });
    }

    #[test]
    pub fn test_redirect_after_login() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            redirect_after_login(&tucan, login_response.clone())
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn test_mlsstart() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            after_login(&tucan, login_response, RevalidationStrategy::default())
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn test_registration() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let _response = anmeldung(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                AnmeldungRequest::default(),
            )
            .await
            .unwrap();
        });
    }

    #[test]
    pub fn vv_top_level() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let action = tucan
                .after_login(login_response, RevalidationStrategy::default())
                .await
                .unwrap()
                .logged_in_head
                .vorlesungsverzeichnis_url;
            let _result = tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    action,
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn vv_first_level() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let action = tucan
                .after_login(login_response, RevalidationStrategy::default())
                .await
                .unwrap()
                .logged_in_head
                .vorlesungsverzeichnis_url;
            let result = tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    action,
                )
                .await
                .unwrap()
                .entries[0]
                .clone()
                .1;
            let _result = tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    result,
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn vv_first_level_4_courses() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let action = tucan
                .after_login(login_response, RevalidationStrategy::default())
                .await
                .unwrap()
                .logged_in_head
                .vorlesungsverzeichnis_url;
            let result = tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    action,
                )
                .await
                .unwrap()
                .entries[4]
                .clone()
                .1;
            let _result = tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    result,
                )
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn vv_first_level_all() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let action = tucan
                .after_login(login_response, RevalidationStrategy::default())
                .await
                .unwrap()
                .logged_in_head
                .vorlesungsverzeichnis_url;
            for (_title, action) in tucan
                .vv(
                    Some(login_response),
                    RevalidationStrategy::default(),
                    action,
                )
                .await
                .unwrap()
                .entries
            {
                let _result = tucan
                    .vv(
                        Some(login_response),
                        RevalidationStrategy::default(),
                        action,
                    )
                    .await
                    .unwrap();
            }
        });
    }

    #[test]
    pub fn test_mymodules() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            tucan
                .my_modules(
                    login_response,
                    RevalidationStrategy::default(),
                    SemesterId::all(),
                )
                .await
                .unwrap();
            let semesters = tucan
                .my_modules(
                    login_response,
                    RevalidationStrategy::default(),
                    SemesterId::current(),
                )
                .await
                .unwrap()
                .semester;
            for semester in semesters {
                tucan
                    .my_modules(
                        login_response,
                        RevalidationStrategy::default(),
                        semester.value,
                    )
                    .await
                    .unwrap();
            }
        });
    }

    #[test]
    pub fn test_mycourses() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            mycourses(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::all(),
            )
            .await
            .unwrap();
            let semesters = mycourses(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::current(),
            )
            .await
            .unwrap()
            .semester;
            for semester in semesters {
                mycourses(
                    &tucan,
                    login_response,
                    RevalidationStrategy::default(),
                    semester.value,
                )
                .await
                .unwrap();
            }
        });
    }

    #[test]
    pub fn test_myexams() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            my_exams(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::all(),
            )
            .await
            .unwrap();
            let semesters = my_exams(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::current(),
            )
            .await
            .unwrap()
            .semester;
            for semester in semesters {
                my_exams(
                    &tucan,
                    login_response,
                    RevalidationStrategy::default(),
                    semester.value,
                )
                .await
                .unwrap();
            }
        });
    }

    #[test]
    pub fn test_courseresults() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let semesters = courseresults(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::current(),
            )
            .await
            .unwrap()
            .semester;
            for semester in semesters {
                let courseresults = courseresults(
                    &tucan,
                    login_response,
                    RevalidationStrategy::default(),
                    semester.value,
                )
                .await
                .unwrap();
                for result in courseresults.results {
                    if let Some(average_url) = result.average_url {
                        println!("{average_url}");
                        let overview = gradeoverview(
                            &tucan,
                            login_response,
                            RevalidationStrategy::cache(),
                            average_url,
                        )
                        .await
                        .unwrap();
                        println!("{overview:?}")
                    }
                }
            }
        });
    }

    #[test]
    pub fn test_examresults() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let result = examresults(
                &tucan,
                login_response,
                RevalidationStrategy::cache(),
                SemesterId::all(),
            )
            .await
            .unwrap();
            for result in result.results {
                if let Some(average_url) = result.average_url {
                    println!("{average_url}");
                    let overview = gradeoverview(
                        &tucan,
                        login_response,
                        RevalidationStrategy::cache(),
                        average_url,
                    )
                    .await
                    .unwrap();
                    println!("{overview:?}")
                }
            }
            let semesters = examresults(
                &tucan,
                login_response,
                RevalidationStrategy::default(),
                SemesterId::current(),
            )
            .await
            .unwrap()
            .semester;
            for semester in semesters {
                examresults(
                    &tucan,
                    login_response,
                    RevalidationStrategy::default(),
                    semester.value,
                )
                .await
                .unwrap();
            }
        });
    }

    #[test]
    pub fn test_mydocuments() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            my_documents(&tucan, login_response, RevalidationStrategy::default())
                .await
                .unwrap();
        });
    }

    #[test]
    pub fn test_student_result() {
        runtime().block_on(async {
            dotenvy::dotenv().unwrap();
            let tucan = get_tucan_connector().await;
            let login_response = get_login_session().await;
            let response =
                student_result(&tucan, login_response, RevalidationStrategy::default(), 0)
                    .await
                    .unwrap();
            for course_of_study in response.course_of_study {
                let response = student_result(
                    &tucan,
                    login_response,
                    RevalidationStrategy::default(),
                    course_of_study.value,
                )
                .await
                .unwrap();
                println!("{response:#?}");
            }
        });
    }
}
