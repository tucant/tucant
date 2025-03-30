use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyExamsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub exams: Vec<Exam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Exam {
    pub id: String,
    pub name: String,
    pub coursedetails_url: String,
    pub tuple_of_courses: String,
    pub examdetail_url: String,
    pub pruefungsart: String,
    pub courseprep_url: String,
    pub date: String,
}
