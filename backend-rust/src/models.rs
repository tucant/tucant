// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tucant_derive::ts;

use crate::schema::{
    courses_unfinished, module_courses, module_menu_module, module_menu_unfinished,
    modules_unfinished,
};

pub fn as_base64<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(buffer.as_ref()))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

pub fn as_option_base64<T, S>(buffer: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    if let Some(ref buffer) = *buffer {
        serializer.serialize_str(&base64::encode(buffer.as_ref()))
    } else {
        serializer.serialize_none()
    }
}

pub fn from_option_base64<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        base64::decode(&s)
            .map(Option::Some)
            .map_err(|err| Error::custom(err.to_string()))
    } else {
        Ok(None)
    }
}

// order needs to be equal to the table definition
#[ts]
#[derive(
    Identifiable,
    Queryable,
    Insertable,
    AsChangeset,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = modules_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct Module {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[ts]
#[derive(
    Identifiable,
    Queryable,
    Insertable,
    Serialize,
    Debug,
    Eq,
    PartialEq,
    Deserialize,
    Clone,
    AsChangeset,
    QueryableByName,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[diesel(treat_none_as_null = false)]
pub struct ModuleMenu {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub child_type: i16,
    #[serde(default)]
    #[serde(
        serialize_with = "as_option_base64",
        deserialize_with = "from_option_base64"
    )]
    pub parent: Option<Vec<u8>>,
}

#[derive(AsChangeset, Debug, Insertable)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct ModuleMenuChangeset {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub child_type: i16,
    pub parent: Option<Option<Vec<u8>>>,
}

#[derive(Identifiable, Queryable, AsChangeset, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct ModuleMenuRef<'a> {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: &'a [u8],
    pub tucan_last_checked: &'a NaiveDateTime,
    pub name: &'a str,
    pub child_type: i16,
    #[serde(
        serialize_with = "as_option_base64",
        deserialize_with = "from_option_base64"
    )]
    pub parent: Option<&'a [u8]>,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[diesel(belongs_to(ModuleMenu))]
#[diesel(belongs_to(Module))]
pub struct ModuleMenuEntryModule {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: Vec<u8>,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[diesel(belongs_to(ModuleMenu))]
#[diesel(belongs_to(Module))]
pub struct ModuleMenuEntryModuleRef<'a> {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: &'a [u8],
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: &'a [u8],
}

#[derive(
    Identifiable,
    Queryable,
    Insertable,
    AsChangeset,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = courses_unfinished)]
#[diesel(treat_none_as_null = true)]
#[ts]
pub struct Course {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[type(String)]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
    pub sws: i16,
    pub content: String,
    pub done: bool,
}

pub struct Testse {
    pub tet: bool,
    #[ts]
    pub dfs: bool
}

#[derive(
    Associations,
    Identifiable,
    Queryable,
    Insertable,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(module, course))]
#[diesel(table_name = module_courses)]
#[diesel(treat_none_as_null = true)]
#[diesel(belongs_to(Module, foreign_key = module))]
#[diesel(belongs_to(Course, foreign_key = course))]
pub struct ModuleCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub course: Vec<u8>,
}
