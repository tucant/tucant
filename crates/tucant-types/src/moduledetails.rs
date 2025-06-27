use std::{convert::Infallible, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::InstructorImage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ToSchema)]
pub struct ModuleDetailsRequest(String);

impl FromStr for ModuleDetailsRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for ModuleDetailsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ModuleDetailsRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        static MODULE_DETAILS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-N(?P<n1>\d+)(,-A[a-zA-Z0-9_~-]+)?$").unwrap());
        let c = &MODULE_DETAILS_REGEX.captures(input).expect(input);
        Self(format!("-N{}", &c["n1"],))
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct ModuleDetailsResponse {
    pub module_id: String,
    pub registered: bool,
    pub display_in_timetable: Option<String>,
    pub duration: String,
    pub count_elective_courses: String,
    pub credits: Option<u64>,
    pub description: Vec<String>,
    pub abweichende_credits: bool,
    pub start_semester: String,
    pub anmeldefristen: Option<Anmeldefristen>,
    pub kurskategorien: Vec<KursKategorie>,
    pub modulverantwortliche: Vec<(String, Option<InstructorImage>)>,
    pub leistungen: Vec<Leistung>,
    pub pruefungen: Vec<Pruefung>,
    pub warteliste_percentage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Anmeldefristen {
    pub anmeldeart: String,
    pub registration_range: String,
    pub unregistration_range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct KursKategorie {
    pub course_no: String,
    pub name: String,
    pub mandatory: bool,
    pub semester: Option<u64>,
    pub credits: f64,
    pub kurse: Vec<Kurs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Kurs {
    pub name: String,
    pub course_id: String,
    pub gefaehrungspotential_schwangere: bool,
    pub semester: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Leistung {
    pub name: String,
    pub compulsory: bool,
    pub weight: String,
    pub weight_more: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Pruefung {
    pub name: String,
    pub compulsory: bool,
    pub termine: Vec<Pruefungstermin>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Pruefungstermin {
    pub subname: String,
    pub date: String,
    pub examiner: String,
}
