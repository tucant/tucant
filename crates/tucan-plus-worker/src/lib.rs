use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::{models::Anmeldung, schema::anmeldungen_plan};

pub mod models;
pub mod schema;

pub trait RequestResponse {
    type Response;
    fn execute(&self) -> Self::Response;
}

pub enum RequestResponseTypes {}
