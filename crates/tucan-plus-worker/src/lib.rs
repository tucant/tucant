use diesel::prelude::*;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::{models::Anmeldung, schema::anmeldungen_plan};

pub mod models;
pub mod schema;

pub trait RequestResponse {
    type Response;
    fn execute(&self) -> Self::Response;
}

#[derive(Deserialize)]
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
