use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::moduledetails::ModuleDetailsRequest;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyModulesResponse {
    pub semester: Vec<Semesterauswahl>,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Semesterauswahl {
    pub name: String,
    pub value: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Module {
    pub nr: String,
    pub title: String,
    pub lecturer: String,
    pub credits: String,
    pub url: ModuleDetailsRequest,
}
