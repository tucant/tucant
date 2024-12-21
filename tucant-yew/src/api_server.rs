use tucant_types::{
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse, Tucan,
};
use url::Url;

pub struct ApiServerTucan;

impl Tucan for ApiServerTucan {
    async fn login(
        request: tucant_types::LoginRequest,
    ) -> Result<tucant_types::LoginResponse, tucant_types::TucanError> {
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
        login_response: tucant_types::LoginResponse,
        request: AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, tucant_types::TucanError> {
        let client = reqwest::Client::new();
        let mut url = Url::parse("http://localhost:1420/api/v1/registration").unwrap();
        url.path_segments_mut().unwrap().push(&request.arguments);
        let response: AnmeldungResponse =
            client.get(url).send().await.unwrap().json().await.unwrap();
        Ok(response)
    }
}
