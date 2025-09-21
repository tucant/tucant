use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::models::Anmeldung;

pub mod models;
pub mod schema;

pub trait RequestResponse: Serialize {
    type Response: DeserializeOwned;
    fn execute(&self) -> Self::Response;
}

#[derive(Serialize, Deserialize)]
pub struct AnmeldungenRequest {
    pub course_of_study: String,
}

impl RequestResponse for AnmeldungenRequest {
    type Response = Vec<Anmeldung>;

    fn execute(&self) -> Self::Response {
        todo!()
    }
}

#[derive(Deserialize)]
pub enum RequestResponseEnum {
    AnmeldungenRequest(AnmeldungenRequest),
}

impl RequestResponseEnum {
    pub fn execute(&self) -> JsValue {
        match self {
            RequestResponseEnum::AnmeldungenRequest(anmeldungen_request) => {
                serde_wasm_bindgen::to_value(&anmeldungen_request.execute()).unwrap()
            }
        }
    }
}
