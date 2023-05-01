#![allow(clippy::wildcard_imports)] // inside diesel macro

use base64::prelude::*;

use diesel::query_builder::UndecoratedInsertRecord;

use diesel::sql_types::Binary;
use diesel::sql_types::Int4;
use diesel::sql_types::SmallInt;

use std::collections::VecDeque;

use std::hash::Hash;
use std::io::ErrorKind;
// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
#[cfg(feature = "diesel")]
use diesel::prelude::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName};
#[cfg(feature = "diesel")]
use diesel::sql_types::Bool;

#[cfg(feature = "diesel")]
use diesel::sql_types::Nullable;
#[cfg(feature = "diesel")]
use diesel::sql_types::Text;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "diesel")]
use tucant_derive::Typescriptable;

#[cfg(feature = "diesel")]
use crate::schema::{
    course_events, course_exams, course_groups_events, course_groups_unfinished,
    courses_unfinished, exams_unfinished, module_courses, module_exam_types, module_exams,
    module_menu_module, module_menu_unfinished, modules_unfinished, sessions, user_course_groups,
    user_courses, user_exams, user_modules, users_unfinished, vv_menu_courses, vv_menu_unfinished,
};

pub fn as_base64<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&BASE64_URL_SAFE_NO_PAD.encode(buffer.as_ref()))
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        BASE64_URL_SAFE_NO_PAD
            .decode(string)
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
            BASE64_URL_SAFE_NO_PAD
                .decode(string)
                .map(Option::Some)
                .map_err(|err| Error::custom(err.to_string()))
        },
    )
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct PartialModule {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct CompleteModule {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Typescriptable)]
#[serde(tag = "type", content = "value")] // TODO FIXME make Typescriptable detect/enforce this
pub enum MaybeCompleteModule {
    Partial(PartialModule),
    Complete(CompleteModule),
}

impl From<&CompleteModule> for InternalModule {
    fn from(value: &CompleteModule) -> Self {
        Self {
            tucan_id: value.tucan_id.clone(),
            tucan_last_checked: value.tucan_last_checked,
            title: value.title.clone(),
            credits: value.credits,
            module_id: value.module_id.clone(),
            content: value.content.clone(),
            done: true,
        }
    }
}

impl From<&PartialModule> for InternalModule {
    fn from(value: &PartialModule) -> Self {
        Self {
            tucan_id: value.tucan_id.clone(),
            tucan_last_checked: value.tucan_last_checked,
            title: value.title.clone(),
            module_id: value.module_id.clone(),
            credits: 0,
            content: String::new(),
            done: false,
        }
    }
}

impl From<&MaybeCompleteModule> for InternalModule {
    fn from(value: &MaybeCompleteModule) -> Self {
        match value {
            MaybeCompleteModule::Partial(value) => value.into(),
            MaybeCompleteModule::Complete(value) => value.into(),
        }
    }
}

impl TryFrom<InternalModule> for CompleteModule {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: InternalModule) -> Result<Self, Self::Error> {
        match TryInto::<MaybeCompleteModule>::try_into(value)? {
            MaybeCompleteModule::Complete(value) => Ok(value),
            MaybeCompleteModule::Partial(_) => Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "expected complete module, got partial module",
            ))),
        }
    }
}

impl TryFrom<InternalModule> for MaybeCompleteModule {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: InternalModule) -> Result<Self, Self::Error> {
        match value {
            InternalModule {
                tucan_id,
                tucan_last_checked,
                title,
                content,
                module_id,
                credits,
                done: true,
            } => Ok(Self::Complete(CompleteModule {
                tucan_id,
                tucan_last_checked,
                title,
                module_id,
                credits,
                content,
            })),
            InternalModule {
                tucan_id,
                tucan_last_checked,
                title,
                module_id,
                credits: 0,
                ref content,
                done: false,
            } if content.is_empty() => Ok(Self::Partial(PartialModule {
                tucan_id,
                tucan_last_checked,
                title,
                module_id,
            })),
            _ => Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "invalid enum in database",
            ))),
        }
    }
}

impl MaybeCompleteModule {
    #[must_use]
    pub const fn tucan_id(&self) -> &Vec<u8> {
        match self {
            Self::Partial(v) => &v.tucan_id,
            Self::Complete(v) => &v.tucan_id,
        }
    }

