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

pub fn as_base64<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode_config(
        buffer.as_ref(),
        base64::URL_SAFE_NO_PAD,
    ))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        base64::decode_config(string, base64::URL_SAFE_NO_PAD)
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

    if let Some(string) = string {
        base64::decode_config(string, base64::URL_SAFE_NO_PAD)
            .map(Option::Some)
            .map_err(|err| Error::custom(err.to_string()))
    } else {
        Ok(None)
    }
}

// order needs to be equal to the table definition
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Typescriptable)
)]
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

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Clone, Deserialize)]
pub struct ModuleMenuPathPart {
    #[serde(skip)]
    pub parent: Option<Vec<u8>>,
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
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
#[cfg_attr(
    feature = "server",
    derive(
        Typescriptable,
    )
)]
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

#[cfg_attr(feature = "server", derive(Debug))]
pub struct ModuleMenuChangeset {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub done: bool,
    pub parent: Option<Option<Vec<u8>>>,
}

#[derive(Serialize, Debug)]
pub struct ModuleMenuRef<'a> {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: &'a [u8],
    pub tucan_last_checked: &'a NaiveDateTime,
    pub name: &'a str,
    pub done: bool,
    #[serde(
        serialize_with = "as_option_base64",
        deserialize_with = "from_option_base64"
    )]
    pub parent: Option<&'a [u8]>,
}

#[derive(Serialize, Debug)]
pub struct ModuleMenuEntryModule {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct ModuleMenuEntryModuleRef<'a> {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: &'a [u8],
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: &'a [u8],
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(
        Typescriptable,
    )
)]
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
        Typescriptable,
    )
)]
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
pub struct ModuleCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub course: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UndoneUser {
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
#[cfg_attr(
    feature = "server",
    derive(Typescriptable)
)]
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
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct UserCourse {
    pub user_id: i32,
    pub course_id: Vec<u8>,
}
