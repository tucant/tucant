use tucant_types::{LoginRequest, LoginResponse, Tucan};

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
        request: tucant_types::registration::AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, tucant_types::TucanError> {
        todo!()
    }
}