    #[must_use]
    pub const fn module_id(&self) -> &String {
        match self {
            Self::Partial(v) => &v.module_id,
            Self::Complete(v) => &v.module_id,
        }
    }
}

impl Insertable<modules_unfinished::table> for MaybeCompleteModule {
    type Values = <InternalModule as Insertable<modules_unfinished::table>>::Values;

    fn values(self) -> Self::Values {
        InternalModule::from(&self).values()
    }
}

impl Insertable<modules_unfinished::table> for &MaybeCompleteModule {
    type Values = <InternalModule as Insertable<modules_unfinished::table>>::Values;

    fn values(self) -> Self::Values {
        InternalModule::from(self).values()
    }
}

impl Insertable<modules_unfinished::table> for &CompleteModule {
    type Values = <InternalModule as Insertable<modules_unfinished::table>>::Values;

    fn values(self) -> Self::Values {
        InternalModule::from(self).values()
    }
}

impl UndecoratedInsertRecord<modules_unfinished::table> for MaybeCompleteModule {}

impl AsChangeset for &MaybeCompleteModule {
    type Target = <InternalModule as AsChangeset>::Target;

    type Changeset = <InternalModule as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        InternalModule::from(self).as_changeset()
    }
}

impl AsChangeset for &CompleteModule {
    type Target = <InternalModule as AsChangeset>::Target;

    type Changeset = <InternalModule as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        InternalModule::from(self).as_changeset()
    }
}

impl<DB: Backend> Queryable<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>
    for MaybeCompleteModule
where
    Vec<u8>: FromSql<Binary, DB>,
    NaiveDateTime: FromSql<Timestamptz, DB>,
    String: FromSql<Text, DB>,
    i32: FromSql<Int4, DB>,
    bool: FromSql<Bool, DB>,
{
    type Row =
        <InternalModule as Queryable<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>>::Row;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value: InternalModule =
            Queryable::<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>::build(row)?;
        value.try_into()
    }
}

impl<DB: Backend> Queryable<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>
    for CompleteModule
where
    Vec<u8>: FromSql<Binary, DB>,
    NaiveDateTime: FromSql<Timestamptz, DB>,
    String: FromSql<Text, DB>,
    i32: FromSql<Int4, DB>,
    bool: FromSql<Bool, DB>,
{
    type Row =
        <InternalModule as Queryable<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>>::Row;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value: InternalModule =
            Queryable::<(Binary, Timestamptz, Text, Text, Int4, Text, Bool), DB>::build(row)?;
        value.try_into()
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = modules_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct InternalModule {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: i32,
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

pub trait PathLike<TI: Eq + Hash> {
    fn leaf(&self) -> bool;
    fn tucan_id(&self) -> TI;
    fn parent(&self) -> Option<TI>;
}

impl PathLike<Vec<u8>> for ModuleMenuPathPart {
    fn leaf(&self) -> bool {
        self.leaf
    }

    fn tucan_id(&self) -> Vec<u8> {
        self.tucan_id.clone()
    }

    fn parent(&self) -> Option<Vec<u8>> {
        self.parent.clone()
    }
}

#[cfg_attr(feature = "server", derive(Typescriptable))]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub submenus: Vec<ModuleMenu>,
    pub modules_and_courses: Vec<(MaybeCompleteModule, Vec<MaybeCompleteCourse>)>,
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
    pub module: CompleteModule,
    pub courses: Vec<MaybeCompleteCourse>,
    pub exam_types: Vec<ModuleExamType>,
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
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable,))]
#[cfg_attr(feature = "server", diesel(primary_key(module_menu_id, module_id)))]
#[cfg_attr(feature = "server", diesel(table_name = module_menu_module))]
pub struct ModuleMenuEntryModule {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_menu_id: Vec<u8>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct PartialCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Typescriptable,))]
pub struct CompleteCourse {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
    pub sws: i16,
    pub content: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Typescriptable)]
