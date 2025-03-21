use std::{convert::Infallible, fmt::Display, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{coursedetails::CourseDetailsRequest, moduledetails::ModuleDetailsRequest};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct AnmeldungRequest {
    arguments: String,
}

impl FromStr for AnmeldungRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for AnmeldungRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arguments)
    }
}

impl AnmeldungRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        if input.is_empty() {
            Self { arguments: "-A".to_owned() }
        } else {
            let registration_details_regex = Regex::new(r"^-N(?P<n1>\d+),-N(?P<n2>\d+),-N(?P<n3>\d+),-N(?P<n4>\d+)$").unwrap();
            let c = &registration_details_regex.captures(input).expect(input);
            Self { arguments: format!("-N{},-N{},-N{},-N{}", &c["n1"], &c["n2"], &c["n3"], &c["n4"],) }
        }
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.arguments.as_str()
    }
}

impl Default for AnmeldungRequest {
    fn default() -> Self {
        Self::parse("")
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
    pub url: CourseDetailsRequest,
    pub id: String,
    pub name: String,
    pub lecturers: Option<String>,
    pub begin_and_end: Option<String>,
    pub registration_until: String,
    pub limit_and_size: String,
    pub registration_button_link: RegistrationState,
}
