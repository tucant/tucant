use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{ModuleGrade, Semesterauswahl, gradeoverview::GradeOverviewRequest};

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
    pub name: String,
    pub grade: ModuleGrade,
    pub credits: String,
    pub pruefungen_url: Option<String>,
    pub average_url: Option<GradeOverviewRequest>,
}
