use std::{convert::Infallible, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::coursedetails::CourseDetailsRequest;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct Vorlesungsverzeichnis {
    pub title: String,
    pub entries: Vec<(String, ActionRequest)>,
    pub path: Vec<(String, ActionRequest)>,
    pub description: Vec<String>,
    pub veranstaltungen_or_module: Vec<Veranstaltung>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct Veranstaltung {
    pub title: String,
    pub coursedetails_url: CourseDetailsRequest,
    pub lecturer_name: Option<String>,
    pub date_range: Option<String>,
    pub course_type: String,
    pub gefaehrdung_schwangere: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ActionRequest {
    arguments: String,
}

impl FromStr for ActionRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for ActionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arguments)
    }
}

impl ActionRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        static ACTION_REQUEST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-A[a-zA-Z0-9_~-]+$").unwrap());
        assert!(&ACTION_REQUEST.is_match(input), "{}", input);
        Self { arguments: input.to_owned() }
    }

    #[must_use]
    pub fn inner(&self) -> &str {
        self.arguments.as_str()
    }
}
