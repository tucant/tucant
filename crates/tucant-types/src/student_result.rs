use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Grade;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CourseOfStudySelection {
    pub name: String,
    pub value: u64,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultEntry {
    pub id: String,
    pub name: String,
    pub resultdetails_url: Option<String>,
    pub cp: Option<u64>,
    pub used_cp: Option<u64>,
    pub grade: Option<Grade>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultLevel {
    pub name: String,
    pub entries: Vec<StudentResultEntry>,
    pub sum_cp: Option<u64>,
    pub sum_used_cp: Option<u64>,
    pub state: Option<String>,
    pub rules: StudentResultRules,
    pub children: Vec<StudentResultLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultResponse {
    pub course_of_study: Vec<CourseOfStudySelection>,
    pub level0: StudentResultLevel,
    pub total_gpa: String,
    pub main_gpa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultRules {
    pub min_cp: u64,
    pub max_cp: Option<u64>,
    pub min_modules: u64,
    pub max_modules: Option<u64>,
}
