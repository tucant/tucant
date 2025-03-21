use reqwest::Client;
use tucant_types::{
    LoggedInHead, LoginRequest, LoginResponse, Tucan, TucanError, Vorlesungsverzeichnis,
    coursedetails::{CourseDetailsRequest, CourseDetailsResponse},
    mlsstart::MlsStart,
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse},
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
            client: reqwest::ClientBuilder::new().user_agent("https://github.com/tucant/tucant d8167c8 Moritz.Hedtke@t-online.de").build().unwrap(),
        }
    }
}

impl Tucan for ApiServerTucan {
    async fn login(&self, request: LoginRequest) -> Result<LoginResponse, TucanError> {
        let response: LoginResponse = self.client.post("http://localhost:1420/api/v1/login").json(&request).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }

    async fn anmeldung(&self, _login_response: LoginResponse, request: AnmeldungRequest) -> Result<tucant_types::registration::AnmeldungResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/registration").unwrap();
        url.path_segments_mut().unwrap().push(&request.inner());
        let response: AnmeldungResponse = self.client.get(url).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }

    async fn module_details(&self, _login_response: &LoginResponse, request: ModuleDetailsRequest) -> Result<ModuleDetailsResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/module-details").unwrap();
        url.path_segments_mut().unwrap().push(&request.inner());
        let response: ModuleDetailsResponse = self.client.get(url).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }

    async fn course_details(&self, _login_response: &LoginResponse, request: CourseDetailsRequest) -> Result<CourseDetailsResponse, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/course-details").unwrap();
        url.path_segments_mut().unwrap().push(request.inner());
        let response: CourseDetailsResponse = self.client.get(url).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }

    async fn logout(&self, _request: &LoginResponse) -> Result<(), TucanError> {
        self.client.post("http://localhost:1420/api/v1/logout").send().await.unwrap().error_for_status().unwrap();

        Ok(())
    }

    async fn after_login(&self, _request: &LoginResponse) -> Result<MlsStart, TucanError> {
        let url = Url::parse("http://localhost:1420/api/v1/after-login").unwrap();
        let response: MlsStart = self.client.get(url).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }

    async fn vv(&self, _login_response: &LoginResponse, action: String) -> Result<Vorlesungsverzeichnis, TucanError> {
        let mut url = Url::parse("http://localhost:1420/api/v1/vv").unwrap();
        url.path_segments_mut().unwrap().push(&action);
        let response: Vorlesungsverzeichnis = self.client.get(url).send().await.unwrap().error_for_status()?.json().await.unwrap();
        Ok(response)
    }
}
