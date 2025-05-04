use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CourseOfStudySelection {
    pub name: String,
    pub value: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultEntry {
    pub id: String,
    pub name: String,
    pub resultdetails_url: Option<String>,
    pub cp: Option<String>,
    pub used_cp: Option<String>,
    pub grade: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultLevel {
    pub name: String,
    pub entries: Vec<StudentResultEntry>,
    pub sum_cp: Option<String>,
    pub sum_used_cp: Option<String>,
    pub state: Option<String>,
    /// can be 0-2 rules, first one is module count, second one is cp
    pub rules: Vec<String>,
    pub children: Vec<StudentResultLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultResponse {
    pub course_of_study: Vec<CourseOfStudySelection>,
    pub level0: StudentResultLevel,
    pub total_gpa: String,
    pub main_gpa: String,
}
