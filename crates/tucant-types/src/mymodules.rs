use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::moduledetails::ModuleDetailsRequest;

pub struct MyModulesResponse {
    pub semester: Vec<Semesterauswahl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Semesterauswahl {
    pub name: String,
    pub value: String,
    pub selected: bool,
}

pub struct Module {
    pub nr: String,
    pub title: String,
    pub lecturer: String,
    pub credits: String,
    pub url: ModuleDetailsRequest,
}
