use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{module_menu_module, module_menu_unfinished, modules_unfinished};

// order needs to be equal to the table definition
#[derive(Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = modules_unfinished)]
pub struct Module {
    pub tucan_id: i64,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[belongs_to(ModuleMenu, foreign_key = "parent")]
pub struct ModuleMenu {
    pub tucan_id: Vec<i64>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub normalized_name: String,
    pub parent: Option<Vec<i64>>,
    pub child_type: i16,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[belongs_to(ModuleMenu)]
#[belongs_to(Module)]
pub struct ModuleMenuEntryModule {
    pub module_menu_id: Vec<i64>,
    pub module_id: i64,
}
