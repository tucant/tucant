use std::{convert::Infallible, fmt::Display, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ModuleDetailsRequest {
    arguments: String,
}

impl FromStr for ModuleDetailsRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for ModuleDetailsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arguments)
    }
}

impl ModuleDetailsRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let module_details_regex = Regex::new(r"^-N(?P<n1>\d+)(,-A[a-zA-Z0-9_~-]+)?$").unwrap();
        let c = &module_details_regex.captures(input).expect("invalid module details url");
        Self { arguments: format!("-N{}", &c["n1"],) }
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.arguments.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ModuleDetailsResponse {
    pub module_id: String,
    pub registered: bool,
    pub dozenten: String,
    pub display_in_timetable: String,
    pub duration: String,
    pub count_elective_courses: String,
    pub credits: String,
    pub description: Vec<String>,
}
