use derive_more::From;
use diesel::{prelude::*, upsert::excluded};
use fragile::Fragile;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;
use web_sys::Worker;

use crate::{
    models::{Anmeldung, AnmeldungEntry},
    schema::{anmeldungen_entries, anmeldungen_plan},
};

pub mod models;
pub mod schema;
pub trait RequestResponse: Serialize {
    type Response: DeserializeOwned;
    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnmeldungenRequest {
    pub course_of_study: String,
}

impl RequestResponse for AnmeldungenRequest {
    type Response = Vec<Anmeldung>;

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_plan::parent.is_null()),
        )
        .select(Anmeldung::as_select())
        .load(connection)
        .expect("Error loading anmeldungen")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnmeldungenRequest2 {
    pub course_of_study: String,
    pub anmeldung: Anmeldung,
}

impl RequestResponse for AnmeldungenRequest2 {
    type Response = Vec<Anmeldung>;

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_plan::parent.eq(&self.anmeldung.url)),
        )
        .select(Anmeldung::as_select())
        .load(connection)
        .expect("Error loading anmeldungen")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fewe {
    pub course_of_study: String,
    pub anmeldung: Anmeldung,
}

impl RequestResponse for Fewe {
    type Response = Vec<AnmeldungEntry>;

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        QueryDsl::filter(
            anmeldungen_entries::table,
            anmeldungen_entries::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_entries::anmeldung.eq(&self.anmeldung.url)),
        )
        .select(AnmeldungEntry::as_select())
        .load(connection)
        .expect("Error loading anmeldungen")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FEwefweewf {
    inserts: Vec<Anmeldung>,
}

impl RequestResponse for FEwefweewf {
    type Response = ();

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::insert_into(anmeldungen_plan::table)
            .values(&self.inserts)
            .on_conflict((anmeldungen_plan::course_of_study, anmeldungen_plan::url))
            .do_update()
            .set(anmeldungen_plan::parent.eq(excluded(anmeldungen_plan::parent)))
            .execute(connection)
            .expect("Error saving anmeldungen");
    }
}

#[derive(Serialize, Deserialize, Debug, From)]
pub enum RequestResponseEnum {
    AnmeldungenRequest(AnmeldungenRequest),
    AnmeldungenRequest2(AnmeldungenRequest2),
    Fewe(Fewe),
}

impl RequestResponseEnum {
    pub fn execute(&self, connection: &mut SqliteConnection) -> JsValue {
        match self {
            RequestResponseEnum::AnmeldungenRequest(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::AnmeldungenRequest2(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::Fewe(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
        }
    }
}
