use diesel::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::{models::Anmeldung, schema::anmeldungen_plan};

pub mod models;
pub mod schema;

pub trait RequestResponse: Serialize {
    type Response: DeserializeOwned;
    fn execute(request: Self) -> Self::Response;
}

#[derive(Serialize)]
pub struct FetchAnmeldungenRequest {
    pub course_of_study: String,
}

impl RequestResponse for FetchAnmeldungenRequest {
    type Response = Vec<Anmeldung>;

    fn execute(request: Self) -> Self::Response {
        let results: Vec<Anmeldung> = QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&request.course_of_study)
                .and(anmeldungen_plan::parent.is_null()),
        )
        .select(Anmeldung::as_select())
        .load(&mut *connection.borrow_mut())
        .expect("Error loading anmeldungen");
        results
    }
}
