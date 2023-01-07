#![allow(clippy::wildcard_imports)] // inside diesel macro

use std::collections::VecDeque;

use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::extract::cookie::Key;
use axum_extra::extract::PrivateCookieJar;
// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::{
    AsChangeset, Associations, Identifiable, Insertable, Queryable, QueryableByName,
};
#[cfg(feature = "server")]
use diesel::sql_types::Bool;
#[cfg(feature = "server")]
use diesel::sql_types::Text;
#[cfg(feature = "server")]
use diesel::sql_types::{Bytea, Nullable};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "server")]
use tucant_derive::Typescriptable;

#[cfg(feature = "server")]
use crate::schema::{
    course_events, course_exams, course_groups_events, course_groups_unfinished,
    courses_unfinished, exams_unfinished, module_courses, module_exams, module_menu_module,
    module_menu_unfinished, modules_unfinished, sessions, user_course_groups, user_courses,
    user_exams, user_modules, users_unfinished,
};

pub fn as_base64<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode_engine(
        buffer.as_ref(),
        &base64::engine::fast_portable::FastPortable::from(
            &base64::alphabet::URL_SAFE,
            base64::engine::fast_portable::NO_PAD,
        ),
    ))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        base64::decode_engine(
            string,
            &base64::engine::fast_portable::FastPortable::from(
                &base64::alphabet::URL_SAFE,
                base64::engine::fast_portable::NO_PAD,
            ),
        )
        .map_err(|err| Error::custom(err.to_string()))
    })
}

pub fn as_option_base64<T, S>(buffer: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    if let Some(ref buffer) = *buffer {
        as_base64(buffer, serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn from_option_base64<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let string: Option<String> = Option::deserialize(deserializer)?;

    string.map_or_else(
        || Ok(None),
        |string| {
            base64::decode_engine(
                string,
                &base64::engine::fast_portable::FastPortable::from(
                    &base64::alphabet::URL_SAFE,
                    base64::engine::fast_portable::NO_PAD,
                ),
            )
            .map(Option::Some)
            .map_err(|err| Error::custom(err.to_string()))
        },
    )
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = modules_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct Module {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[cfg_attr(feature = "server", derive(QueryableByName, Typescriptable))]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Clone, Deserialize)]
pub struct ModuleMenuPathPart {
    #[cfg_attr(feature = "server", diesel(sql_type = Nullable<Bytea>))]
    #[serde(skip)]
    pub parent: Option<Vec<u8>>,
    #[cfg_attr(feature = "server", diesel(sql_type = Bytea))]
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    #[cfg_attr(feature = "server", diesel(sql_type = Text))]
    pub name: String,
    #[cfg_attr(feature = "server", diesel(sql_type = Bool))]
    #[serde(skip)]
    pub leaf: bool,
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub submenus: Vec<ModuleMenu>,
    pub modules_and_courses: Vec<(Module, Vec<Course>)>,
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ModuleMenuResponse {
    pub module_menu: ModuleMenu,
    pub entries: Registration,
    pub path: Vec<VecDeque<ModuleMenuPathPart>>,
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ModuleResponse {
    pub module: Module,
    pub path: Vec<VecDeque<ModuleMenuPathPart>>,
}

#[derive(Serialize, Debug, Eq, PartialEq, Deserialize, Clone)]
#[cfg_attr(
    feature = "server",
    derive(
        Identifiable,
        Queryable,
        Insertable,
        AsChangeset,
        QueryableByName,
        Typescriptable,
    )
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = false))]
pub struct ModuleMenu {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub done: bool,
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(
        serialize_with = "as_option_base64",
        deserialize_with = "from_option_base64"
    )]
    pub parent: Option<Vec<u8>>,
}

#[derive(Serialize, Debug)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(module_menu_id, module_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_module))]
#[cfg_attr(feature = "server", diesel(belongs_to(ModuleMenu)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Module)))]
pub struct ModuleMenuEntryModule {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(
        Identifiable,
        Queryable,
        Insertable,
        AsChangeset,
        Typescriptable,
        Associations
    )
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = courses_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[diesel(belongs_to(ModuleCourse, foreign_key = tucan_id))]
pub struct Course {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
    pub sws: i16,
    pub content: String,
    pub done: bool,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(
        Identifiable,
        Queryable,
        Insertable,
        AsChangeset,
        Typescriptable,
        Associations
    )
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = course_groups_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(Course, foreign_key = course)))]
pub struct CourseGroup {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub tucan_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub course: Vec<u8>,
    pub title: String,
    pub done: bool,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(module, course)))]
#[cfg_attr(feature = "server", diesel(table_name = module_courses))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(Module, foreign_key = module)))]
pub struct ModuleCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub course: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(matriculation_number)))]
#[cfg_attr(feature = "server", diesel(table_name = users_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct User {
    matriculation_number: i32,
    title: String,
    academic_title: String,
    post_name: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    pre_name: String,
    redirect_messages_to_university_email: bool,
    subject: String,
    email: String,
    department: i32,
    post_title: String,
    street: String,
    address_addition: String,
    country: String,
    plz: i32,
    city: String,
    phone_number: String,
    done: bool,
}

