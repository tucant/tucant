use std::collections::VecDeque;

// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
use chrono::NaiveDateTime;
#[cfg(feature = "server")]
use diesel::prelude::*;
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
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable,)
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
#[serde(tag = "type", content = "value")]
pub enum RegistrationEnum {
    Submenu(Vec<ModuleMenu>),
    Modules(Vec<Module>),
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ModuleMenuResponse {
    pub module_menu: ModuleMenu,
    pub entries: RegistrationEnum,
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
    pub child_type: i16,
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(
        serialize_with = "as_option_base64",
        deserialize_with = "from_option_base64"
    )]
    pub parent: Option<Vec<u8>>,
}

#[cfg_attr(feature = "server", derive(AsChangeset, Debug, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct ModuleMenuChangeset {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub child_type: i16,
    pub parent: Option<Option<Vec<u8>>>,
}

#[derive(Serialize, Debug)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, AsChangeset, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
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

#[derive(Serialize, Debug)]
#[cfg_attr(
    feature = "server",
    derive(Associations, Identifiable, Queryable, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(module_menu_id, module_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_module))]
#[cfg_attr(feature = "server", diesel(belongs_to(ModuleMenu)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Module)))]
pub struct ModuleMenuEntryModuleRef<'a> {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: &'a [u8],
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: &'a [u8],
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = courses_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
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
    derive(Associations, Identifiable, Queryable, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(module, course)))]
#[cfg_attr(feature = "server", diesel(table_name = module_courses))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "server", diesel(belongs_to(Module, foreign_key = module)))]
#[cfg_attr(feature = "server", diesel(belongs_to(Course, foreign_key = course)))]
pub struct ModuleCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub course: Vec<u8>,
}
