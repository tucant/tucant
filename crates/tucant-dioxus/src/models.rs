use crate::schema::*;
use diesel::{
    backend::Backend, deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, prelude::*, serialize::{self, IsNull, Output, ToSql}, sql_types::Text, sqlite::{Sqlite, SqliteValue}
};

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Copy, Clone)]
#[diesel(sql_type = Text)]
pub enum Semester {
    Sommersemester,
    Wintersemester,
}

impl ToSql<Text, diesel::sqlite::Sqlite> for Semester
{
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

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = anmeldungen)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Anmeldung {
    pub semester: Semester,
    pub url: String,
    pub name: String,
    pub parent: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = anmeldungen)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewAnmeldung<'a> {
    pub semester: Semester,
    pub url: &'a str,
    pub name: &'a str,
    pub parent: Option<&'a str>,
}
