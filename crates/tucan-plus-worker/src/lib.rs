use diesel::prelude::*;
use fragile::Fragile;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use web_sys::Worker;

use crate::{models::Anmeldung, schema::anmeldungen_plan};

pub mod models;
pub mod schema;

pub trait RequestResponse: Serialize {
    type Response: DeserializeOwned;
    fn execute(&self, connection: SqliteConnection) -> Self::Response;
}

#[derive(Serialize, Deserialize)]
pub struct AnmeldungenRequest {
    pub course_of_study: String,
}

impl RequestResponse for AnmeldungenRequest {
    type Response = Vec<Anmeldung>;

    fn execute(&self, mut connection: SqliteConnection) -> Self::Response {
        QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_plan::parent.is_null()),
        )
        .select(Anmeldung::as_select())
        .load(&mut connection)
        .expect("Error loading anmeldungen")
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnmeldungenRequest2 {
    pub course_of_study: String,
    pub anmeldung: Anmeldung,
}

impl RequestResponse for AnmeldungenRequest2 {
    type Response = Vec<Anmeldung>;

    fn execute(&self, mut connection: SqliteConnection) -> Self::Response {
        QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_plan::parent.eq(&self.anmeldung.url)),
        )
        .select(Anmeldung::as_select())
        .load(&mut connection)
        .expect("Error loading anmeldungen")
    }
}

#[derive(Deserialize)]
pub enum RequestResponseEnum {
    AnmeldungenRequest(AnmeldungenRequest),
}

impl RequestResponseEnum {
    pub fn execute(&self, mut connection: SqliteConnection) -> JsValue {
        match self {
            RequestResponseEnum::AnmeldungenRequest(anmeldungen_request) => {
                serde_wasm_bindgen::to_value(&anmeldungen_request.execute(connection)).unwrap()
            }
        }
    }
}
