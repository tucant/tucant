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
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "server")]
use tucant_derive::Typescriptable;

// order needs to be equal to the table definition
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable))]
pub struct Module {
    #[serde(rename = "_id")]
    pub tucan_id: String,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Clone, Deserialize)]
pub struct ModuleMenuPathPart {
    #[serde(skip)]
    pub parent: Option<String>,
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(rename = "_id")]
    pub tucan_id: String,
    pub name: String,
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
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct ModuleMenu {
    #[serde(rename = "_id")]
    pub tucan_id: String,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub done: bool,
    pub parent: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ModuleMenuEntryModule {
    pub module_menu_id: String,
    pub module_id: String,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct Course {
    #[serde(rename = "_id")]
    pub tucan_id: String,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
    pub sws: i16,
    pub content: String,
    pub done: bool,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct CourseGroup {
    #[serde(rename = "_id")]
    pub tucan_id: String,
    pub course: String,
    pub title: String,
    pub done: bool,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ModuleCourse {
    pub module: String,
    pub course: String,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    #[serde(rename = "_id")]
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

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UndoneUser {
    #[serde(rename = "_id")]
    pub matriculation_number: i32,
    pub done: bool,
}

impl UndoneUser {
    pub fn new(matriculation_number: i32) -> Self {
        Self {
            matriculation_number,
            done: false,
        }
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable))]
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
            .map_err(|err| err.into_response())?;

        let session: TucanSession = serde_json::from_str(
            cookie_jar
                .get("session")
                .ok_or_else(|| "session not found".into_response())?
                .value(),
        )
        .map_err(|err| Into::<tucant::MyError>::into(err).into_response())?;
        Ok(session)
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UserModule {
    pub user_id: i32,
    pub module_id: String,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UserCourse {
    pub user_id: i32,
    pub course_id: String,
}
