use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Semesterauswahl;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyExamsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub exams: Vec<Exam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Exam {
    pub id: String,
    pub name: (String, String, Option<String>),
    pub examdetail_url: String,
    pub pruefungsart: String,
    pub date_and_courseprep: (String, Option<String>),
}
