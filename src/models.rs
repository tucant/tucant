use diesel::{prelude::*, sql_types::Timestamptz};

use crate::schema::modules;

// order needs to be equal to the table definition
#[derive(Queryable)]
pub struct Module {
    pub tucan_id: String,
    pub tucan_last_checked: Timestamptz,
    pub title: String,
    pub module_id: String,
    pub credits: i32,
    pub content: String,
}
