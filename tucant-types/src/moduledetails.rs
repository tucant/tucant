use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ModuleDetailsRequest {
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDetailsResponse {
    pub module_id: String,
    pub registered: bool,
    pub dozenten: String,
    pub display_in_timetable: String,
    pub length: String,
    pub count_elective_courses: String,
    pub credits: String,
    pub description: Vec<String>,
}
