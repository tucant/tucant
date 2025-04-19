use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyExamsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub exams: Vec<Exam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Exam {
    pub id: String,
    pub name: String,
    pub coursedetails_url: CourseDetailsRequest,
    pub tuple_of_courses: Option<String>,
    pub examdetail_url: String,
    pub pruefungsart: String,
    pub date: String,
    pub courseprep_url: Option<String>,
}
