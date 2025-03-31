use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Semesterauswahl, coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModuleResultsResponse {
    pub semester: Vec<Semesterauswahl>,
    pub results: Vec<ModuleResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModuleResult {}