#[serde(tag = "type", content = "value")] // TODO FIXME make Typescriptable detect/enforce this
pub enum MaybeCompleteCourse {
    Partial(PartialCourse),
    Complete(CompleteCourse),
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = courses_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct InternalCourse {
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

impl From<&MaybeCompleteCourse> for InternalCourse {
    fn from(value: &MaybeCompleteCourse) -> Self {
        match value {
            MaybeCompleteCourse::Partial(value) => Self {
                tucan_id: value.tucan_id.clone(),
                tucan_last_checked: value.tucan_last_checked,
                title: value.title.clone(),
                course_id: value.course_id.clone(),
                sws: 0,
                content: String::new(),
                done: false,
            },
            MaybeCompleteCourse::Complete(value) => Self {
                tucan_id: value.tucan_id.clone(),
                tucan_last_checked: value.tucan_last_checked,
                title: value.title.clone(),
                course_id: value.course_id.clone(),
                sws: value.sws,
                content: value.content.clone(),
                done: true,
            },
        }
    }
}

impl TryFrom<InternalCourse> for CompleteCourse {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: InternalCourse) -> Result<Self, Self::Error> {
        match TryInto::<MaybeCompleteCourse>::try_into(value)? {
            MaybeCompleteCourse::Complete(value) => Ok(value),
            MaybeCompleteCourse::Partial(_) => Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "expected complete course, got partial course",
            ))),
        }
    }
}

impl TryFrom<InternalCourse> for MaybeCompleteCourse {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: InternalCourse) -> Result<Self, Self::Error> {
        match value {
            InternalCourse {
                tucan_id,
                tucan_last_checked,
                title,
                course_id,
                sws,
                content,
                done: true,
            } => Ok(Self::Complete(CompleteCourse {
                tucan_id,
                tucan_last_checked,
                title,
                course_id,
                sws,
                content,
            })),
            InternalCourse {
                tucan_id,
                tucan_last_checked,
                title,
                course_id,
                sws: 0,
                ref content,
                done: false,
            } if content.is_empty() => Ok(Self::Partial(PartialCourse {
                tucan_id,
                tucan_last_checked,
                title,
                course_id,
            })),
            _ => Err(Box::new(std::io::Error::new(
                ErrorKind::Other,
                "invalid enum in database",
            ))),
        }
    }
}

impl MaybeCompleteCourse {
    #[must_use]
    pub const fn tucan_id(&self) -> &Vec<u8> {
        match self {
            Self::Partial(v) => &v.tucan_id,
            Self::Complete(v) => &v.tucan_id,
        }
    }

    #[must_use]
    pub const fn title(&self) -> &String {
        match self {
            Self::Partial(v) => &v.title,
            Self::Complete(v) => &v.title,
        }
    }

    #[must_use]
    pub const fn course_id(&self) -> &String {
        match self {
            Self::Partial(v) => &v.course_id,
            Self::Complete(v) => &v.course_id,
        }
    }
}

impl Insertable<courses_unfinished::table> for MaybeCompleteCourse {
    type Values = <InternalCourse as Insertable<courses_unfinished::table>>::Values;

    fn values(self) -> Self::Values {
        InternalCourse::from(&self).values()
    }
}

impl Insertable<courses_unfinished::table> for &MaybeCompleteCourse {
    type Values = <InternalCourse as Insertable<courses_unfinished::table>>::Values;

    fn values(self) -> Self::Values {
        InternalCourse::from(self).values()
    }
}

impl UndecoratedInsertRecord<courses_unfinished::table> for MaybeCompleteCourse {}

impl AsChangeset for &MaybeCompleteCourse {
    type Target = <InternalCourse as AsChangeset>::Target;

    type Changeset = <InternalCourse as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        InternalCourse::from(self).as_changeset()
    }
}

impl<DB: Backend> Queryable<(Binary, Timestamptz, Text, Text, SmallInt, Text, Bool), DB>
    for MaybeCompleteCourse
where
    Vec<u8>: FromSql<Binary, DB>,
    NaiveDateTime: FromSql<Timestamptz, DB>,
    String: FromSql<Text, DB>,
    i16: FromSql<SmallInt, DB>,
    bool: FromSql<Bool, DB>,
{
    type Row = <InternalCourse as Queryable<
        (Binary, Timestamptz, Text, Text, SmallInt, Text, Bool),
        DB,
    >>::Row;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value: InternalCourse =
            Queryable::<(Binary, Timestamptz, Text, Text, SmallInt, Text, Bool), DB>::build(row)?;
        value.try_into()
    }
}

impl<DB: Backend> Queryable<(Binary, Timestamptz, Text, Text, SmallInt, Text, Bool), DB>
    for CompleteCourse
