use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::moduledetails::ModuleDetailsRequest;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct AnmeldungRequest {
    pub arguments: String,
}

impl Default for AnmeldungRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl AnmeldungRequest {
    #[must_use]
    pub fn new() -> Self {
        Self {
            arguments: ",-N000311,-A".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnmeldungResponse {
    pub path: Vec<(String, AnmeldungRequest)>,
    pub submenus: Vec<(String, AnmeldungRequest)>,
    pub entries: Vec<AnmeldungEntry>,
    pub additional_information: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnmeldungEntry {
    pub module: Option<AnmeldungModule>,
    pub courses: Vec<(Option<AnmeldungExam>, AnmeldungCourse)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum RegistrationState {
    Unknown,
    Registered { unregister_link: String },
    NotRegistered { register_link: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnmeldungModule {
    pub url: ModuleDetailsRequest,
    pub id: String,
    pub name: String,
    pub lecturer: Option<String>,
    pub date: String,
    pub limit_and_size: String,
    pub registration_button_link: RegistrationState,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnmeldungExam {
    pub name: String,
    pub typ: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnmeldungCourse {
    pub url: String,
    pub id: String,
    pub name: String,
    pub lecturers: Option<String>,
    pub begin_and_end: Option<String>,
    pub registration_until: String,
    pub limit_and_size: String,
    pub registration_button_link: RegistrationState,
}
