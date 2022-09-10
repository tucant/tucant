use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::modules;

// order needs to be equal to the table definition
#[derive(Queryable, Insertable)]
#[diesel(table_name = modules)]
pub struct Module {
    pub tucan_id: String,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: i32,
    pub content: String,
}
