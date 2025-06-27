use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{gradeoverview::GradeOverviewRequest, Semesterauswahl};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExamResultsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub results: Vec<ExamResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExamResult {
    pub id: String,
    pub name: String,
    pub exam_type: String,
    pub date: Option<String>,
    pub grade: String,
    pub grade_text: Option<String>,
    pub average_url: Option<GradeOverviewRequest>,
}
