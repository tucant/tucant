use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MyDocumentsResponse {
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Document {
    pub name: String,
    pub date: String,
    pub time: String,
    pub url: String,
}
