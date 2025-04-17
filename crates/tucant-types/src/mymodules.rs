use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyModulesResponse {
    pub semester: Vec<Semesterauswahl>,
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Module {
    pub nr: String,
    pub title: String,
    pub lecturer: String,
    pub credits: Option<String>,
    pub url: ModuleDetailsRequest,
}