where
    Vec<u8>: FromSql<Binary, DB>,
    NaiveDateTime: FromSql<Timestamptz, DB>,
    String: FromSql<Text, DB>,
    i16: FromSql<SmallInt, DB>,
    bool: FromSql<Bool, DB>,
{
    type Row = <InternalCourse as Queryable<
        (Binary, Timestamptz, Text, Text, SmallInt, Text, Bool),
        DB,
    >>::Row;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        let value: InternalCourse =
            Queryable::<(Binary, Timestamptz, Text, Text, SmallInt, Text, Bool), DB>::build(row)?;
        value.try_into()
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Insertable, AsChangeset, Typescriptable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = course_groups_unfinished))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
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
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable,))]
#[cfg_attr(feature = "server", diesel(primary_key(module, course)))]
#[cfg_attr(feature = "server", diesel(table_name = module_courses))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
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

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, module_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_modules))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct UserModule {
    pub user_id: i32,
    pub module_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone, Typescriptable)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, course_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_courses))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
pub struct UserCourse {
    pub user_id: i32,
    //#[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    #[cfg_attr(feature = "server", ts_type(String))]
    pub course_id: Vec<u8>,
}

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone, Typescriptable)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(feature = "server", diesel(primary_key(user_id, course_group_id)))]
#[cfg_attr(feature = "server", diesel(table_name = user_course_groups))]
#[cfg_attr(feature = "server", diesel(treat_none_as_null = true))]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Debug)]
#[cfg_attr(
    feature = "server",
    derive(Identifiable, Queryable, Typescriptable, Insertable,)
)]
#[cfg_attr(feature = "server", diesel(primary_key(tucan_id)))]
#[cfg_attr(feature = "server", diesel(table_name = vv_menu_unfinished))]
pub struct VVMenuItem {
    pub tucan_id: String,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub done: bool,
    pub parent: Option<String>,
}

#[cfg_attr(feature = "server", derive(QueryableByName, Typescriptable))]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Clone, Deserialize)]
pub struct VVMenuPathPart {
    #[cfg_attr(feature = "server", diesel(sql_type = Nullable<Text>))]
    #[serde(skip)]
    pub parent: Option<String>,
    #[cfg_attr(feature = "server", diesel(sql_type = Text))]
    pub tucan_id: String,
    #[cfg_attr(feature = "server", diesel(sql_type = Text))]
    pub name: String,
    #[cfg_attr(feature = "server", diesel(sql_type = Bool))]
    #[serde(skip)]
    pub leaf: bool,
}

impl PathLike<String> for VVMenuPathPart {
    fn leaf(&self) -> bool {
        self.leaf
    }

    fn tucan_id(&self) -> String {
        self.tucan_id.clone()
    }

    fn parent(&self) -> Option<String> {
        self.parent.clone()
    }
}

#[derive(Serialize, Debug)]
#[cfg_attr(feature = "server", derive(Identifiable, Queryable, Insertable,))]
#[cfg_attr(feature = "server", diesel(primary_key(vv_menu_id, course_id)))]
#[cfg_attr(feature = "server", diesel(table_name = vv_menu_courses))]
#[cfg_attr(feature = "server", diesel(belongs_to(MaybeCompleteCourse)))]
#[cfg_attr(feature = "server", diesel(belongs_to(VVMenuItem, foreign_key = vv_menu_id)))]
pub struct VVMenuCourses {
    pub vv_menu_id: String,
    pub course_id: Vec<u8>,
}

#[derive(
    Insertable, Queryable, Typescriptable, Clone, PartialEq, Eq, Serialize, Deserialize, Debug,
)]
#[cfg_attr(feature = "server", diesel(table_name = module_exam_types))]
pub struct ModuleExamType {
    pub module_id: Vec<u8>,
    pub exam_type: String,
    pub required: bool,
    pub weight: i16,
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
    courses_unfinished::tucan_id,
    courses_unfinished::tucan_last_checked,
    courses_unfinished::title,
    courses_unfinished::course_id,
    courses_unfinished::sws,
    courses_unfinished::content,
    courses_unfinished::done,
) = (
    courses_unfinished::tucan_id,
    courses_unfinished::tucan_last_checked,
    courses_unfinished::title,
    courses_unfinished::course_id,
    courses_unfinished::sws,
    courses_unfinished::content,
    courses_unfinished::done,
);
