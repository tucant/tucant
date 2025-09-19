use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{LoggedInHead, coursedetails::CourseDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct MlsStart {
    pub logged_in_head: LoggedInHead,
    pub stundenplan: Vec<StundenplanEintrag>,
    pub messages: Vec<Nachricht>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct StundenplanEintrag {
    pub is_exam: bool,
    pub course_name: String,
    pub coursedetails_url: CourseDetailsRequest,
    pub courseprep_url: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Nachricht {
    pub url: String,
    pub date: String,
    pub hour: String,
    pub source: String,
    pub message: String,
    pub delete_url: String,
}
