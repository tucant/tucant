use serde_json::json;
use tucan_connector::{
    login::login, moduledetails::index::moduledetails, registration::index::anmeldung_cached,
};
use tucant_types::{
    moduledetails::{ModuleDetailsRequest, ModuleDetailsResponse},
    registration::{AnmeldungRequest, AnmeldungResponse},
    LoginRequest, LoginResponse, Tucan, TucanError,
};

pub struct DirectTucan;

impl Tucan for DirectTucan {
    async fn login(request: LoginRequest) -> Result<LoginResponse, TucanError> {
        let tucan = tucan_connector::Tucan::new().await?;
        login(&tucan.client, &request).await
    }

    async fn anmeldung(
        login_response: LoginResponse,
        request: AnmeldungRequest,
    ) -> Result<AnmeldungResponse, TucanError> {
        let tucan = tucan_connector::Tucan::new().await?;
        anmeldung_cached(&tucan, &login_response, request).await
    }

    async fn module_details(
        login_response: &LoginResponse,
        request: ModuleDetailsRequest,
    ) -> Result<ModuleDetailsResponse, TucanError> {
        let tucan = tucan_connector::Tucan::new().await?;
        moduledetails(&tucan, &login_response, request).await
    }
}
