use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    courses_unfinished, module_courses, module_menu_module, module_menu_tree,
    module_menu_unfinished, modules_unfinished,
};

// order needs to be equal to the table definition
#[derive(
    Identifiable,
    Queryable,
    Insertable,
    AsChangeset,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = modules_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct Module {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub module_id: String,
    pub credits: Option<i32>,
    pub content: String,
    pub done: bool,
}

#[derive(
    Identifiable,
    Queryable,
    AsChangeset,
    Insertable,
    Serialize,
    Debug,
    Eq,
    PartialEq,
    Deserialize,
    Clone,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct ModuleMenu {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub name: String,
    pub normalized_name: String,
    pub child_type: i16,
}

#[derive(Identifiable, Queryable, AsChangeset, Insertable, Serialize, Debug)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = module_menu_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct ModuleMenuRef<'a> {
    pub tucan_id: &'a [u8],
    pub tucan_last_checked: &'a NaiveDateTime,
    pub name: &'a str,
    pub normalized_name: &'a str,
    pub child_type: i16,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[diesel(belongs_to(ModuleMenu))]
#[diesel(belongs_to(Module))]
pub struct ModuleMenuEntryModule {
    pub module_menu_id: Vec<u8>,
    pub module_id: Vec<u8>,
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(module_menu_id, module_id))]
#[diesel(table_name = module_menu_module)]
#[diesel(belongs_to(ModuleMenu))]
#[diesel(belongs_to(Module))]
pub struct ModuleMenuEntryModuleRef<'a> {
    pub module_menu_id: &'a [u8],
    pub module_id: &'a [u8],
}

#[derive(Associations, Identifiable, Queryable, Insertable, Serialize, Debug)]
#[diesel(primary_key(parent, child))]
#[diesel(table_name = module_menu_tree)]
#[diesel(belongs_to(ModuleMenu, foreign_key = child))]
pub struct ModuleMenuTreeEntry {
    pub parent: Vec<u8>,
    pub child: Vec<u8>,
}

#[derive(
    Identifiable,
    Queryable,
    Insertable,
    AsChangeset,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(tucan_id))]
#[diesel(table_name = courses_unfinished)]
#[diesel(treat_none_as_null = true)]
pub struct Course {
    pub tucan_id: Vec<u8>,
    pub tucan_last_checked: NaiveDateTime,
    pub title: String,
    pub course_id: String,
    pub sws: i16,
    pub content: String,
    pub done: bool,
}

#[derive(
    Associations,
    Identifiable,
    Queryable,
    Insertable,
    Serialize,
    Debug,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(primary_key(module, course))]
#[diesel(table_name = module_courses)]
#[diesel(treat_none_as_null = true)]
#[diesel(belongs_to(Module, foreign_key = module))]
#[diesel(belongs_to(Course, foreign_key = course))]
pub struct ModuleCourse {
    pub module: Vec<u8>,
    pub course: Vec<u8>,
}
