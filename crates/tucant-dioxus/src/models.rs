use crate::schema::*;
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Text,
};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Copy, Clone, Hash)]
#[diesel(sql_type = Text)]
pub enum Semester {
    Sommersemester,
    Wintersemester,
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Semester {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> serialize::Result {
        out.set_value(match self {
            Semester::Sommersemester => "s",
            Semester::Wintersemester => "w",
        });
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Semester
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "s" => Ok(Semester::Sommersemester),
            "w" => Ok(Semester::Wintersemester),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[derive(Queryable, Selectable, Clone, PartialEq, Debug)]
#[diesel(table_name = anmeldungen_plan)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Anmeldung {
    pub url: String,
    pub name: String,
    pub parent: Option<String>,
    pub min_cp: i32,
    pub max_cp: Option<i32>,
    pub min_modules: i32,
    pub max_modules: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = anmeldungen_plan)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewAnmeldung<'a> {
    pub url: &'a str,
    pub name: &'a str,
    pub parent: Option<&'a str>,
    pub min_cp: i32,
    pub max_cp: Option<i32>,
    pub min_modules: i32,
    pub max_modules: Option<i32>,
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Copy, Clone)]
#[diesel(sql_type = Text)]
pub enum State {
    NotPlanned,
    Planned,
    Done,
}

impl ToSql<Text, diesel::sqlite::Sqlite> for State {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::sqlite::Sqlite>) -> serialize::Result {
        out.set_value(match self {
            Self::NotPlanned => "not_planned",
            Self::Planned => "planned",
            Self::Done => "done",
        });
        Ok(IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for State
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "not_planned" => Ok(Self::NotPlanned),
            "planned" => Ok(Self::Planned),
            "done" => Ok(Self::Done),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[derive(Queryable, Selectable, Clone, PartialEq, Debug, AsChangeset, Identifiable)]
#[diesel(table_name = anmeldungen_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(available_semester, anmeldung, id))]
#[diesel(treat_none_as_default_value = false)]
#[diesel(treat_none_as_null = true)]
pub struct AnmeldungEntry {
    pub available_semester: Semester,
    pub anmeldung: String,
    pub module_url: String,
    pub id: String,
    pub name: String,
    pub credits: i32,
    pub state: State,
    pub semester: Option<Semester>,
    pub year: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = anmeldungen_entries)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewAnmeldungEntry<'a> {
    pub semester: Semester,
    pub anmeldung: &'a str,
    pub module_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    pub credits: i32,
    pub state: State,
}
