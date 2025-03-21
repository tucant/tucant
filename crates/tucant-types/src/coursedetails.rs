use std::{convert::Infallible, fmt::Display, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseDetailsRequest {
    arguments: String,
}

impl FromStr for CourseDetailsRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for CourseDetailsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arguments)
    }
}

impl CourseDetailsRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        let course_details_regex = Regex::new(r"^-N(?P<n1>\d+),-N(?P<n2>\d+),-N(?P<n3>\d+),-N(?P<n4>\d+),-N(?P<n5>\d+)(,-N(?P<n6>\d+)(,-A[a-zA-Z0-9_~-]+)?)?$").unwrap();
        let c = &course_details_regex.captures(input).expect("invalid course details url");
        Self {
            arguments: format!("-N{},-N{},-N{},-N{},-N{}{}", &c["n1"], &c["n2"], &c["n3"], &c["n4"], &c["n5"], c.name("n6").map(|e| format!(",-N{}", e.as_str())).unwrap_or_default()),
        }
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.arguments.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseDetailsResponse {
    pub name: String,
    pub material_and_messages_url: Option<(String, String)>,
    pub dozent: Option<String>,
    pub r#type: String,
    pub type_number: u64,
    pub fachbereich: String,
    pub anzeige_im_stundenplan: String,
    pub shortname: String,
    pub courselevel: u64,
    pub sws: Option<u64>,
    pub credits: Option<u64>,
    pub language: String,
    pub language_id: u64,
    pub teilnehmer_range: String,
    pub teilnehmer_max: String,
    pub description: Vec<String>,
    pub uebungsgruppen: Vec<CourseUebungsGruppe>,
    pub course_anmeldefristen: Vec<CourseAnmeldefrist>,
    pub enhalten_in_modulen: Vec<String>,
    pub termine: Vec<Termin>,
    pub short_termine: Vec<(String, String)>,
    pub instructors: Vec<(String, Option<InstructorImage>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct InstructorImage {
    pub href: String,
    pub imgsrc: String,
    pub alt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseUebungsGruppe {
    pub name: String,
    pub uebungsleiter: String,
    pub date_range: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseAnmeldefrist {
    pub zulassungstyp: String,
    pub block_type: String,
    pub start: String,
    pub ende_anmeldung: String,
    pub ende_abmeldung: String,
    pub ende_hoerer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Room {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Termin {
    pub id: String,
    pub date: String,
    pub time_start: String,
    pub time_end: String,
    pub rooms: Vec<Room>,
    pub instructors: String,
}
