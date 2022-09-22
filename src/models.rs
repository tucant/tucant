use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{module_menu_module, module_menu_unfinished, modules_unfinished};

// order needs to be equal to the table definition
#[derive(Identifiable, Queryable, Insertable, AsChangeset, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = modules_unfinished)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Module {
    pub tucan_id: i64,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[derive(Associations, Identifiable, Queryable, AsChangeset, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[belongs_to(ModuleMenu, foreign_key = "parent")]
#[changeset_options(treat_none_as_null = "true")]
pub struct ModuleMenu {
    pub tucan_id: Vec<i64>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub normalized_name: String,
    pub parent: Option<Vec<i64>>,
    pub child_type: i16,
}

#[derive(Associations, Identifiable, Queryable, AsChangeset, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[belongs_to(ModuleMenu, foreign_key = "parent")]
#[changeset_options(treat_none_as_null = "true")]
pub struct ModuleMenuRef<'a> {
    pub tucan_id: &'a Vec<i64>,
    pub tucan_last_checked: &'a NaiveDateTime,
    pub name: &'a str,
    pub normalized_name: &'a str,
    pub parent: Option<&'a Vec<i64>>,
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

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[belongs_to(ModuleMenu)]
#[belongs_to(Module)]
pub struct ModuleMenuEntryModuleRef<'a> {
    pub module_menu_id: &'a Vec<i64>,
    pub module_id: i64,
}
