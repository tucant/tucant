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
    /// only one of these two are Some
    pub coursedetails_url: Option<CourseDetailsRequest>,
    /// only one of these two are Some
    pub moduledetails_url: Option<ModuleDetailsRequest>,
    pub tuple_of_courses: Option<String>,
    pub examdetail_url: String,
    pub pruefungsart: String,
    pub date: String,
    pub courseprep_url: Option<String>,
    pub examunreg_url: Option<String>
}
