use derive_more::From;
use diesel::{prelude::*, upsert::excluded};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::{
    models::{Anmeldung, AnmeldungEntry, Semester, State},
    schema::{anmeldungen_entries, anmeldungen_plan},
};
use tucan_types::{
    Semesterauswahl,
    courseresults::ModuleResult,
    student_result::{StudentResultLevel, StudentResultResponse},
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
    pub inserts: Vec<Anmeldung>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Wlewifhewefwef {
    pub insert: AnmeldungEntry,
}

impl RequestResponse for Wlewifhewefwef {
    type Response = ();

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::insert_into(anmeldungen_entries::table)
            .values(&self.insert)
            .on_conflict((
                anmeldungen_entries::course_of_study,
                anmeldungen_entries::anmeldung,
                anmeldungen_entries::available_semester,
                anmeldungen_entries::id,
            ))
            .do_update()
            .set((
                // TODO FIXME I think updating does not work
                anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)),
                (anmeldungen_entries::credits.eq(excluded(anmeldungen_entries::credits))),
            ))
            .execute(connection)
            .expect("Error saving anmeldungen");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildUrl {
    pub course_of_study: String,
    pub url: String,
    pub name: String,
    pub child: StudentResultLevel,
}

impl RequestResponse for ChildUrl {
    type Response = String;

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::update(QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(
                    anmeldungen_plan::parent
                        .eq(&self.url)
                        .and(anmeldungen_plan::name.eq(&self.name)),
                ),
        ))
        .set((
            anmeldungen_plan::min_cp.eq(self.child.rules.min_cp as i32),
            anmeldungen_plan::max_cp.eq(self.child.rules.max_cp.map(|v| v as i32)),
            anmeldungen_plan::min_modules.eq(self.child.rules.min_modules as i32),
            anmeldungen_plan::max_modules.eq(self.child.rules.max_modules.map(|v| v as i32)),
        ))
        .returning(anmeldungen_plan::url)
        .get_result(connection)
        .expect("Error updating anmeldungen")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateModule {
    pub course_of_study: String,
    pub semester: Semesterauswahl,
    pub module: ModuleResult,
}

impl RequestResponse for UpdateModule {
    type Response = ();

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::update(anmeldungen_entries::table)
            .filter(
                anmeldungen_entries::course_of_study
                    .eq(&self.course_of_study)
                    .and(
                        anmeldungen_entries::id
                            .eq(&self.module.nr)
                            // TODO FIXME if you can register it at multiple paths
                            // this will otherwise break
                            .and(anmeldungen_entries::state.ne(State::NotPlanned)),
                    ),
            )
            .set((
                anmeldungen_entries::semester.eq(if self.semester.name.starts_with("SoSe ") {
                    Semester::Sommersemester
                } else {
                    Semester::Wintersemester
                }),
                (anmeldungen_entries::year.eq(self.semester.name[5..9].parse::<i32>().unwrap())),
            ))
            .execute(connection)
            .expect("Error updating anmeldungen");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetStateAndCredits {
    pub inserts: Vec<AnmeldungEntry>,
}

impl RequestResponse for SetStateAndCredits {
    type Response = ();

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::insert_into(anmeldungen_entries::table)
            .values(&self.inserts)
            .on_conflict((
                anmeldungen_entries::course_of_study,
                anmeldungen_entries::anmeldung,
                anmeldungen_entries::available_semester,
                anmeldungen_entries::id,
            ))
            .do_update()
            .set((
                anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)),
                (anmeldungen_entries::credits.eq(excluded(anmeldungen_entries::credits))),
            ))
            .execute(connection)
            .expect("Error saving anmeldungen");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetCpAndModuleCount {
    pub course_of_study: String,
    pub name: String,
    pub student_result: StudentResultResponse,
}

impl RequestResponse for SetCpAndModuleCount {
    type Response = String;

    fn execute(&self, connection: &mut SqliteConnection) -> Self::Response {
        diesel::update(QueryDsl::filter(
            anmeldungen_plan::table,
            anmeldungen_plan::course_of_study
                .eq(&self.course_of_study)
                .and(anmeldungen_plan::name.eq(&self.name)),
        ))
        .set((
            anmeldungen_plan::min_cp.eq(self.student_result.level0.rules.min_cp as i32),
            anmeldungen_plan::max_cp.eq(self.student_result.level0.rules.max_cp.map(|v| v as i32)),
            anmeldungen_plan::min_modules.eq(self.student_result.level0.rules.min_modules as i32),
            anmeldungen_plan::max_modules.eq(self
                .student_result
                .level0
                .rules
                .max_modules
                .map(|v| v as i32)),
        ))
        .returning(anmeldungen_plan::url)
        .get_result(connection)
        .expect("Error updating anmeldungen")
    }
}

#[derive(Serialize, Deserialize, Debug, From)]
pub enum RequestResponseEnum {
    AnmeldungenRequest(AnmeldungenRequest),
    AnmeldungenRequest2(AnmeldungenRequest2),
    Fewe(Fewe),
    FEwefweewf(FEwefweewf),
    Wlewifhewefwef(Wlewifhewefwef),
    ChildUrl(ChildUrl),
    UpdateModule(UpdateModule),
    SetStateAndCredits(SetStateAndCredits),
    SetCpAndModuleCount(SetCpAndModuleCount),
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
            RequestResponseEnum::FEwefweewf(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::Wlewifhewefwef(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::ChildUrl(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::UpdateModule(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::SetStateAndCredits(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
            RequestResponseEnum::SetCpAndModuleCount(value) => {
                serde_wasm_bindgen::to_value(&value.execute(connection)).unwrap()
            }
        }
    }
}
