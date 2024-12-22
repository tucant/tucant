use tucant_types::{
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse, Tucan, TucanError,
};
use url::Url;

pub struct ApiServerTucan;

impl Tucan for ApiServerTucan {
    async fn login(request: LoginRequest) -> Result<LoginResponse, TucanError> {
        let client = reqwest::Client::new();

        let response: LoginResponse = client
            .post("http://localhost:1420/api/v1/login")
            .json(&request)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        Ok(response)
    }

    async fn anmeldung(
        login_response: LoginResponse,
        request: AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, TucanError> {
        let client = reqwest::Client::new();
        let mut url = Url::parse("http://localhost:1420/api/v1/registration").unwrap();
        url.path_segments_mut().unwrap().push(&request.arguments);
        let response: AnmeldungResponse =
            client.get(url).send().await.unwrap().json().await.unwrap();
        Ok(response)
    }

    async fn module_details(
        login_response: &LoginResponse,
        request: ModuleDetailsRequest,
    ) -> Result<ModuleDetailsResponse, TucanError> {
        let client = reqwest::Client::new();
        let mut url = Url::parse("http://localhost:1420/api/v1/module-details").unwrap();
        url.path_segments_mut().unwrap().push(&request.arguments);
        let response: ModuleDetailsResponse =
            client.get(url).send().await.unwrap().json().await.unwrap();
        Ok(response)
    }
}
