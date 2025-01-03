use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseDetailsRequest {
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseDetailsResponse {}
