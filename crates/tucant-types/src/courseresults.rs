use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModuleResultsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub results: Vec<ModuleResult>,
    pub average_grade: String,
    pub sum_credits: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModuleResult {
    pub nr: String,
    pub grade: String,
    pub credits: String,
    pub status: Option<String>,
    pub pruefungen_url: String,
    pub average_url: Option<String>,
}
