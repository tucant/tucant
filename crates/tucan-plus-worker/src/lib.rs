use crate::models::Anmeldung;

pub mod models;
pub mod schema;

pub trait RequestResponse {
    type Response: DeserializeOwned;
    fn execute(request: Self) -> Self::Response;
}

pub struct FetchAnmeldungenRequest {
    pub course_of_study: String,
}

impl RequestResponse for FetchAnmeldungenRequest {
    type Response = Vec<Anmeldung>;

    fn execute(request: Self) -> Self::Response {
        todo!()
    }
}
