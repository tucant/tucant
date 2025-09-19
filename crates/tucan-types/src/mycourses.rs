use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyCoursesResponse {
    pub semester: Vec<Semesterauswahl>,
    pub sections: Vec<(String, Vec<Course>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Course {
    pub nr: String,
    pub title: String,
    pub date_range: String,
    pub location: String,
    pub url: CourseDetailsRequest,
    pub credits: Option<String>,
}
