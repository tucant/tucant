use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Semesterauswahl;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExamResultsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub results: Vec<ExamResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExamResult {
    pub name: String,
    pub exam_type: String,
    pub date: String,
    pub grade: String,
    pub grade_text: String,
    pub average_url: String,
}
