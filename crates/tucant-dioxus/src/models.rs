use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = anmeldungen)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Anmeldung {
    pub url: String,
    pub name: String,
    pub parent: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = anmeldungen)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewAnmeldung<'a> {
    pub url: &'a str,
    pub name: &'a str,
    pub parent: Option<&'a str>,
}
