use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultEntry {
    pub id: String,
    pub name: String,
    pub resultdetails_url: Option<String>,
    pub cp: Option<String>,
    pub used_cp: Option<String>,
    pub grade: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultLevel {
    pub name: String,
    pub entries: Vec<StudentResultEntry>,
    pub sum_cp: Option<String>,
    pub sum_used_cp: Option<String>,
    pub state: String,
    pub rules: String,
    pub children: Vec<StudentResultLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultResponse {
    pub level0: StudentResultLevel,
    pub total_gpa: String,
    pub main_gpa: String,
}
