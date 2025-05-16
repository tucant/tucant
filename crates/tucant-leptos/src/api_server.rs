use reqwest::Client;
use tucant_types::{
    LoginRequest, LoginResponse, RevalidationStrategy, SemesterId, Tucan, TucanError,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    courseresults::ModuleResultsResponse,
    examresults::ExamResultsResponse,
    mlsstart::MlsStart,
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    mycourses::MyCoursesResponse,
    mydocuments::MyDocumentsResponse,
    myexams::MyExamsResponse,
    mymodules::MyModulesResponse,
    registration::AnmeldungRequest,
    student_result::StudentResultResponse,
    vv::{ActionRequest, Vorlesungsverzeichnis},
};
use url::Url;

pub struct ApiServerTucan {
    client: Client,
}

impl Default for ApiServerTucan {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiServerTucan {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder().user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de").build().unwrap(),
        }
    }
}

impl Tucan for ApiServerTucan {
    async fn login(&self, request: LoginRequest) -> Result<LoginResponse, TucanError> {
        let response = self.client.post("http://localhost:1420/api/v1/login").json(&request).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn anmeldung(&self, _login_response: LoginResponse, revalidation_strategy: RevalidationStrategy, request: AnmeldungRequest) -> Result<tucant_types::registration::AnmeldungResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/registration").unwrap();
        url.path_segments_mut().unwrap().push(request.inner());
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn module_details(&self, _login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/module-details").unwrap();
        url.path_segments_mut().unwrap().push(request.inner());
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn course_details(&self, _login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, request: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/course-details").unwrap();
        url.path_segments_mut().unwrap().push(request.inner());
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn logout(&self, _request: &LoginResponse) -> Result<(), TucanError> {
        self.client.post("http://localhost:1420/api/v1/logout").send().await?.error_for_status()?;

        Ok(())
    }

    async fn after_login(&self, _request: &LoginResponse, revalidation_strategy: RevalidationStrategy) -> Result<MlsStart, TucanError> {
        let url = Url::parse("http://localhost:1420/api/v1/after-login").unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn my_modules(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyModulesResponse, TucanError> {
        let url = Url::parse(&format!("http://localhost:1420/api/v1/my-modules/{}", semester.inner())).unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn my_courses(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyCoursesResponse, TucanError> {
        let url = Url::parse(&format!("http://localhost:1420/api/v1/my-courses/{}", semester.inner())).unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn my_exams(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<MyExamsResponse, TucanError> {
        let url = Url::parse(&format!("http://localhost:1420/api/v1/my-exams/{}", semester.inner())).unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn exam_results(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<ExamResultsResponse, TucanError> {
        let url = Url::parse(&format!("http://localhost:1420/api/v1/exam-results/{}", semester.inner())).unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn course_results(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy, semester: SemesterId) -> Result<ModuleResultsResponse, TucanError> {
        let url = Url::parse(&format!("http://localhost:1420/api/v1/course-results/{}", semester.inner())).unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn my_documents(&self, _request: &tucant_types::LoginResponse, revalidation_strategy: RevalidationStrategy) -> Result<MyDocumentsResponse, TucanError> {
        let url = Url::parse("http://localhost:1420/api/v1/my-documents").unwrap();
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn vv(&self, _login_response: Option<&LoginResponse>, revalidation_strategy: RevalidationStrategy, action: ActionRequest) -> Result<Vorlesungsverzeichnis, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/vv").unwrap();
        url.path_segments_mut().unwrap().push(action.inner());
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn student_result(&self, _login_response: &LoginResponse, revalidation_strategy: RevalidationStrategy, course_of_study: u64) -> Result<StudentResultResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/student-result").unwrap();
        url.path_segments_mut().unwrap().push(&course_of_study.to_string());
        let response = self.client.get(url).header("X-Revalidation-Strategy", serde_json::to_string(&revalidation_strategy).unwrap()).send().await?.error_for_status()?.json().await?;
        Ok(response)
    }

    async fn welcome(&self) -> Result<tucant_types::LoggedOutHead, TucanError> {
        todo!()
    }
}