// TODO FIXME maybe we can convert this to a user enum with undone and done
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(matriculation_number)))]
#[cfg_attr(feature = "server", diesel(table_name = users_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct UndoneUser {
    pub matriculation_number: i32,
    pub done: bool,
}

impl UndoneUser {
    #[must_use]
    pub const fn new(matriculation_number: i32) -> Self {
        Self {
            matriculation_number,
            done: false,
        }
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable)
)]
#[cfg_attr(
    feature = "server",
    diesel(primary_key(matriculation_number, session_nr, session_id))
)]
#[cfg_attr(feature = "server", diesel(table_name = sessions))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct TucanSession {
    pub matriculation_number: i32,
    pub session_nr: i64,
    pub session_id: String,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for TucanSession
where
    Key: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = PrivateCookieJar::<Key>::from_request_parts(parts, state)
            .await
            .map_err(axum::response::IntoResponse::into_response)?;

        let session: Self = serde_json::from_str(
            cookie_jar
                .get("session")
                .ok_or_else(|| {
                    (axum::http::StatusCode::UNAUTHORIZED, "session not found").into_response()
                })?
                .value(),
        )
        .map_err(|err| Into::<tucant::MyError>::into(err).into_response())?;
        Ok(session)
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, module_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_modules))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(User, foreign_key = user_id)))]
#[cfg_attr(feature = "server", diesel(belongs_to(UndoneUser, foreign_key = user_id)))]
pub struct UserModule {
    pub user_id: i32,
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone, Typescriptable)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, course_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_courses))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(User, foreign_key = user_id)))]
#[cfg_attr(feature = "server", diesel(belongs_to(UndoneUser, foreign_key = user_id)))]
pub struct UserCourse {
    pub user_id: i32,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub course_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone, Typescriptable)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, course_group_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_course_groups))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(User, foreign_key = user_id)))]
#[cfg_attr(feature = "server", diesel(belongs_to(UndoneUser, foreign_key = user_id)))]
pub struct UserCourseGroup {
    pub user_id: i32,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub course_group_id: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = exams_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct Exam {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub tucan_id: Vec<u8>,
    pub exam_type: String,
    pub semester: String,
    pub exam_time_start: Option<NaiveDateTime>,
    pub exam_time_end: Option<NaiveDateTime>,
    pub registration_start: NaiveDateTime,
    pub registration_end: NaiveDateTime,
    pub unregistration_start: NaiveDateTime,
    pub unregistration_end: NaiveDateTime,
    pub examinator: Option<String>,
    pub room: Option<String>,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(course_id, exam)))]
#[cfg_attr(feature = "server", diesel(table_name = course_exams))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct CourseExam {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub course_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub exam: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(module_id, exam)))]
#[cfg_attr(feature = "server", diesel(table_name = module_exams))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct ModuleExam {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub module_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub exam: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(matriculation_number, exam)))]
#[cfg_attr(feature = "server", diesel(table_name = user_exams))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct UserExam {
    pub matriculation_number: i32,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub exam: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable, AsChangeset)
)]
#[cfg_attr(
    feature = "server",
    diesel(primary_key(course, timestamp_start, timestamp_end, room))
)]
#[cfg_attr(feature = "server", diesel(table_name = course_events))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct CourseEvent {
    pub course: Vec<u8>,
    pub timestamp_start: NaiveDateTime,
    pub timestamp_end: NaiveDateTime,
    pub room: String,
    pub teachers: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, Typescriptable, AsChangeset)
)]
#[cfg_attr(
    feature = "server",
    diesel(primary_key(course, timestamp_start, timestamp_end, room))
)]
#[cfg_attr(feature = "server", diesel(table_name = course_groups_events))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct CourseGroupEvent {
    pub course: Vec<u8>,
    pub timestamp_start: NaiveDateTime,
    pub timestamp_end: NaiveDateTime,
    pub room: String,
    pub teachers: String,
}

pub const MODULES_UNFINISHED: (
    modules_unfinished::columns::tucan_id,
    modules_unfinished::columns::tucan_last_checked,
    modules_unfinished::columns::title,
    modules_unfinished::columns::module_id,
    modules_unfinished::columns::credits,
    modules_unfinished::columns::content,
    modules_unfinished::columns::done,
) = (
    modules_unfinished::tucan_id,
    modules_unfinished::tucan_last_checked,
    modules_unfinished::title,
    modules_unfinished::module_id,
    modules_unfinished::credits,
    modules_unfinished::content,
    modules_unfinished::done,
);

pub const COURSES_UNFINISHED: (
    courses_unfinished::columns::tucan_id,
    courses_unfinished::columns::tucan_last_checked,
    courses_unfinished::columns::title,
    courses_unfinished::columns::course_id,
    courses_unfinished::columns::sws,
    courses_unfinished::columns::content,
    courses_unfinished::columns::done,
) = (
    courses_unfinished::tucan_id,
    courses_unfinished::tucan_last_checked,
    courses_unfinished::title,
    courses_unfinished::course_id,
    courses_unfinished::sws,
    courses_unfinished::content,
    courses_unfinished::done,
);
