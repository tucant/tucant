use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{module_menu, module_menu_module, modules};

// order needs to be equal to the table definition
#[derive(Identifiable, Queryable, Insertable, Serialize)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = modules)]
pub struct Module {
    pub tucan_id: [u64; 4],
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu)]
#[belongs_to(ModuleMenu, foreign_key = "parent")]
pub struct ModuleMenu {
    pub tucan_id: [u64; 4],
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub normalized_name: String,
    pub parent: Option<[u64; 4]>,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[belongs_to(ModuleMenu)]
#[belongs_to(Module)]
pub struct ModuleMenuEntryModule {
    pub module_menu_id: [u64; 4],
    pub module_id: [u64; 4],
}
