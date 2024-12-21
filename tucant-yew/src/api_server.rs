use tucant_types::Tucan;

pub struct ApiServerTucan;

impl Tucan for ApiServerTucan {
    async fn login(
        request: tucant_types::LoginRequest,
    ) -> Result<tucant_types::LoginResponse, tucant_types::TucanError> {
        todo!()
    }

    async fn anmeldung(
        login_response: tucant_types::LoginResponse,
        request: tucant_types::registration::AnmeldungRequest,
    ) -> Result<tucant_types::registration::AnmeldungResponse, tucant_types::TucanError> {
        todo!()
    }
}
