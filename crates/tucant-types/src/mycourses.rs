use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyCoursesResponse {
    pub semester: Vec<Semesterauswahl>,
    pub courses: Vec<Course>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Course {
    pub nr: String,
    pub title: String,
    pub date_range: String,
    pub location: String,
    pub url: CourseDetailsRequest,
}
