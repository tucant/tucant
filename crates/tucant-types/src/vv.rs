use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::coursedetails::CourseDetailsRequest;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct Vorlesungsverzeichnis {
    pub entries: Vec<String>,
    pub path: Vec<(String, Option<String>)>,
    pub description: Vec<String>,
    pub veranstaltungen_or_module: Vec<Veranstaltung>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct Veranstaltung {
    pub title: String,
    pub coursedetails_url: CourseDetailsRequest,
    pub lecturer_name: String,
    pub date_range: Option<String>,
    pub course_type: String,
}
